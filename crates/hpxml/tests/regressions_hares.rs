//! Minimal reproductions for xsd-parser / hpxml-rs deserializer issues found
//! while integrating hpxml-rs into HARES (see the audit in that repo).
//!
//! Requires the xsd-parser fork fix pinned in the root `Cargo.toml`
//! (`[patch.crates-io]`, rev `b2b672a`): unconsumed elements that are not part
//! of the content model are skipped instead of raising
//! `ErrorKind::UnexpectedEvent` (upstream issue Bergmann89/xsd-parser#293,
//! fix Bergmann89/xsd-parser#294).

use hpxml::HpxmlSerialize;

/// v4 envelope, v3-shaped `Site` content: `SiteID`, `Latitude`, `Longitude`
/// are direct children of `Site` (v3 layout). v4 has no such elements there,
/// so the deserializer must SKIP them — the released generator (1.5.2)
/// instead fails with `Unexpected event: Start(Latitude)`.
///
/// `BuildingDetails` is a required child of `Building` in both v3 and v4, so
/// it is present (with its own all-optional children) to keep the rest of the
/// document schema-valid.
const V4_ENVELOPE_V3_SITE: &str = r#"<HPXML xmlns='http://hpxmlonline.com/2023/09' schemaVersion='4.0'>
  <XMLTransactionHeaderInformation>
    <XMLType>HPXML</XMLType>
    <XMLGeneratedBy>test</XMLGeneratedBy>
    <CreatedDateAndTime>2026-01-01T00:00:00Z</CreatedDateAndTime>
    <Transaction>create</Transaction>
  </XMLTransactionHeaderInformation>
  <SoftwareInfo/>
  <Building>
    <BuildingID id='MyBuilding'/>
    <Site>
      <SiteID id='SiteID'/>
      <Latitude>40.01</Latitude>
      <Longitude>-105.27</Longitude>
    </Site>
    <ProjectStatus>
      <EventType>audit</EventType>
    </ProjectStatus>
    <BuildingDetails/>
  </Building>
</HPXML>"#;

#[test]
fn v4_parse_skips_v3_shaped_site_children() {
    let doc = hpxml::v4::parse(V4_ENVELOPE_V3_SITE.as_bytes())
        .expect("v4 parse must skip unknown Site children");
    // Round-trip must remain lossless for the known content.
    let xml = doc.to_xml().expect("serialize");
    let text = String::from_utf8(xml).expect("utf8");
    assert!(text.contains("BuildingID"), "known content preserved");
    assert!(text.contains("audit"), "known content preserved");
    // The v3-shaped Site children are not part of the v4 Site content model,
    // so the skip fix must have dropped them.
    assert!(
        !text.contains("Latitude") && !text.contains("Longitude"),
        "unknown Site children must be skipped, got: {text}"
    );
}

#[test]
fn v3_parse_accepts_v3_shaped_site() {
    // Same content under the v3 namespace/schemaVersion. Note: in v3,
    // `Site/SiteID` is schema-valid, but v3 keeps latitude/longitude under
    // `Site/GeoLocation/Latitude` — a direct `Site/Latitude` is NOT part of
    // the v3 content model, so with the skip fix it parses but is dropped.
    let xml = V4_ENVELOPE_V3_SITE
        .replace(
            "http://hpxmlonline.com/2023/09",
            "http://hpxmlonline.com/2019/10",
        )
        .replace("schemaVersion='4.0'", "schemaVersion='3.0'");
    let doc = hpxml::v3::parse(xml.as_bytes()).expect("v3 parse of v3-shaped site");
    let out = String::from_utf8(doc.to_xml().expect("serialize")).expect("utf8");
    assert!(
        out.contains("BuildingID"),
        "known content preserved, got: {out}"
    );
    assert!(
        !out.contains("Latitude") && !out.contains("Longitude"),
        "v3 has no direct Site/Latitude or Site/Longitude; they must be skipped, got: {out}"
    );
}

#[test]
fn v3_preserves_geolocation_latitude() {
    // Positive control for the fix: latitude under its schema-valid v3 path
    // (`Site/GeoLocation/Latitude`) is part of the content model and must
    // still be consumed and round-tripped, not skipped.
    let xml = r#"<HPXML xmlns='http://hpxmlonline.com/2019/10' schemaVersion='3.0'>
  <XMLTransactionHeaderInformation>
    <XMLType>HPXML</XMLType>
    <XMLGeneratedBy>test</XMLGeneratedBy>
    <CreatedDateAndTime>2026-01-01T00:00:00Z</CreatedDateAndTime>
    <Transaction>create</Transaction>
  </XMLTransactionHeaderInformation>
  <SoftwareInfo/>
  <Building>
    <BuildingID id='MyBuilding'/>
    <Site>
      <SiteID id='SiteID'/>
      <GeoLocation>
        <Latitude>40.01</Latitude>
        <Longitude>-105.27</Longitude>
      </GeoLocation>
    </Site>
    <ProjectStatus>
      <EventType>audit</EventType>
    </ProjectStatus>
    <BuildingDetails/>
  </Building>
</HPXML>"#;
    let doc = hpxml::v3::parse(xml.as_bytes()).expect("v3 parse of GeoLocation");
    let out = String::from_utf8(doc.to_xml().expect("serialize")).expect("utf8");
    assert!(
        out.contains("<Latitude>40.01</Latitude>")
            && out.contains("<Longitude>-105.27</Longitude>"),
        "v3 must preserve Site/GeoLocation latitude/longitude, got: {out}"
    );
}

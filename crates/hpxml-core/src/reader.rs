//! Depth-tracking XML reader wrapper that enforces the `max_depth` limit.

use hpxml_common::ParseConfig;
use quick_xml::events::Event;
use xsd_parser_types::quick_xml::{Error as XmlError, XmlReader, XmlReaderSync};

/// A wrapper around `quick_xml::NsReader` that enforces a maximum nesting depth.
///
/// This is the wrapper approach: zero overhead, single-pass parsing.
/// We wrap the inner NsReader and intercept Start/End events for depth tracking.
///
/// If the XML nesting depth exceeds the configured limit, [`XmlReaderSync::read_event`]
/// returns an error that wraps [`hpxml_common::ParseError::DepthLimitExceeded`].
pub struct DepthLimitedReader<'a> {
    inner: quick_xml::NsReader<&'a [u8]>,
    config: ParseConfig,
    current_depth: usize,
}

impl<'a> DepthLimitedReader<'a> {
    /// Creates a new `DepthLimitedReader` wrapping the provided bytes.
    pub fn new(bytes: &'a [u8], config: ParseConfig) -> Self {
        Self {
            inner: quick_xml::NsReader::from_reader(bytes),
            config,
            current_depth: 0,
        }
    }

    /// Creates a new `DepthLimitedReader` from a string (convenience method).
    pub fn from_str(s: &'a str, config: ParseConfig) -> Self {
        Self::new(s.as_bytes(), config)
    }

    /// Returns the current nesting depth.
    pub fn current_depth(&self) -> usize {
        self.current_depth
    }
}

impl XmlReader for DepthLimitedReader<'_> {
    fn extend_error(&self, error: XmlError) -> XmlError {
        let mut error = error;
        if error.position.is_none() {
            error.position = Some(self.inner.buffer_position());
        }
        error
    }
}

impl<'a> XmlReaderSync<'a> for DepthLimitedReader<'a> {
    /// Reads the next XML event, tracking depth and enforcing the max_depth limit.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The inner reader returns an error
    /// - The nesting depth exceeds the configured limit
    fn read_event(&mut self) -> Result<Event<'a>, XmlError> {
        let event = self.inner.read_event().map_err(XmlError::from)?;

        // Track depth: Start events increase depth, End events decrease it.
        // Empty events (self-closing tags like <tag/>) represent both start and end,
        // so they temporarily increase depth by 1 and then decrease it.
        match &event {
            Event::Start(_) => {
                self.current_depth += 1;
                self.check_depth_limit()?;
            }
            Event::Empty(_) => {
                // Self-closing tags: temporarily enter the element to check depth
                self.current_depth += 1;
                self.check_depth_limit()?;
                // Immediately exit the element (self-closing means start+end)
                self.current_depth -= 1;
            }
            Event::End(_) => {
                // Ensure we don't underflow (malformed XML could have mismatched tags)
                if self.current_depth > 0 {
                    self.current_depth -= 1;
                }
            }
            _ => {}
        }

        Ok(event)
    }
}

impl<'a> DepthLimitedReader<'a> {
    /// Check if current depth exceeds limit and return error if so.
    fn check_depth_limit(&self) -> Result<(), XmlError> {
        if self.current_depth > self.config.max_depth {
            let parse_error = hpxml_common::ParseError::DepthLimitExceeded {
                depth: self.current_depth,
                limit: self.config.max_depth,
            };
            return Err(XmlError::custom(parse_error));
        }
        Ok(())
    }
}

#[cfg(test)]
fn assert_depth_limit_error(err: &XmlError, expected_limit: usize) {
    let xsd_parser_types::quick_xml::ErrorKind::Custom(source) = &err.kind else {
        panic!("expected custom error kind, got: {err:?}");
    };
    let Some(parse_error) = source.as_ref().downcast_ref::<hpxml_common::ParseError>() else {
        panic!("expected ParseError source, got: {source}");
    };
    let hpxml_common::ParseError::DepthLimitExceeded { depth, limit } = parse_error else {
        panic!("expected DepthLimitExceeded, got: {parse_error:?}");
    };
    assert_eq!(*limit, expected_limit);
    assert!(*depth > expected_limit);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config(max_depth: usize) -> ParseConfig {
        ParseConfig {
            max_depth,
            ..ParseConfig::default()
        }
    }

    /// Helper: reads all events from reader, returns (events, optional error)
    fn read_all_events<'a>(
        reader: &'a mut DepthLimitedReader<'a>,
    ) -> (Vec<Event<'a>>, Option<XmlError>) {
        let mut events = Vec::new();
        loop {
            match reader.read_event() {
                Ok(Event::Eof) => break,
                Ok(e) => events.push(e),
                Err(e) => {
                    return (events, Some(e));
                }
            }
        }
        (events, None)
    }

    /// Helper: reads all events and collects events with their depths
    fn read_events_with_depths<'a>(
        reader: &'a mut DepthLimitedReader<'a>,
    ) -> Vec<(Event<'a>, usize)> {
        let mut results = Vec::new();
        loop {
            match reader.read_event() {
                Ok(Event::Eof) => break,
                Ok(event) => {
                    results.push((event, reader.current_depth()));
                }
                Err(e) => {
                    panic!("Unexpected error: {:?}", e);
                }
            }
        }
        results
    }

    #[test]
    fn test_depth_exactly_at_limit_succeeds() {
        // Create XML with depth exactly at limit (3 nested elements)
        let xml = r#"<a><b><c></c></b></a>"#;
        let mut reader = DepthLimitedReader::from_str(xml, test_config(3));

        let (events, maybe_err) = read_all_events(&mut reader);
        assert!(maybe_err.is_none(), "Depth should not be exceeded at limit");

        // Should have: Start(a), Start(b), Start(c), End(c), End(b), End(a)
        let start_count = events
            .iter()
            .filter(|e| matches!(e, Event::Start(_)))
            .count();
        let end_count = events.iter().filter(|e| matches!(e, Event::End(_))).count();
        assert_eq!(start_count, 3, "Expected 3 start events");
        assert_eq!(end_count, 3, "Expected 3 end events");
    }

    #[test]
    fn test_depth_exceeds_limit_fails() {
        // Create XML with depth at limit + 1 (4 nested elements with limit of 3)
        let xml = r#"<a><b><c><d></d></c></b></a>"#;
        let mut reader = DepthLimitedReader::from_str(xml, test_config(3));

        let (_, maybe_err) = read_all_events(&mut reader);
        let err = maybe_err.expect("expected depth limit error");
        assert_depth_limit_error(&err, 3);
    }

    #[test]
    fn test_depth_tracking() {
        let xml = r#"<a><b><c></c><d></d></b></a>"#;
        let mut reader = DepthLimitedReader::from_str(xml, test_config(10));

        let events_with_depths: Vec<_> = read_events_with_depths(&mut reader)
            .into_iter()
            .filter(|(e, _)| matches!(e, Event::Start(_) | Event::End(_)))
            .collect();

        // Verify depth was tracked correctly
        // Depth goes: 0 -> 1 (Start a) -> 2 (Start b) -> 3 (Start c) -> 2 (End c) -> 3 (Start d) -> 2 (End d) -> 1 (End b) -> 0 (End a)
        // That's 8 events: Start a, Start b, Start c, End c, Start d, End d, End b, End a
        assert_eq!(events_with_depths.len(), 8, "Expected 8 non-eof events");
    }

    #[test]
    fn test_empty_document() {
        let xml = "";
        let mut reader = DepthLimitedReader::from_str(xml, test_config(10));

        let event = reader.read_event();
        assert!(matches!(event, Ok(Event::Eof)));
    }

    #[test]
    fn test_self_closing_tags() {
        // Self-closing tags like <b/> generate Event::Empty, not Start+End
        let xml = r#"<a><b/><c/><d></d></a>"#;
        let mut reader = DepthLimitedReader::from_str(xml, test_config(3));

        let (events, maybe_err) = read_all_events(&mut reader);
        assert!(maybe_err.is_none(), "Depth should not be exceeded");

        // Should have: Start(a), Empty(b), Empty(c), Start(d), End(d), End(a)
        let start_count = events
            .iter()
            .filter(|e| matches!(e, Event::Start(_)))
            .count();
        let empty_count = events
            .iter()
            .filter(|e| matches!(e, Event::Empty(_)))
            .count();
        let end_count = events.iter().filter(|e| matches!(e, Event::End(_))).count();

        assert_eq!(start_count, 2, "Expected 2 start events (a, d)");
        assert_eq!(empty_count, 2, "Expected 2 empty events (b, c)");
        assert_eq!(end_count, 2, "Expected 2 end events (d, a)");
    }

    #[test]
    fn test_xml_declaration_does_not_affect_depth() {
        // XML declarations like <?xml version="1.0"?> should not affect depth
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?><a><b><c></c></b></a>"#;
        let mut reader = DepthLimitedReader::from_str(xml, test_config(3));

        let (_, maybe_err) = read_all_events(&mut reader);
        assert!(
            maybe_err.is_none(),
            "XML with declaration should work at depth 3"
        );
    }

    #[test]
    fn test_comments_do_not_affect_depth() {
        // Comments <!-- comment --> should not affect depth
        let xml = r#"<a><!-- comment --><b><c></c></b></a>"#;
        let mut reader = DepthLimitedReader::from_str(xml, test_config(3));

        let (_, maybe_err) = read_all_events(&mut reader);
        assert!(
            maybe_err.is_none(),
            "XML with comment should work at depth 3"
        );
    }

    #[test]
    fn test_whitespace_does_not_affect_depth() {
        // Whitespace between elements should not affect depth
        let xml = r#"<a>

  <b>

    <c></c>

  </b>

</a>"#;
        let mut reader = DepthLimitedReader::from_str(xml, test_config(3));

        let (_, maybe_err) = read_all_events(&mut reader);
        assert!(
            maybe_err.is_none(),
            "XML with whitespace should work at depth 3"
        );
    }

    #[test]
    fn test_processing_instruction_does_not_affect_depth() {
        // Processing instructions <?target content?> should not affect depth
        let xml = r#"<a><?some-pi data?><b><c></c></b></a>"#;
        let mut reader = DepthLimitedReader::from_str(xml, test_config(3));

        let (_, maybe_err) = read_all_events(&mut reader);
        assert!(
            maybe_err.is_none(),
            "XML with processing instruction should work at depth 3"
        );
    }

    #[test]
    fn test_cdata_does_not_affect_depth() {
        // CDATA sections should not affect depth
        let xml = r#"<a><![CDATA[<some>content</more>]]><b><c></c></b></a>"#;
        let mut reader = DepthLimitedReader::from_str(xml, test_config(3));

        let (_, maybe_err) = read_all_events(&mut reader);
        assert!(maybe_err.is_none(), "XML with CDATA should work at depth 3");
    }

    #[test]
    fn test_mixed_content_depth_tracking() {
        // Test depth tracking with mixed content: text, elements, whitespace
        // Note: Using <c></c> not <c/> because self-closing tags increment and
        // decrement depth within the same read_event() call
        let xml = r#"<a>text1<b>text2<c>text3</c>text4</b>text5</a>"#;
        let mut reader = DepthLimitedReader::from_str(xml, test_config(10));

        let events_with_depths = read_events_with_depths(&mut reader);

        // Extract just the depths for Start/End/Empty events
        let depths: Vec<usize> = events_with_depths
            .into_iter()
            .filter(|(e, _)| matches!(e, Event::Start(_) | Event::End(_) | Event::Empty(_)))
            .map(|(_, d)| d)
            .collect();

        // Verify depth tracking is correct:
        // Start(a)=1, Start(b)=2, Start(c)=3, End(c)=2, End(b)=1, End(a)=0
        assert!(
            depths.iter().any(|&d| d == 1),
            "Should have depth 1 for element a, got {:?}",
            depths
        );
        assert!(
            depths.iter().any(|&d| d == 2),
            "Should have depth 2 for element b, got {:?}",
            depths
        );
        assert!(
            depths.iter().any(|&d| d == 3),
            "Should have depth 3 for element c, got {:?}",
            depths
        );
    }
}

#[cfg(all(test, feature = "v4"))]
mod integration_tests {
    use super::*;
    use hpxml_types_v4::HpxmlType;
    use xsd_parser_types::quick_xml::DeserializeSync;

    /// Integration test: verify DepthLimitedReader can deserialize minimal.xml
    /// This proves the wrapper approach works with xsd-parser's generated types.
    #[test]
    fn test_deserialize_minimal_xml() {
        let xml = include_bytes!("../tests/data/v4/minimal.xml");
        let config = ParseConfig::default();

        let mut reader = DepthLimitedReader::new(xml, config);

        // This should successfully deserialize into HpxmlType
        let result = HpxmlType::deserialize(&mut reader);

        assert!(
            result.is_ok(),
            "Failed to deserialize minimal.xml: {:?}",
            result
        );
    }

    /// Integration test: verify depth limit is enforced during deserialization
    #[test]
    fn test_depth_limit_enforced_during_deserialize() {
        // Create deeply nested HPXML that exceeds the depth limit
        // Use a simple nested structure within a valid HPXML wrapper
        let xml = r#"<?xml version='1.0' encoding='UTF-8'?>
<HPXML xmlns='http://hpxmlonline.com/2023/09' schemaVersion='4.2'>
  <XMLTransactionHeaderInformation>
    <XMLType>HPXML</XMLType>
    <XMLGeneratedBy>test</XMLGeneratedBy>
    <CreatedDateAndTime>2024-01-01T00:00:00-00:00</CreatedDateAndTime>
    <Transaction>create</Transaction>
  </XMLTransactionHeaderInformation>
  <SoftwareInfo/>
  <Building>
    <BuildingID id='Building1'/>
    <ProjectStatus>
      <EventType>audit</EventType>
    </ProjectStatus>
    <BuildingDetails>
      <Enclosure>
        <AirInfiltration>
          <AirInfiltrationMeasurement>
            <SystemIdentifier id='Infiltration1'/>
            <HousePressure>50</HousePressure>
            <BuildingAirLeakage>
              <UnitofMeasure>ACH</UnitofMeasure>
              <AirLeakage>5.0</AirLeakage>
            </BuildingAirLeakage>
          </AirInfiltrationMeasurement>
        </AirInfiltration>
      </Enclosure>
    </BuildingDetails>
  </Building>
</HPXML>"#;

        let config = ParseConfig {
            max_depth: 3, // This XML has depth ~7-8, this should fail
            ..ParseConfig::default()
        };

        // First verify the reader itself rejects deep XML - test with a clearly deep XML
        let deep_xml = "<a><b><c><d><e></e></d></c></b></a>";
        let config_for_test = ParseConfig {
            max_depth: 3,
            ..ParseConfig::default()
        };
        let mut reader = DepthLimitedReader::new(deep_xml.as_bytes(), config_for_test);
        loop {
            match reader.read_event() {
                Ok(quick_xml::events::Event::Eof) => panic!("expected depth limit error"),
                Ok(_) => {}
                Err(e) => {
                    assert_depth_limit_error(&e, 3);
                    break;
                }
            }
        }

        // Now verify the valid HPXML with low depth limit fails
        let mut reader2 = DepthLimitedReader::new(xml.as_bytes(), config);
        let result = HpxmlType::deserialize(&mut reader2);

        assert!(result.is_err(), "Expected error due to depth limit");
        let err = result.unwrap_err();
        assert_depth_limit_error(&err, 3);

        // Now verify minimal.xml also fails with a very low limit
        let minimal_xml = include_bytes!("../tests/data/v4/minimal.xml");
        let very_low_config = ParseConfig {
            max_depth: 2, // minimal.xml has depth ~7-8, this should definitely fail
            ..ParseConfig::default()
        };

        let mut reader3 = DepthLimitedReader::new(minimal_xml, very_low_config);
        let result2 = HpxmlType::deserialize(&mut reader3);

        assert!(
            result2.is_err(),
            "Expected error due to depth limit on minimal.xml"
        );
        let err2 = result2.unwrap_err();
        assert_depth_limit_error(&err2, 2);
    }

    /// Integration test: verify self-closing tags are handled correctly
    #[test]
    fn test_self_closing_tags_depth_limit() {
        // Self-closing tags at deep nesting should also trigger depth limit
        let xml = r#"<a><b><c><d/></c></b></a>"#;
        let config = ParseConfig {
            max_depth: 3,
            ..ParseConfig::default()
        };

        let mut reader = DepthLimitedReader::new(xml.as_bytes(), config);

        // Depth 4 should fail (a=1, b=2, c=3, d=4 via self-closing)
        loop {
            match reader.read_event() {
                Ok(quick_xml::events::Event::Eof) => panic!("expected depth limit error"),
                Ok(_) => {}
                Err(e) => {
                    assert_depth_limit_error(&e, 3);
                    break;
                }
            }
        }

        // But with limit 4, it should work
        let config_ok = ParseConfig {
            max_depth: 4,
            ..ParseConfig::default()
        };

        let mut reader2 = DepthLimitedReader::new(xml.as_bytes(), config_ok);
        let mut events = Vec::new();
        loop {
            match reader2.read_event() {
                Ok(quick_xml::events::Event::Eof) => break,
                Ok(e) => events.push(e),
                Err(e) => panic!("Unexpected error: {:?}", e),
            }
        }

        // Should have successfully read all events
        let empty_count = events
            .iter()
            .filter(|e| matches!(e, quick_xml::events::Event::Empty(_)))
            .count();
        assert_eq!(empty_count, 1, "Expected 1 empty event for <d/>");
    }
}

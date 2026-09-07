<?xml version="1.0" encoding="UTF-8"?>
<sch:schema xmlns:sch="http://purl.oclc.org/dsdl/schematron">
  <sch:title>Example IRA Measured Required 5 (Non-Official)</sch:title>
  <sch:ns uri="http://hpxmlonline.com/2023/09" prefix="h"/>

  <sch:pattern id="ira-measured-required-example">
    <sch:rule context="/h:HPXML/h:Building/h:BuildingDetails/h:Enclosure">
      <sch:assert test="count(h:Foundations/h:Foundation) &gt;= 1">
        At least one Foundation must be present.
      </sch:assert>
      <sch:assert test="count(h:FoundationWalls/h:FoundationWall) &gt;= 1">
        At least one FoundationWall must be present.
      </sch:assert>
    </sch:rule>
    <sch:rule context="/h:HPXML/h:Building/h:BuildingDetails/h:BuildingSummary/h:BuildingConstruction">
      <sch:assert test="number(h:NumberofBedrooms) &gt;= 1">
        NumberofBedrooms must be at least 1.
      </sch:assert>
    </sch:rule>
  </sch:pattern>
</sch:schema>

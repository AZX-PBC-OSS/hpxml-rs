<?xml version="1.0" encoding="UTF-8"?>
<sch:schema xmlns:sch="http://purl.oclc.org/dsdl/schematron">
  <sch:title>Example IRA Modeled Required 6 (Non-Official)</sch:title>
  <sch:ns uri="http://hpxmlonline.com/2023/09" prefix="h"/>

  <sch:pattern id="ira-modeled-required-example">
    <sch:rule context="/h:HPXML/h:Building/h:BuildingDetails/h:Enclosure">
      <sch:assert test="count(h:Walls/h:Wall[h:ExteriorAdjacentTo='outside']) &gt;= 1">
        At least one exterior wall needs to be present.
      </sch:assert>
    </sch:rule>
    <sch:rule context="//h:AirInfiltrationMeasurement">
      <sch:assert test="number(h:HousePressure)=50">
        Expected HousePressure to be 50.
      </sch:assert>
    </sch:rule>
  </sch:pattern>
</sch:schema>

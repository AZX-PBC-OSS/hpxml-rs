<?xml version="1.0" encoding="UTF-8"?>
<sch:schema xmlns:sch="http://purl.oclc.org/dsdl/schematron">
  <sch:title>Example Home Energy Score Use Case 1 (Non-Official)</sch:title>
  <sch:ns uri="http://hpxmlonline.com/2023/09" prefix="h"/>

  <sch:pattern id="home-energy-score-example">
    <sch:rule context="/h:HPXML/h:Building/h:BuildingDetails">
      <sch:assert test="count(h:Systems/h:HVAC/h:HVACPlant) &gt;= 1">
        HVACPlant must be present for Home Energy Score use-case checks.
      </sch:assert>
    </sch:rule>
    <sch:rule context="/h:HPXML/h:Building/h:BuildingDetails/h:Systems/h:HVAC/h:HVACPlant">
      <sch:assert test="count(h:HeatingSystem | h:HeatPump) &gt;= 1">
        At least one HeatingSystem or HeatPump must be present.
      </sch:assert>
      <sch:assert test="count(h:CoolingSystem | h:HeatPump) &gt;= 1">
        At least one CoolingSystem or HeatPump must be present.
      </sch:assert>
    </sch:rule>
  </sch:pattern>
</sch:schema>

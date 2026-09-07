<?xml version="1.0" encoding="UTF-8"?>
<sch:schema xmlns:sch="http://purl.oclc.org/dsdl/schematron">
  <sch:title>Example DER Custom Rules (Non-Official)</sch:title>
  <sch:ns uri="http://hpxmlonline.com/2023/09" prefix="h"/>

  <sch:pattern id="der-example">
    <sch:rule context="/h:HPXML/h:Building/h:BuildingDetails">
      <sch:assert test="count(h:Systems) &gt;= 1">
        Systems section must be present.
      </sch:assert>
      <sch:assert test="count(h:Systems/h:Photovoltaics/h:PVSystem) + count(h:Systems/h:Batteries/h:Battery) + count(h:Systems/h:ElectricVehicleChargers/h:ElectricVehicleCharger) + count(h:Systems/h:Vehicles/h:Vehicle) &gt;= 1">
        At least one DER asset is required (PV, Battery, EV Charger, or Vehicle).
      </sch:assert>
    </sch:rule>
    <sch:rule context="//h:Battery">
      <sch:assert test="number(h:UsableCapacity/h:Value) &gt; 0">
        Battery usable capacity must be greater than 0.
      </sch:assert>
    </sch:rule>
  </sch:pattern>
</sch:schema>

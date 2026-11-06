## Why
Auf Messen sollen sich Badges bei Begegnung freuen. Je öfter man dieselbe Badge trifft, desto stärker die Reaktion, ohne dass Dauergespräche ständig auslösen.

## What Changes
- Funkbasierte Erkennung anderer Badges per Bluetooth LE (Beacon mit fester Badge-ID).
- Begegnung gilt bei ausreichend starkem und stabilem Signal (Startwert etwa −65 dBm, mehrere Messungen in wenigen Sekunden).
- Pause von 10 Minuten pro Badge gegen Dauerauslösung.
- Begegnungsgedächtnis pro Badge mit 5 Stufen; ohne Treffen sinkt die Stufe alle 30 Minuten um eins.
- Dieselbe Freude-Animation steigert sich mit der Stufe (mehr und größere Herzen, stärkerer Ausdruck, Klatschen).
- Gedächtnis im Flash für die letzten ca. 32 Kontakte.

## Capabilities
### New Capabilities
- `encounter-detection`: Erkennung per BLE mit Schwellwert und Pause.
- `encounter-memory`: Stufen pro Badge, Abklingen, Speicherung.
- `encounter-reaction`: gesteigerte Freude-Animation.

### Modified Capabilities

## Impact
Bluetooth-Stack und Flash-Speicher; Auswirkung auf Stromverbrauch. Abhängig von `character-themes` (Reaktion je Figur).

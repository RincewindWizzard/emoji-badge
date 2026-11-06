## ADDED Requirements

### Requirement: Wischen im Karussell
Die Firmware SHALL durch horizontales Wischen zur nächsten oder vorherigen Emotion wechseln. Die Reihenfolge SHALL ein Kreis sein, ohne Anfang und Ende.

#### Scenario: Wischen nach links
- **WHEN** der Nutzer nach links wischt
- **THEN** SHALL die nächste Emotion erscheinen

#### Scenario: Ende der Liste
- **WHEN** der Nutzer an der letzten Emotion nach links wischt
- **THEN** SHALL die erste Emotion erscheinen

### Requirement: Bedienung ohne Sprache
Alle Funktionen der Emotionsauswahl SHALL ohne Sprachsteuerung bedienbar sein.

#### Scenario: Kein Mikrofon
- **WHEN** das Mikrofon nie benutzt wird
- **THEN** SHALL die Auswahl vollständig per Wischen und Tippen funktionieren

## ADDED Requirements

### Requirement: Erkennung per Bluetooth LE
Die Badge SHALL ein Beacon mit einer festen Badge-ID senden und Beacons anderer Badges empfangen.

#### Scenario: Zwei Badges nebeneinander
- **WHEN** zwei Badges in Reichweite sind
- **THEN** SHALL jede die ID der anderen erkennen

### Requirement: Nähe-Schwelle
Eine Begegnung SHALL erst gelten, wenn das geglättete Signal einen konfigurierbaren Schwellwert (Startwert −65 dBm) über mindestens 3 Messungen innerhalb von 5 Sekunden überschreitet.

#### Scenario: Kurzes Vorbeigehen
- **WHEN** eine Badge nur kurz schwach empfangen wird
- **THEN** SHALL keine Begegnung ausgelöst werden

### Requirement: Pause pro Badge
Nach einer Begegnung SHALL für dieselbe Badge 10 Minuten lang keine neue Begegnung zählen.

#### Scenario: Gemeinsam unterwegs
- **WHEN** zwei Badges dauerhaft nebeneinander bleiben
- **THEN** SHALL innerhalb der Pause keine weitere Reaktion ausgelöst werden

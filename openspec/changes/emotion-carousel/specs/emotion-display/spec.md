## ADDED Requirements

### Requirement: Grundemotionen
Die Firmware SHALL mindestens die Emotionen fröhlich, traurig, überrascht, wütend, verliebt, müde und neutral als Gesicht darstellen.

#### Scenario: Emotion wird angezeigt
- **WHEN** eine Emotion ausgewählt ist
- **THEN** SHALL das zugehörige Gesicht auf dem Display erscheinen

### Requirement: Eintrittsanimation und Halten
Beim Wechsel zu einer Emotion SHALL eine kurze Eintrittsanimation (ca. 0,5–1 s) spielen. Danach SHALL die Emotion ihren Ausdruck unverändert halten, bis sie angetippt oder gewechselt wird.

#### Scenario: Wechsel
- **WHEN** zu einer Emotion gewechselt wird
- **THEN** SHALL die Eintrittsanimation einmal spielen
- **AND** die Emotion SHALL danach im Ausdruck verharren

#### Scenario: Antippen
- **WHEN** der Nutzer die gehaltene Emotion antippt
- **THEN** SHALL die Eintrittsanimation erneut spielen

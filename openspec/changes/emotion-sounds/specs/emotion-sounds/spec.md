## ADDED Requirements

### Requirement: Melodie je Emotion
Jede Emotion SHALL eine eigene kurze Melodie besitzen, die zur Stimmung passt (Trauer: Trauermarsch-Auszug, Freude: Tusch, fröhlich: trällernde Tonfolge).

#### Scenario: Taste drücken
- **WHEN** der Nutzer die BOOT-Taste drückt
- **THEN** SHALL die Melodie der aktuell angezeigten Emotion einmal vollständig spielen

### Requirement: Nie ungewollter Ton
Die Firmware SHALL Töne nur nach einem Tastendruck abspielen.

#### Scenario: Wechsel der Emotion
- **WHEN** die Emotion per Wischen oder Schütteln wechselt
- **THEN** SHALL kein Ton spielen

### Requirement: Lautstärke
Die Firmware SHALL eine maximale Lautstärke nicht überschreiten und eine Stummschaltung erlauben.

#### Scenario: Stummgeschaltet
- **WHEN** die Stummschaltung aktiv ist
- **THEN** SHALL ein Tastendruck keinen Ton erzeugen

## ADDED Requirements

### Requirement: Wischen im Karussell
Die Firmware SHALL durch horizontales Wischen die Emotion wechseln: Wischen von links nach rechts zeigt die nächste Emotion, Wischen von rechts nach links die vorherige. Die Reihenfolge SHALL ein Kreis sein, ohne Anfang und Ende.

#### Scenario: Wischen von links nach rechts
- **WHEN** der Nutzer von links nach rechts wischt
- **THEN** SHALL die nächste Emotion erscheinen

#### Scenario: Wischen von rechts nach links
- **WHEN** der Nutzer von rechts nach links wischt
- **THEN** SHALL die vorherige Emotion erscheinen

#### Scenario: Ende der Liste
- **WHEN** der Nutzer an der letzten Emotion von links nach rechts wischt
- **THEN** SHALL die erste Emotion erscheinen

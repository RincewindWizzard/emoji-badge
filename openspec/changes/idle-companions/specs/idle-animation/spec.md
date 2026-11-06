## ADDED Requirements

### Requirement: Lebendiger Ruhezustand
Eine Figur im Ruhezustand SHALL in zufälligen Abständen kleine Bewegungen zeigen (Blinzeln, Atmen und figurspezifische Bewegungen).

#### Scenario: Katze im Ruhezustand
- **WHEN** die Katze lange nicht berührt wurde
- **THEN** SHALL sie gelegentlich atmen und ein Ohr wackeln

### Requirement: Sparsam
Im Ruhezustand SHALL die Firmware Bildrate und Helligkeit reduzieren dürfen.

#### Scenario: Lange Ruhe
- **WHEN** längere Zeit keine Eingabe erfolgt
- **THEN** SHALL das Display gedimmt werden

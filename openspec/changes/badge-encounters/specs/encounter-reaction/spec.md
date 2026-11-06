## ADDED Requirements

### Requirement: Freude bei Begegnung
Bei einer Begegnung SHALL die Figur sich freuen und ein animiertes Herz aufsteigen lassen.

#### Scenario: Erste Begegnung
- **WHEN** eine Begegnung der Stufe 1 gezählt wird
- **THEN** SHALL ein kleines Herz aufsteigen und die Figur dezent lächeln

### Requirement: Steigerung
Die Reaktion SHALL dieselbe Animation bleiben und sich mit der Stufe steigern: mehr und größere Herzen, stärkerer Ausdruck, Klatschen bei hohen Stufen.

#### Scenario: Stufe 5
- **WHEN** eine Begegnung der Stufe 5 gezählt wird
- **THEN** SHALL die Figur klatschen und mehrere große Herzen aufsteigen lassen

### Requirement: Reaktion unabhängig vom Ton
Die Begegnungsreaktion SHALL keinen Ton auslösen.

#### Scenario: Begegnung
- **WHEN** eine Begegnung gezählt wird
- **THEN** SHALL kein Ton spielen

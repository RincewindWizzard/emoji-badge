## ADDED Requirements

### Requirement: Stufen pro Badge
Die Badge SHALL pro anderer Badge eine Stufe von 1 bis 5 führen: Stufe 1 beim ersten Treffer, 2 bei 2–3, 3 bei 4–7, 4 bei 8–15, 5 ab 16 Treffern.

#### Scenario: Dritte Begegnung
- **WHEN** dieselbe Badge zum dritten Mal gezählt wird
- **THEN** SHALL die Stufe 2 gelten

### Requirement: Abklingen
Ohne Treffer SHALL die Stufe jeder Badge alle 30 Minuten um eins sinken, bis Stufe 1 erreicht ist.

#### Scenario: Zwei Stunden ohne Treffen
- **WHEN** 2 Stunden ohne Treffen vergehen
- **THEN** SHALL eine Badge mit Stufe 5 wieder auf Stufe 1 stehen

### Requirement: Speicherung
Die Badge SHALL die letzten etwa 32 Kontakte dauerhaft speichern und beim Überlauf den ältesten verdrängen.

#### Scenario: Neustart
- **WHEN** die Badge neu gestartet wird
- **THEN** SHALL der Stand der Kontakte erhalten bleiben

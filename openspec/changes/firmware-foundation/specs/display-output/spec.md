## ADDED Requirements

### Requirement: Bildausgabe auf das runde Display
Die Firmware SHALL ein 466×466-Bild im RGB565-Format über QSPI auf dem CO5300 ausgeben. Pixel außerhalb des Kreises SHALL schwarz sein.

#### Scenario: Farbtest beim Start
- **WHEN** das Gerät startet
- **THEN** SHALL der Selbsttest Rot, Grün, Blau und einen Kreis anzeigen
- **AND** das Display SHALL nach dem Test den Normalbetrieb zeigen

### Requirement: Helligkeitssteuerung
Die Firmware SHALL die Helligkeit über CO5300-Befehle einstellen, weil es kein GPIO-Backlight gibt.

#### Scenario: Helligkeit ändern
- **WHEN** ein Helligkeitswert gesetzt wird
- **THEN** SHALL das Display die neue Helligkeit zeigen, ohne das Bild neu zu übertragen

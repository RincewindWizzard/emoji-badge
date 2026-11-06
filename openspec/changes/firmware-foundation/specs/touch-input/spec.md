## ADDED Requirements

### Requirement: Touch-Ereignisse
Die Firmware SHALL Berührungen des CST9217 als Ereignisse (Tippen, Wischen links/rechts mit Position) bereitstellen.

#### Scenario: Tippen
- **WHEN** der Nutzer das Display kurz berührt
- **THEN** SHALL ein Tipp-Ereignis mit Koordinaten entstehen

#### Scenario: Wischen
- **WHEN** der Nutzer horizontal über das Display wischt
- **THEN** SHALL ein Wisch-Ereignis mit Richtung entstehen

#### Scenario: Controller nicht erreichbar
- **WHEN** der CST9217 beim Start nicht antwortet
- **THEN** SHALL die Firmware den Fehler melden und mehrfach neu versuchen, ohne zu blockieren

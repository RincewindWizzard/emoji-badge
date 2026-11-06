## ADDED Requirements

### Requirement: Einheitliches Figurenmuster
Jede Figur SHALL alle Grundemotionen, einen Ruhezustand und eine Begegnungsreaktion darstellen können.

#### Scenario: Vollständigkeit
- **WHEN** eine Figur zum Projekt hinzugefügt wird
- **THEN** SHALL ein Test prüfen, dass sie jede Grundemotion liefert

### Requirement: Figurenwahl
Der Nutzer SHALL die aktive Figur wechseln können, ohne die Firmware neu zu flashen.

#### Scenario: Figur wechseln
- **WHEN** der Nutzer die Figurenauswahl öffnet und eine Figur wählt
- **THEN** SHALL das Karussell mit dieser Figur weiterlaufen

### Requirement: Figurenbestand
Das Projekt SHALL schrittweise folgende Figuren bereitstellen: Katze, Hund, Maus, Hase, Vogel, Kürbis, Geist, Hexe, Zauberer, Skelett, Fledermaus, Einhorn, Weihnachtsmann, Roboter und eine Trauerfigur.

#### Scenario: Erste Welle
- **WHEN** die erste Lieferung abgeschlossen ist
- **THEN** SHALL mindestens Katze, Kürbis und Roboter wählbar sein

### Requirement: Trauerfigur
Die Trauerfigur SHALL ruhig auftreten und bei Begegnung ein kleines schwarzes Herz zeigen, das verblasst, ohne fröhliche Reaktion.

#### Scenario: Begegnung bei der Trauerfigur
- **WHEN** die Trauerfigur einer anderen Badge begegnet
- **THEN** SHALL ein kleines schwarzes Herz erscheinen und verblassen
- **AND** es SHALL keine Freude-Animation und kein Klatschen spielen

## ADDED Requirements

### Requirement: Push-to-Talk
Das Mikrofon SHALL nur aufnehmen, solange die dafür vorgesehene Taste gehalten wird.

#### Scenario: Keine Aufnahme ohne Taste
- **WHEN** die Taste nicht gedrückt ist
- **THEN** SHALL keine Audioaufnahme stattfinden

### Requirement: Offline-Erkennung
Die Erkennung SHALL vollständig auf dem Gerät ohne Netzwerkverbindung erfolgen. Aufnahmen SHALL nach der Erkennung verworfen werden.

#### Scenario: Ohne WLAN
- **WHEN** kein WLAN verfügbar ist
- **THEN** SHALL die Erkennung trotzdem funktionieren

### Requirement: Befehle zu Emotionen
Ein erkanntes Befehlswort SHALL die zugehörige Emotion anzeigen.

#### Scenario: Wort "angry"
- **WHEN** der Nutzer bei gehaltener Taste "angry" sagt
- **THEN** SHALL die wütende Emotion mit ihrer Eintrittsanimation erscheinen

### Requirement: Nicht erforderlich
Alle anderen Funktionen SHALL ohne Spracherkennung vollständig bedienbar bleiben.

#### Scenario: Funktion deaktiviert
- **WHEN** die Spracherkennung deaktiviert ist
- **THEN** SHALL das Gerät unverändert per Wischen und Tippen bedienbar sein

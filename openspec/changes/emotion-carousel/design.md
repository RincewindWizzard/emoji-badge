## Context
`badge-core` rendert bisher statische Gesichter. Animationen brauchen einen zeitgesteuerten Zustand, der auf dem Host testbar bleiben soll.

## Goals / Non-Goals
- Goals: deterministische Animationslogik im Kern (Zeit rein, Parameter raus), Rendern aus Parametern.
- Non-Goals: Figurensätze (`character-themes`), Ton (`emotion-sounds`).

## Decisions
- **Zustandsautomat** pro Emotion: Eintritt (feste Dauer, ca. 600 ms) → Halten. Zeitquelle wird als Parameter übergeben, dadurch ohne Hardware testbar.
- **Animation als Funktion von Zeit** (Parameter wie Augenöffnung, Mundkurve), nicht als Bildfolge: wenig Speicher, skalierbar für Stärkestufen.
- **Karussell** als Index mit Modulo; Wisch-Übergang schiebt das alte Gesicht heraus.
- **Schütteln** über Beschleunigungsbetrag mit Schwellwert und Sperrzeit gegen Doppelauslösung.

## Risks / Trade-offs
- [Wisch und Tippen verwechselt] → Schwellwerte für Weg und Zeit in Konfiguration.
- [IMU-Rauschen löst Schütteln aus] → Schwelle und Filter, am Gerät abstimmen.

## Open Questions
- Endgültige Liste der Emotionen und ihre Reihenfolge im Karussell.

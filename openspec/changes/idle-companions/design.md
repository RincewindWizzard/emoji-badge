## Context
Idle-Bewegungen müssen selten und zufällig sein, damit sie natürlich wirken und wenig Strom brauchen.

## Goals / Non-Goals
- Goals: Ruhe-Animationsrahmen, Schlummer/Aufwach-Zustände.
- Non-Goals: Tageszeit-Verhalten und Anlass-Automatik (spätere Ideen).

## Decisions
- **Zustände**: Aktiv → Idle (nach Inaktivität) → Schlummern; Begegnung oder Tippen weckt.
- **Idle-Ereignisse mit zufälligen Pausen** (Zufallszahl, nicht im Takt).
- **Gemeinsame Hilfen** (Atmen, Blinzeln) und figurenspezifische Bewegungen als Daten.
- **Dimmen und niedrigere Bildrate** im Schlummern für Strom.

## Risks / Trade-offs
- [Zufall wirkt hakelig] → Pausenbereiche testen, weiche Übergänge.
- [Wecken durch Rauschen] → Wecken nur durch Touch oder Begegnung.

## Open Questions
- Zeit bis zum Einschlummern.

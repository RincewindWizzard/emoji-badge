## Context
Emotionen sind parametrisiert (`emotion-carousel`). Figuren müssen dieselben Parameter auf andere Formen abbilden.

## Goals / Non-Goals
- Goals: kleines, einheitliches Figuren-Interface, wenig Speicher, Stufen skalierbar.
- Non-Goals: Begegnungslogik und Schlummerverhalten selbst (eigene Changes).

## Decisions
- **Prozedural aus einfachen Formen** (Kreise, Bögen, Polygone) statt Bitmaps: bei 14 Figuren mit allen Zuständen wären Bilder zu groß und nicht steigerbar.
- **Figur = Trait/Tabelle** mit Emotion → Parametersatz; eine Figur muss alle Emotionen liefern (Pflicht, per Test geprüft).
- **Lieferung in Wellen**: Muster mit drei Figuren, danach einzeln.
- **Trauerfigur ohne Freude-Herzchen**: Begegnung zeigt ein schwarzes Herz, das verblasst.

## Risks / Trade-offs
- [14 Figuren × alle Emotionen = viel Arbeit] → Wellen, Test für Vollständigkeit.
- [Wiedererkennbarkeit auf 1,75"] → Figuren großflächig und kontrastreich zeichnen.

## Open Questions
- Auswahlgeste für den Figurenwechsel (länger drücken oder Randwisch).

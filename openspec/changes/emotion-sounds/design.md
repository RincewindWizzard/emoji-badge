## Context
Wiedergabe über ES8311 (I2C 0x18) mit I2S (MCLK 42, BCLK 9, WS 45, DOUT 8) und Verstärker NS4150B (PA_CTRL GPIO46). Siehe `board-pinout`.

## Goals / Non-Goals
- Goals: einfacher Tonsequenzer, Melodien als Daten, einmalige Wiedergabe.
- Non-Goals: Mikrofonaufnahme, Streaming, Sprachausgabe.

## Decisions
- **Eigener Sequenzer** (Note, Dauer, Lautstärke) mit Rechteck/Sinus-Synthese statt Audiodateien: wenig Speicher, leicht änderbar.
- **Melodien nur gemeinfrei oder selbst komponiert**, wegen Urheberrecht.
- **Nur auf Tastendruck**, nie automatisch; auch Begegnungen lösen keinen Ton aus, außer später per Konfiguration.
- **Codec-Setup minimal** (Abtastrate, Lautstärke), Verstärker erst kurz vor dem Ton einschalten (Knackser, Strom).

## Risks / Trade-offs
- [Codec-Initialisierung aufwändig] → eigener Change, einzeln testbar.
- [Knacken beim Ein/Ausschalten des Verstärkers] → Fade-in/out.
- [Zu laut auf der Messe] → Lautstärkenlimit und Stummschalter.

## Open Questions
- Aktiver Pegel von PA_CTRL (am Gerät messen).

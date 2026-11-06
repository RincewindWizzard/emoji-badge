## Context
Mikrofone hängen am ES7210 (TDM, MIC1/MIC2, MIC3 als Referenz). Espressif MultiNet unterstützt auf dem ESP32-S3 laut Doku nur Chinesisch und Englisch.

## Goals / Non-Goals
- Goals: Push-to-Talk, lokale Erkennung, Datenschutz.
- Non-Goals: Wake Word, Cloud, Dauerlauschen, Sprachausgabe.

## Decisions
- **Push-to-Talk**: Aufnahme nur bei gehaltener Taste (z. B. langer Druck auf BOOT), Puffer wenige Sekunden im PSRAM, danach verworfen.
- **Zuerst englische Befehle** mit MultiNet; deutsches eigenes Modell als Experiment.
- **Framework-Entscheidung** erst bei Beginn dieses Changes (ggf. Wechsel auf `esp-idf-svc`).

## Risks / Trade-offs
- [Deutsch nicht unterstützt] → Englisch oder eigenes Modell.
- [Erkennung in lauter Messeumgebung schwach] → Funktion bleibt optional.
- [Framework-Wechsel] → Kern in `core/` bleibt unabhängig.

## Open Questions
- Befehlswortliste und Sprache.

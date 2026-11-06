## Context
Siehe `openspec/specs/board-pinout` und `board-hardware`. Auf der Entwicklungsumgebung ist keine Xtensa-Toolchain vorhanden; Hardware-Code kann dort nicht gebaut werden. Die Rendering-Logik liegt in `core/` und ist auf dem Host testbar.

## Goals / Non-Goals
- Goals: bootendes Gerüst, Display- und Touch-Treiber, Selbsttest, reproduzierbarer Build.
- Non-Goals: Emotionen, Audio, Funk, Energieverwaltung.

## Decisions
- **`esp-hal` (bare-metal) statt `esp-idf-svc`**: kleiner, passt zum `no_std`-Kern. Alternative `esp-idf-svc` wäre nötig für C-Bibliotheken wie ESP-SR; falls `voice-commands` das erzwingt, wird neu entschieden.
- **Zeilenweises Rendern** (Zeilenpuffer) statt Vollbild-Framebuffer: spart RAM; ein Framebuffer im PSRAM bleibt möglich.
- **Pins in einem Modul `board`** gebündelt, damit Abweichungen beim Modell "C" an einer Stelle korrigiert werden.
- **CO5300-Treiber**: zuerst vorhandenes Crate prüfen, sonst Initsequenz aus der Espressif-Komponente portieren.

## Risks / Trade-offs
- [Pinbelegung weicht beim Modell 1.75C ab] → Selbsttest zuerst, Pins zentral.
- [QSPI-Durchsatz reicht für flüssige Animation nicht] → Teilbereiche aktualisieren, Messung früh.
- [Keine Toolchain in CI] → CI-Job mit `espup` einplanen.

## Open Questions
- Tatsächliche Flash-Größe (16 oder 32 MB) per `espflash board-info`.

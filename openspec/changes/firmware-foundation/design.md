## Context
Siehe `openspec/specs/board-pinout` und `board-hardware`. Auf der Entwicklungsumgebung ist keine Xtensa-Toolchain vorhanden; Hardware-Code kann dort nicht gebaut werden. Die Rendering-Logik liegt in `core/` und ist auf dem Host testbar.

## Goals / Non-Goals
- Goals: bootendes Gerüst, Display- und Touch-Treiber, Selbsttest, reproduzierbarer Build.
- Non-Goals: Emotionen, Audio, Funk, Energieverwaltung.

## Decisions
- **`esp-hal` (bare-metal)**: klein, passt zum `no_std`-Kern. Es gibt keine C-Bibliotheken-Abhängigkeit, die `esp-idf-svc` erzwingen würde.
- **Zeilenweises Rendern** (Zeilenpuffer) statt Vollbild-Framebuffer: spart RAM; ein Framebuffer im PSRAM bleibt möglich.
- **Pins in einem Modul `board`** gebündelt, damit Abweichungen beim Modell "C" an einer Stelle korrigiert werden.
- **CO5300-Treiber**: zuerst vorhandenes Crate prüfen, sonst Initsequenz aus der Espressif-Komponente portieren.

## Risks / Trade-offs
- [Pinbelegung weicht beim Modell 1.75C ab] → Selbsttest zuerst, Pins zentral.
- [QSPI-Durchsatz reicht für flüssige Animation nicht] → Teilbereiche aktualisieren, Messung früh.
- [Keine Toolchain in CI] → CI-Job mit `espup` einplanen.

## Open Questions
- Flash-Größe und Pinbelegung des Modells 1.75C sind nicht bekannt (kein Datenblatt vom Händler). Der Selbsttest ermittelt die Flash-Größe zur Laufzeit; die Pins stammen aus der Waveshare-Referenz und werden am Gerät geprüft.

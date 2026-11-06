## Why
Es gibt noch keine lauffähige Firmware. Alle weiteren Funktionen (Emotionen, Figuren, Begegnungen) brauchen zuerst ein Rust-Projekt, das auf dem Board bootet, das Display ansteuert und Touch liest. Die Pinbelegung stammt aus der Waveshare-Referenz und ist für das gekaufte Modell "1.75C" noch nicht bestätigt, daher ist diese Stufe bewusst klein und einzeln testbar.

## What Changes
- Neues Firmware-Crate `firmware/` (Rust, `no_std`, `esp-hal`, Xtensa-Toolchain) im Workspace; `core/` bleibt hardwareunabhängig.
- CO5300-Display über QSPI initialisieren, Zeilen aus `badge-core` ausgeben, Helligkeit per Panelbefehl.
- Geteilter I2C-Bus (GPIO14/15) und CST9217-Touch (Koordinaten, Tippen, Wischrichtung).
- Hardware-Selbsttest beim Start (Display-Farbtest, Touch-Ausgabe, Flash-Größe, I2C-Scan) zur Verifikation der Pinbelegung.
- Build-, Flash- und CI-Anleitung (espup, espflash).

## Capabilities
### New Capabilities
- `display-output`: Bild auf dem runden AMOLED ausgeben, Helligkeit und Schwarz außerhalb des Kreises.
- `touch-input`: Berührungen und Gesten vom CST9217 als Ereignisse bereitstellen.

### Modified Capabilities

## Impact
Neues Crate `firmware/`, Workspace-Konfiguration, Toolchain-Anforderung (Xtensa). Abhängigkeiten: `esp-hal`, `embedded-hal`, evtl. `display-driver-co5300` oder eigener Treiber nach `esp_lcd_co5300`-Initsequenz.

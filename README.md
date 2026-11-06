# emoji-badge

Eigene Rust-Firmware für das Board **ESP32-S3-Touch-AMOLED-1.75C** (466×466 rund, ESP32-S3R8).
Anforderungen: `openspec/` (`openspec list`, `openspec validate --all`).

## Aufbau
- `core/` – Emoji-Rendering, `no_std`, auf dem Host testbar
- `drivers/` – CO5300-Display, CST9217-Touch, Gestenerkennung (generisch über `embedded-hal`), auf dem Host testbar
- `firmware/` – ESP32-S3-Anbindung mit `esp-hal` (braucht Xtensa-Toolchain)

## Host-Tests
```bash
cargo test
```

## Firmware bauen und flashen
Einmalig die Toolchain installieren:
```bash
cargo install espup espflash
espup install
. ~/export-esp.sh
```
Bauen und flashen (Board per USB-C; BOOT halten und anstecken, falls nötig):
```bash
cd firmware
cargo run --release
```
Der Selbsttest zeigt ein Gesicht und gibt Berührungen mit den Rohbytes über die serielle Konsole aus.

## Status
Die Hardware-Anbindung (`firmware/`) und die Treiberdetails (CO5300-Init, CST9217-Berichtsformat, Pins des Modells 1.75C) sind **nicht am Gerät geprüft**. Beim ersten Flashen können Anpassungen nötig sein; die Rohausgabe des Selbsttests dient dazu.

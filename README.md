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
espup install --toolchain-version 1.95.0.0
. ~/export-esp.sh
```
Bauen und flashen (Board per USB-C; BOOT halten und anstecken, falls nötig):
```bash
cd firmware
cargo run --release
```
Der Selbsttest zeigt ein Gesicht und gibt Berührungen mit den Rohbytes über die serielle Konsole aus.

Hinweis: Die Toolchain `1.99.0.0` linkt mit `esp-hal 1.0` nicht (`l32r`-Fehler); `1.95.0.0` funktioniert.

## Status
Die Firmware **baut** (`cargo build --release` mit Toolchain 1.95.0.0). Die Treiberdetails (CO5300-Init, CST9217-Berichtsformat, Pins des Modells 1.75C) sind **nicht am Gerät geprüft**. Beim ersten Flashen können Anpassungen nötig sein; die Rohausgabe des Selbsttests dient dazu.

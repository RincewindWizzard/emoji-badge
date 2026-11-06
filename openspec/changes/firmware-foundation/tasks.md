## 1. Projektgerüst
- [ ] 1.1 Xtensa-Toolchain dokumentieren (espup, espflash) und `firmware/`-Crate anlegen
- [x] 1.2 Pinbelegung als Modul `board` mit Werten aus `board-pinout`
- [ ] 1.3 CI-Job für den Firmware-Build ergänzen

## 2. Display
- [ ] 2.1 QSPI-Bus und CO5300-Initialisierung (Reset GPIO39)
- [ ] 2.2 Zeilenweise Ausgabe aus `badge-core`
- [ ] 2.3 Helligkeit per Panelbefehl

## 3. Touch und I2C
- [ ] 3.1 Geteilter I2C-Bus (GPIO14/15, 400 kHz)
- [ ] 3.2 CST9217 initialisieren (Reset GPIO40, INT GPIO11)
- [x] 3.3 Tippen und Wischen als Ereignisse

## 4. Verifikation am Gerät
- [ ] 4.1 Selbsttest (Farbtest, Touch-Ausgabe, I2C-Scan, Flash-Größe)
- [ ] 4.2 Pinbelegung des Modells 1.75C bestätigen und Specs korrigieren

## Hinweis zum Stand
Geschrieben und auf dem Host getestet: Pin-Modul `board`, Gestenerkennung (Tippen/Wischen), CO5300-Befehls- und Rahmenlogik, CST9217-Bericht-Parser (Format unverifiziert). `firmware/` (esp-hal-Anbindung, Selbsttest) ist geschrieben, aber nicht gebaut oder getestet, weil in der Entwicklungsumgebung keine Xtensa-Toolchain verfügbar ist. Die Aufgaben 1.1, 1.3, 2.x, 3.1, 3.2 und 4.x bleiben offen, bis sie am Gerät laufen.

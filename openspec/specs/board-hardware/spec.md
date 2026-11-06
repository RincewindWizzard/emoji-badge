# board-hardware Specification

## Purpose
Hardware-Referenz des Zielboards ESP32-S3-Touch-AMOLED-1.75C-EN (Waveshare-Typ). Die Firmware SHALL gegen diese Eckdaten entwickelt werden. Pinbelegungen sind hier bewusst nicht enthalten und müssen aus dem Schaltplan des Herstellers übernommen werden.

## Requirements

### Requirement: Mikrocontroller und Speicher
Das Board SHALL einen ESP32-S3R8 (Xtensa LX7, 32 Bit, Dual-Core, bis 240 MHz) verwenden. Der Chip hat 512 KB SRAM, 384 KB ROM und 8 MB PSRAM; zusätzlich ist externer Flash verbaut. **Offen:** Die Händlerangabe nennt 32 MB, Waveshare nennt für die Standardversion 16 MB; die tatsächliche Größe SHALL per `espflash board-info` geprüft werden.

#### Scenario: Speicherbudget der Firmware
- **WHEN** die Firmware Speicher plant (z. B. Framebuffer, Audiopuffer)
- **THEN** SHALL sie von 8 MB PSRAM ausgehen und zunächst nur 16 MB Flash annehmen
- **AND** ein vollständiger RGB565-Framebuffer (466×466×2 B ≈ 434 KiB) SHALL im PSRAM liegen, nicht im internen SRAM

### Requirement: Drahtlose Kommunikation
Das Board SHALL 2,4-GHz-WLAN (802.11 b/g/n) und Bluetooth 5 (LE) über eine integrierte Antenne bieten.

#### Scenario: Funkfähigkeiten
- **WHEN** die Firmware eine Netzwerkverbindung benötigt
- **THEN** SHALL nur 2,4-GHz-WLAN verwendet werden (kein 5 GHz)
- **AND** Bluetooth SHALL nur als LE verfügbar vorausgesetzt werden (kein Classic)

### Requirement: AMOLED-Display
Das Board SHALL ein rundes 1,75-Zoll-AMOLED-Display mit 466 × 466 Pixeln, 16,7 Millionen Farben, 700 cd/m² Helligkeit und 100000:1 Kontrast besitzen. Der Treiber-IC ist ein CO5300, angebunden über QSPI.

#### Scenario: Ansteuerung des Displays
- **WHEN** die Firmware Pixel ausgibt
- **THEN** SHALL sie den CO5300 über QSPI ansteuern
- **AND** Inhalte außerhalb des Kreises SHALL schwarz sein, da AMOLED-Pixel dann keinen Strom verbrauchen

### Requirement: Kapazitiver Touchscreen
Das Board SHALL einen kapazitiven Touch-Controller CST9217 besitzen, angebunden über I2C.

#### Scenario: Touch-Eingabe
- **WHEN** der Nutzer das Display berührt
- **THEN** SHALL die Firmware die Koordinaten über I2C vom CST9217 lesen

### Requirement: Audio
Das Board SHALL ein Dual-Mikrofon-Array mit Rauschunterdrückung und Echounterdrückung (Nah- und Fernfeld), einen Wiedergabe-Codec ES8311 mit Verstärker NS4150B, einen 4-Kanal-Aufnahme-ADC ES7210 (Mikrofone) und einen Lautsprecher-Anschluss (MX1.25) besitzen. Außerdem sind eine RTC (PCF85063), ein TCA9554-IO-Expander und ein TF-Kartenslot verbaut.

#### Scenario: Sprachaufnahme und Wiedergabe
- **WHEN** die Firmware Sprache aufnimmt oder wiedergibt
- **THEN** SHALL sie dafür den Audio-Codec des Boards verwenden
- **AND** die Echounterdrückung SHALL für gleichzeitige Wiedergabe und Aufnahme genutzt werden

### Requirement: Bewegungssensor
Das Board SHALL einen QMI8658 6-Achsen-IMU (3-Achsen-Beschleunigung, 3-Achsen-Gyroskop) besitzen.

#### Scenario: Bewegungserkennung
- **WHEN** die Firmware Bewegung, Gesten oder Schritte auswerten will
- **THEN** SHALL sie den QMI8658 verwenden

### Requirement: Energieversorgung
Das Board SHALL einen AXP2101 als Power-Management-Chip mit mehreren konfigurierbaren Spannungsausgängen sowie Lade- und Batteriemanagement besitzen. Ein 3,7-V-Lithiumakku SHALL über eine MX1.25-Buchse angeschlossen werden können; der Betrieb ohne Netzteil ist möglich.

#### Scenario: Akkubetrieb
- **WHEN** das Board nur mit Akku läuft
- **THEN** SHALL die Firmware Ladezustand und Laden über den AXP2101 abfragen können

### Requirement: Schnittstellen und Bedienelemente
Das Board SHALL einen USB-Typ-C-Anschluss sowie zwei frei belegbare Seitentasten (PWR und BOOT) besitzen.

#### Scenario: Tasten
- **WHEN** die Firmware eigene Tastenfunktionen definiert
- **THEN** MAY PWR und BOOT frei belegt werden
- **AND** BOOT SHALL beim Einschalten weiterhin den Download-Modus auslösen können

### Requirement: Gehäuse und Lieferumfang
Das Board SHALL in einem CNC-gefertigten Aluminiumgehäuse geliefert werden. Der Lieferumfang umfasst das Hauptgerät (ESP32-S3-Touch-AMOLED-1.75C-EN), ein silbernes Polyester-Lanyard und optional einen Lithiumakku.

#### Scenario: Metallgehäuse
- **WHEN** WLAN-/BT-Reichweite bewertet wird
- **THEN** SHALL berücksichtigt werden, dass das Metallgehäuse die Antenne beeinflussen kann

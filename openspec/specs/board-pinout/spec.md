# board-pinout Specification

## Purpose
Pinbelegung, I2C-Adressen und Peripherie-Zuordnung des Boards. Quelle: offizielles Waveshare-Repository `ESP32-S3-Touch-AMOLED-1.75` (HARDWARE_REFERENCE.md, gegen Schaltplan und BSP geprüft). Das gekaufte Board heißt "1.75C"; es ist **nicht verifiziert**, dass es identisch zur Waveshare-Standardversion ist. Pins SHALL vor dem ersten Flash mit einem Hardwaretest bestätigt werden.

## Requirements

### Requirement: Display-Pins (CO5300, QSPI)
Die Firmware SHALL das Display mit folgender Belegung ansteuern: DATA0=GPIO4, DATA1=GPIO5, DATA2=GPIO6, DATA3=GPIO7, CS=GPIO12, PCLK=GPIO38, RST=GPIO39, TE=GPIO13.

#### Scenario: Helligkeit
- **WHEN** die Helligkeit geändert wird
- **THEN** SHALL dies per CO5300-Befehl geschehen, da es kein GPIO-gesteuertes Backlight gibt

### Requirement: Shared I2C-Bus
Alle I2C-Geräte SHALL an einem gemeinsamen Bus hängen: SCL=GPIO14, SDA=GPIO15 (400 kHz). Die 7-Bit-Adressen sind: ES8311 0x18, TCA9554 0x20, AXP2101 0x34, ES7210 0x40, PCF85063 0x51, CST9217 0x5A, QMI8658 0x6B.

#### Scenario: Ein Bus-Treiber
- **WHEN** mehrere Treiber I2C nutzen
- **THEN** SHALL der Bus einmal initialisiert und geteilt werden (z. B. mit `embedded-hal-bus`)

### Requirement: Touch-Pins
Der CST9217 SHALL über den Shared-I2C-Bus (0x5A) mit INT=GPIO11 und RST=GPIO40 betrieben werden. Display-Reset (GPIO39) und Touch-Reset (GPIO40) SHALL getrennt behandelt werden.

#### Scenario: Touch-Initialisierung
- **WHEN** das Board startet
- **THEN** SHALL zuerst der Reset-Pin getoggelt und danach der Controller über I2C initialisiert werden

### Requirement: IO-Expander TCA9554
Ein TCA9554 (0x20) SHALL folgende Signale liefern: EXIO3=RTC_INT, EXIO4=SYS_OUT (PWR-Taste, high = gedrückt), EXIO5=AXP_IRQ, EXIO6=QMI_INT1, EXIO7=GPS_RST (nur GPS-Variante); EXIO0-2 sind frei.

#### Scenario: PWR-Taste lesen
- **WHEN** die PWR-Taste ausgewertet wird
- **THEN** SHALL EXIO4 über den Expander gelesen werden, nicht über einen ESP32-GPIO

### Requirement: Audio-Pfad
Audio SHALL über I2S mit MCLK=GPIO42, BCLK=GPIO9, LRCK/WS=GPIO45, DOUT=GPIO8 (zu ES8311) und DIN=GPIO10 (von ES7210) laufen. Der Verstärker (NS4150B) wird über PA_CTRL=GPIO46 gesteuert. Die zwei Mikrofone hängen am ES7210 (MIC1, MIC2); MIC3 dient als Playback-Referenz für Echounterdrückung, MIC4 ist unbelegt.

#### Scenario: Echounterdrückung
- **WHEN** Sprache aufgenommen wird, während Audio spielt
- **THEN** SHALL die Referenz (MIC3) im TDM-Strom mitgelesen werden
- **AND** die Slot-Reihenfolge SHALL MIC1, MIC3, MIC2, MIC4 sein (nicht die physische Nummerierung)

### Requirement: Weitere Pins
- BOOT-Taste = GPIO0 (low = gedrückt)
- IMU-Interrupt INT2 = GPIO21
- TF-Karte (SDMMC, 1-Bit): CMD=GPIO1, CLK=GPIO2, D0=GPIO3, D3/CS=GPIO41
- Native USB: D-=GPIO19, D+=GPIO20 (SHALL NOT anderweitig genutzt werden)
- Erweiterungsleiste: GPIO16, GPIO17, GPIO18, UART0 TX=GPIO43, RX=GPIO44

#### Scenario: Reservierte Pins
- **WHEN** externe Peripherie angeschlossen wird
- **THEN** SHALL nur GPIO16/17/18 als frei nutzbar gelten

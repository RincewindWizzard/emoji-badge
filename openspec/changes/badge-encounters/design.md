## Context
ESP32-S3 hat BLE 5 (kein Classic). Das Aluminiumgehäuse und der Körper dämpfen das Signal, Bluetooth-Adressen wechseln zufällig.

## Goals / Non-Goals
- Goals: robuste Nähe-Erkennung, verständliche Stufen, wenig Strom.
- Non-Goals: Austausch von Kontaktdaten, Gruppen/Namen (spätere Erweiterung).

## Decisions
- **BLE-Advertising** mit eigenem Paket (Magic-Byte, Version, Badge-ID) und gleichzeitigem Scannen; ESP-NOW verworfen, da nur zwischen ESP32 und für die Messereichweite unnötig.
- **Eigene Badge-ID im Paket**, weil Bluetooth-Adressen rotieren.
- **Nähe über geglättete RSSI** (Mittel über mehrere Pakete) mit konfigurierbarem Schwellwert; Werte am Gerät kalibrieren.
- **Stufen als Intensität 1–5**; Animationsparameter (Herzenzahl, Größe, Ausdruck) sind Funktionen der Stufe, nicht eigene Animationen.
- **Gedächtnis**: Tabelle (ID, Stufe, letzter Treffer, Zähler), LRU, Zeit aus der RTC; Abklingen beim Lesen berechnet statt Timer.
- **Duty-Cycling** von Senden/Scannen zum Stromsparen.

## Risks / Trade-offs
- [Schwellwert stimmt je Badge/Trageweise nicht] → Konfiguration und Kalibrierung am Gerät.
- [Viele Badges auf engem Raum] → Pause und Tabellengröße begrenzen.
- [Flash-Verschleiß] → Schreiben nur bei Änderungen, gebündelt.
- [Zeit nach Stromverlust] → RTC wird über den Akku gepuffert.

## Open Questions
- Ob Badges später Gruppen/Namen austauschen sollen.

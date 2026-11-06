## Why
Emotionen sollen optional per gesprochenem Wort wechseln. Wichtig: nie Dauerlauschen und keine Internetverbindung; die Bedienung per Wischen bleibt vorrangig.

## What Changes
- Mikrofon nur aktiv, solange eine Taste gehalten wird (Push-to-Talk).
- Erkennung offline auf dem Gerät, keine Daten verlassen das Gerät.
- Wenige Befehlswörter ("toll", "sauer" o. Ä.), die auf Emotionen abgebildet werden.
- Start mit englischen Befehlen (ESP-SR MultiNet), Deutsch nur per eigenem Modell.

## Capabilities
### New Capabilities
- `voice-commands`: Push-to-Talk-Erkennung weniger Wörter ohne Internet.

### Modified Capabilities

## Impact
Mikrofone (ES7210), Audio-Pfad; ESP-SR ist eine C-Bibliothek und kann die Wahl `esp-idf-svc` statt `esp-hal` erzwingen. Niedrige Priorität, später.

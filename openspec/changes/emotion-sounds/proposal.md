## Why
Zu jeder Emotion soll eine kurze Melodie die Stimmung untermalen, ausgelöst per Knopfdruck. Auf einer Messe darf nie ungewollt Ton entstehen.

## What Changes
- Töne werden aus Notenlisten erzeugt (kein Audio-Dateiformat nötig).
- Jede Emotion bekommt ein Jingle: Trauer/Beerdigung = Auszug aus Chopins Trauermarsch (gemeinfrei), Freude = Tusch, fröhlich = trällernde Tonfolge; weitere eigene Melodien.
- BOOT-Taste spielt das Jingle der aktuellen Emotion einmal; PWR bleibt für Ein/Aus.
- Lautstärke begrenzt, Stummschaltung möglich.

## Capabilities
### New Capabilities
- `emotion-sounds`: Melodien zu Emotionen, Wiedergabe per Taste.

### Modified Capabilities

## Impact
Audio-Pfad (I2S, ES8311, Verstärker GPIO46). Abhängig von `firmware-foundation`; Audio-Hardware ist ungetestet.

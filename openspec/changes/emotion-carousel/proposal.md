## Why
Kern der Badge sind einfache Emoji-Gesichter mit Grundemotionen, die man per Wisch wie in einem Karussell durchschaltet. Jede Emotion soll beim Erscheinen kurz reagieren und danach im Ausdruck verharren.

## What Changes
- Grundemotionen als Gesichter (Startmenge: fröhlich, traurig, überrascht, wütend, verliebt, müde, neutral).
- Beim Wechsel spielt eine Emotion eine kurze Eintrittsanimation und hält dann ihren Ausdruck.
- Antippen spielt die Eintrittsanimation erneut ab.
- Wischen von links nach rechts zeigt die nächste Emotion (rechts nach links: die vorherige), im Kreis (Karussell, Endlosschleife).
- Schütteln wählt eine zufällige Emotion, Neigen lässt die Figur in Neigungsrichtung blicken (IMU QMI8658).
- Bedienung ausschließlich per Berührung: Wischen und Tippen.

## Capabilities
### New Capabilities
- `emotion-display`: Emotionen, Eintrittsanimation, Halten, erneutes Abspielen.
- `carousel-navigation`: Wisch-Navigation im Kreis.
- `motion-reactions`: Reaktionen auf Schütteln und Neigung.

### Modified Capabilities

## Impact
Erweitert `badge-core` um Animationszustände; Firmware bindet Touch und IMU an. Abhängig von `firmware-foundation`.

//! Erkennung von Tippen und Wischen aus Berührungsproben. Rein logisch, ohne Hardware.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TouchSample {
    pub t_ms: u32,
    pub x: u16,
    pub y: u16,
    pub pressed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gesture {
    Tap { x: u16, y: u16 },
    /// Wischen von links nach rechts (nächste Emotion).
    SwipeRight,
    /// Wischen von rechts nach links (vorherige Emotion).
    SwipeLeft,
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub swipe_min_dx: i32,
    pub swipe_max_ms: u32,
    pub tap_max_move: i32,
    pub tap_max_ms: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self { swipe_min_dx: 80, swipe_max_ms: 900, tap_max_move: 20, tap_max_ms: 400 }
    }
}

pub struct Recognizer {
    cfg: Config,
    start: Option<TouchSample>,
    last: Option<TouchSample>,
}


impl Recognizer {
    pub fn new(cfg: Config) -> Self {
        Self { cfg, start: None, last: None }
    }

    /// Eine Probe einspeisen; bei Loslassen kommt gegebenenfalls eine Geste zurück.
    pub fn feed(&mut self, s: TouchSample) -> Option<Gesture> {
        if s.pressed {
            if self.start.is_none() {
                self.start = Some(s);
            }
            self.last = Some(s);
            return None;
        }
        let (a, b) = (self.start.take()?, self.last.take()?);
        let dx = b.x as i32 - a.x as i32;
        let dy = b.y as i32 - a.y as i32;
        let dt = b.t_ms.saturating_sub(a.t_ms);
        let c = &self.cfg;
        if dx.abs() >= c.swipe_min_dx && dx.abs() > dy.abs() && dt <= c.swipe_max_ms {
            return Some(if dx > 0 { Gesture::SwipeRight } else { Gesture::SwipeLeft });
        }
        if dx.abs() <= c.tap_max_move && dy.abs() <= c.tap_max_move && dt <= c.tap_max_ms {
            return Some(Gesture::Tap { x: a.x, y: a.y });
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(points: &[(u32, u16, u16)]) -> Option<Gesture> {
        let mut r = Recognizer::new(Config::default());
        for (t, x, y) in points {
            assert_eq!(r.feed(TouchSample { t_ms: *t, x: *x, y: *y, pressed: true }), None);
        }
        let (t, x, y) = *points.last().unwrap();
        r.feed(TouchSample { t_ms: t + 10, x, y, pressed: false })
    }

    #[test]
    fn left_to_right_is_swipe_right() {
        assert_eq!(run(&[(0, 100, 230), (100, 200, 235), (200, 320, 240)]), Some(Gesture::SwipeRight));
    }

    #[test]
    fn right_to_left_is_swipe_left() {
        assert_eq!(run(&[(0, 350, 230), (150, 200, 232), (250, 100, 234)]), Some(Gesture::SwipeLeft));
    }

    #[test]
    fn short_touch_is_tap() {
        assert_eq!(run(&[(0, 230, 230), (80, 232, 231)]), Some(Gesture::Tap { x: 230, y: 230 }));
    }

    #[test]
    fn vertical_move_is_nothing() {
        assert_eq!(run(&[(0, 230, 100), (150, 232, 300)]), None);
    }

    #[test]
    fn slow_swipe_is_ignored() {
        assert_eq!(run(&[(0, 100, 230), (2000, 320, 230)]), None);
    }

    #[test]
    fn release_without_press_is_ignored() {
        let mut r = Recognizer::new(Config::default());
        assert_eq!(r.feed(TouchSample { t_ms: 0, x: 0, y: 0, pressed: false }), None);
    }
}

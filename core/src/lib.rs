//! Zeilenweiser Software-Renderer für Emoji-Gesichter auf einem runden 466×466-Display.
//! Ausgabe: RGB565. Kein Framebuffer nötig (466*466*2 B ≈ 434 KiB), nur ein Zeilenpuffer.
#![no_std]

pub const WIDTH: usize = 466;
pub const HEIGHT: usize = 466;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Face {
    Happy,
    Sad,
    Surprised,
    Neutral,
}

impl Face {
    pub const ALL: [Face; 4] = [Face::Happy, Face::Sad, Face::Surprised, Face::Neutral];
    pub fn next(self) -> Face {
        let i = Self::ALL.iter().position(|f| *f == self).unwrap_or(0);
        Self::ALL[(i + 1) % Self::ALL.len()]
    }
}

const fn rgb565(r: u8, g: u8, b: u8) -> u16 {
    ((r as u16 & 0xF8) << 8) | ((g as u16 & 0xFC) << 3) | (b as u16 >> 3)
}
pub const BLACK: u16 = rgb565(0, 0, 0);
pub const YELLOW: u16 = rgb565(255, 204, 0);
pub const DARK: u16 = rgb565(60, 40, 0);

const C: i32 = (WIDTH / 2) as i32;
const R: i32 = C - 2;

fn d2(x: i32, y: i32, cx: i32, cy: i32) -> i32 {
    let (dx, dy) = (x - cx, y - cy);
    dx * dx + dy * dy
}

fn pixel(face: Face, x: i32, y: i32) -> u16 {
    if d2(x, y, C, C) > R * R {
        return BLACK;
    }
    let eye_dx = 90;
    let eye_y = C - 70;
    let eye_r = if face == Face::Surprised { 38 } else { 26 };
    let in_eye = d2(x, y, C - eye_dx, eye_y) <= eye_r * eye_r
        || d2(x, y, C + eye_dx, eye_y) <= eye_r * eye_r;
    if in_eye {
        return DARK;
    }
    let my = C + 80;
    let mouth = match face {
        // Lächeln: Ring-Segment unterhalb der Mundmitte
        Face::Happy => {
            let d = d2(x, y, C, my - 40);
            d <= 110 * 110 && d >= 90 * 90 && y > my - 40
        }
        // Traurig: Ring-Segment oberhalb
        Face::Sad => {
            let d = d2(x, y, C, my + 70);
            d <= 110 * 110 && d >= 90 * 90 && y < my + 70
        }
        Face::Surprised => d2(x, y, C, my) <= 35 * 35,
        Face::Neutral => (y - my).abs() <= 8 && (x - C).abs() <= 80,
    };
    if mouth { DARK } else { YELLOW }
}

/// Rendert Zeile `y` des Gesichts in `out` (Länge >= WIDTH).
pub fn render_line(face: Face, y: usize, out: &mut [u16]) {
    for (x, px) in out.iter_mut().take(WIDTH).enumerate() {
        *px = pixel(face, x as i32, y as i32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(face: Face, y: usize) -> [u16; WIDTH] {
        let mut b = [0u16; WIDTH];
        render_line(face, y, &mut b);
        b
    }

    #[test]
    fn corners_are_black_center_is_yellow() {
        assert_eq!(line(Face::Happy, 0)[0], BLACK);
        assert_eq!(line(Face::Happy, C as usize)[C as usize], YELLOW);
    }

    #[test]
    fn faces_differ() {
        let mut seen = [[0u16; WIDTH]; 4];
        for (i, f) in Face::ALL.iter().enumerate() {
            seen[i] = line(*f, (C + 80) as usize);
        }
        assert_ne!(seen[0], seen[1]);
        assert_ne!(seen[2], seen[3]);
    }

    #[test]
    fn next_cycles() {
        let mut f = Face::Happy;
        for _ in 0..4 { f = f.next(); }
        assert_eq!(f, Face::Happy);
    }
}

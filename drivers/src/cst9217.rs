//! CST9217-Touch (I2C 0x5A). Registeradresse und Berichtsformat sind aus dem Gedächtnis
//! (Referenz: esp_lcd_touch_cst9217) und UNVERIFIZIERT. Der Selbsttest gibt die Rohbytes aus,
//! damit `parse` am Gerät geprüft werden kann (Task 4.1).
use embedded_hal::i2c::I2c;

pub const ADDR: u8 = 0x5A;
/// Register mit dem Touch-Bericht (16-Bit-Adresse, Big-Endian).
pub const REG_REPORT: [u8; 2] = [0xD0, 0x00];
pub const REPORT_LEN: usize = 8;

/// Wertet einen Bericht aus. Rückgabe: Position, wenn mindestens ein Finger anliegt.
pub fn parse(frame: &[u8; REPORT_LEN]) -> Option<(u16, u16)> {
    let points = frame[5] & 0x7F;
    let down = frame[0] == 0x06;
    if points == 0 || !down {
        return None;
    }
    let x = ((frame[1] as u16) << 4) | ((frame[3] as u16) >> 4);
    let y = ((frame[2] as u16) << 4) | ((frame[3] as u16) & 0x0F);
    Some((x, y))
}

pub struct Cst9217<I> {
    i2c: I,
}

impl<I: I2c> Cst9217<I> {
    pub fn new(i2c: I) -> Self {
        Self { i2c }
    }

    /// Liest den Rohbericht (auch für den Selbsttest).
    pub fn read_raw(&mut self) -> Result<[u8; REPORT_LEN], I::Error> {
        let mut buf = [0u8; REPORT_LEN];
        self.i2c.write_read(ADDR, &REG_REPORT, &mut buf)?;
        Ok(buf)
    }

    pub fn read_position(&mut self) -> Result<Option<(u16, u16)>, I::Error> {
        Ok(parse(&self.read_raw()?))
    }

    pub fn release(self) -> I {
        self.i2c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_finger_yields_none() {
        assert_eq!(parse(&[0; REPORT_LEN]), None);
    }

    #[test]
    fn finger_is_decoded() {
        // x = 0x1D1 (465) -> hi 0x1D, lo-Nibble 0x1; y = 0x0E6 (230) -> hi 0x0E, lo 0x6
        let f = [0x06, 0x1D, 0x0E, 0x16, 0, 1, 0, 0];
        assert_eq!(parse(&f), Some((0x1D1, 0x0E6)));
    }
}

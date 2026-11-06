//! CO5300-AMOLED-Treiber (QSPI). Initialisierungssequenz und Rahmenformat sind aus dem Gedächtnis
//! nach den Referenzen von Arduino_GFX/Espressif `esp_lcd_co5300` übernommen und müssen am Gerät
//! verifiziert werden (siehe Task 4.1).
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

pub const WIDTH: u16 = 466;
pub const HEIGHT: u16 = 466;
/// Das Panel hat 6 Spalten Versatz (466 von 480 Spalten genutzt).
pub const COL_OFFSET: u16 = 6;

/// Bus mit 1 Leitung für Befehle und wahlweise 4 Leitungen für Daten.
pub trait QspiBus {
    type Error;
    /// `opcode` 0x02 = Daten einzeilig, 0x32 = Daten über 4 Leitungen; `addr` = 24 Bit.
    fn transfer(&mut self, opcode: u8, addr: u32, data: &[u8], quad_data: bool) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub enum Error<B, P> {
    Bus(B),
    Pin(P),
}

pub struct Co5300<B, RST, D> {
    bus: B,
    rst: RST,
    delay: D,
}

impl<B: QspiBus, RST: OutputPin, D: DelayNs> Co5300<B, RST, D> {
    pub fn new(bus: B, rst: RST, delay: D) -> Self {
        Self { bus, rst, delay }
    }

    fn cmd(&mut self, c: u8, params: &[u8]) -> Result<(), Error<B::Error, RST::Error>> {
        self.bus.transfer(0x02, (c as u32) << 8, params, false).map_err(Error::Bus)
    }

    pub fn init(&mut self) -> Result<(), Error<B::Error, RST::Error>> {
        self.rst.set_high().map_err(Error::Pin)?;
        self.delay.delay_ms(10);
        self.rst.set_low().map_err(Error::Pin)?;
        self.delay.delay_ms(10);
        self.rst.set_high().map_err(Error::Pin)?;
        self.delay.delay_ms(120);

        self.cmd(0x11, &[])?; // Sleep Out
        self.delay.delay_ms(120);
        self.cmd(0xFE, &[0x00])?; // Seite 0
        self.cmd(0xC4, &[0x80])?; // SPI-Modus
        self.cmd(0x3A, &[0x55])?; // 16 Bit RGB565
        self.cmd(0x35, &[0x00])?; // Tearing-Effect an
        self.cmd(0x53, &[0x20])?; // Helligkeitssteuerung an
        self.cmd(0x63, &[0xFF])?;
        self.cmd(0x29, &[])?; // Display On
        self.delay.delay_ms(10);
        self.set_brightness(0xFF)
    }

    pub fn set_brightness(&mut self, level: u8) -> Result<(), Error<B::Error, RST::Error>> {
        self.cmd(0x51, &[level])
    }

    pub fn set_window(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) -> Result<(), Error<B::Error, RST::Error>> {
        let (xa, xb) = (x0 + COL_OFFSET, x1 + COL_OFFSET);
        self.cmd(0x2A, &[(xa >> 8) as u8, xa as u8, (xb >> 8) as u8, xb as u8])?;
        self.cmd(0x2B, &[(y0 >> 8) as u8, y0 as u8, (y1 >> 8) as u8, y1 as u8])
    }

    /// Sendet RGB565-Pixel (Big-Endian im Bus). `first` = true beginnt einen neuen Schreibvorgang (0x2C),
    /// sonst wird fortgesetzt (0x3C).
    pub fn write_pixels(&mut self, first: bool, pixels: &[u16], scratch: &mut [u8]) -> Result<(), Error<B::Error, RST::Error>> {
        let mut first = first;
        for chunk in pixels.chunks(scratch.len() / 2) {
            for (i, p) in chunk.iter().enumerate() {
                scratch[2 * i] = (p >> 8) as u8;
                scratch[2 * i + 1] = *p as u8;
            }
            let c = if first { 0x2C } else { 0x3C };
            first = false;
            self.bus
                .transfer(0x32, (c as u32) << 8, &scratch[..chunk.len() * 2], true)
                .map_err(Error::Bus)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use core::convert::Infallible;
    use std::vec::Vec;

    #[derive(Default)]
    struct Rec(Vec<(u8, u32, Vec<u8>, bool)>);
    impl QspiBus for Rec {
        type Error = Infallible;
        fn transfer(&mut self, o: u8, a: u32, d: &[u8], q: bool) -> Result<(), Infallible> {
            self.0.push((o, a, d.to_vec(), q));
            Ok(())
        }
    }
    struct Pin;
    impl embedded_hal::digital::ErrorType for Pin { type Error = Infallible; }
    impl OutputPin for Pin {
        fn set_low(&mut self) -> Result<(), Infallible> { Ok(()) }
        fn set_high(&mut self) -> Result<(), Infallible> { Ok(()) }
    }
    struct Nop;
    impl DelayNs for Nop { fn delay_ns(&mut self, _: u32) {} }

    fn drv() -> Co5300<Rec, Pin, Nop> { Co5300::new(Rec::default(), Pin, Nop) }

    #[test]
    fn init_starts_with_sleep_out_and_ends_with_brightness() {
        let mut d = drv();
        d.init().unwrap();
        let log = &d.bus.0;
        assert_eq!(log[0].1, 0x1100);
        assert!(log.iter().any(|e| e.1 == 0x2900));
        let last = log.last().unwrap();
        assert_eq!((last.1, last.2.as_slice()), (0x5100, &[0xFFu8][..]));
    }

    #[test]
    fn window_applies_column_offset() {
        let mut d = drv();
        d.set_window(0, 0, 465, 465).unwrap();
        let col = &d.bus.0[0];
        assert_eq!(col.1, 0x2A00);
        assert_eq!(col.2, [0, 6, 1, 0xD7]); // 6 .. 471
        assert_eq!(d.bus.0[1].2, [0, 0, 1, 0xD1]); // 0 .. 465
    }

    #[test]
    fn pixels_are_big_endian_and_continue_with_3c() {
        let mut d = drv();
        let mut scratch = [0u8; 4]; // 2 Pixel pro Block
        d.write_pixels(true, &[0x1234, 0xABCD, 0x0001], &mut scratch).unwrap();
        let log = &d.bus.0;
        assert_eq!(log.len(), 2);
        assert_eq!((log[0].0, log[0].1, log[0].3), (0x32, 0x2C00, true));
        assert_eq!(log[0].2, [0x12, 0x34, 0xAB, 0xCD]);
        assert_eq!(log[1].1, 0x3C00);
        assert_eq!(log[1].2, [0x00, 0x01]);
    }
}

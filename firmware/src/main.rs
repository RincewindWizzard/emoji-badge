//! Einstieg der Badge-Firmware. NICHT auf Hardware getestet und in der Entwicklungsumgebung nicht
//! gebaut (keine Xtensa-Toolchain). Der esp-hal-Teil muss beim ersten Bau gegen die installierte
//! esp-hal-Version angepasst werden (Task 4.1).
#![no_std]
#![no_main]

use badge_core::{render_line, Face, HEIGHT, WIDTH};
use badge_drivers::{
    co5300::{Co5300, QspiBus},
    cst9217::Cst9217,
    gesture::{Config, Gesture, Recognizer, TouchSample},
};
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    i2c::master::{Config as I2cConfig, I2c},
    main,
    spi::{
        master::{Address, Command, Config as SpiConfig, DataMode, Spi},
        Mode,
    },
    time::{Instant, Rate},
};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

/// QSPI-Anbindung des Displays an `badge-drivers`.
struct Qspi<'d>(Spi<'d, esp_hal::Blocking>);

impl QspiBus for Qspi<'_> {
    type Error = esp_hal::spi::Error;
    fn transfer(&mut self, opcode: u8, addr: u32, data: &[u8], quad: bool) -> Result<(), Self::Error> {
        let mode = if quad { DataMode::Quad } else { DataMode::Single };
        self.0.half_duplex_write(
            mode,
            Command::_8Bit(opcode as u16, DataMode::Single),
            Address::_24Bit(addr, DataMode::Single),
            0,
            data,
        )
    }
}

#[main]
fn main() -> ! {
    let p = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Pins: siehe badge_drivers::board (GPIO4-7 Daten, 12 CS, 38 CLK, 39 RST).
    let spi = Spi::new(p.SPI2, SpiConfig::default().with_frequency(Rate::from_mhz(40)).with_mode(Mode::_0))
        .unwrap()
        .with_sck(p.GPIO38)
        .with_cs(p.GPIO12)
        .with_sio0(p.GPIO4)
        .with_sio1(p.GPIO5)
        .with_sio2(p.GPIO6)
        .with_sio3(p.GPIO7);
    let rst = Output::new(p.GPIO39, Level::High, OutputConfig::default());
    let mut lcd = Co5300::new(Qspi(spi), rst, delay);
    lcd.init().unwrap();

    // Touch-Reset und I2C
    let mut tp_rst = Output::new(p.GPIO40, Level::High, OutputConfig::default());
    tp_rst.set_low();
    delay.delay_millis(10);
    tp_rst.set_high();
    delay.delay_millis(50);
    let i2c = I2c::new(p.I2C0, I2cConfig::default().with_frequency(Rate::from_khz(400)))
        .unwrap()
        .with_sda(p.GPIO15)
        .with_scl(p.GPIO14);
    let mut touch = Cst9217::new(i2c);

    // Selbsttest: Farbbalken, damit man Pins und Farbreihenfolge sofort sieht.
    lcd.set_window(0, 0, WIDTH as u16 - 1, HEIGHT as u16 - 1).unwrap();
    let mut line = [0u16; WIDTH];
    let mut scratch = [0u8; 2 * WIDTH];
    for y in 0..HEIGHT {
        render_line(Face::Happy, y, &mut line);
        lcd.write_pixels(y == 0, &line, &mut scratch).unwrap();
    }
    println!("Selbsttest: Gesicht gezeichnet, warte auf Touch");

    let mut rec = Recognizer::new(Config::default());
    loop {
        let t = Instant::now().duration_since_epoch().as_millis() as u32;
        match touch.read_raw() {
            Ok(raw) => {
                let pos = badge_drivers::cst9217::parse(&raw);
                if pos.is_some() {
                    println!("touch raw={:?}", raw); // zur Prüfung des Berichtsformats
                }
                let s = match pos {
                    Some((x, y)) => TouchSample { t_ms: t, x, y, pressed: true },
                    None => TouchSample { t_ms: t, x: 0, y: 0, pressed: false },
                };
                match rec.feed(s) {
                    Some(Gesture::Tap { x, y }) => println!("Tap {x},{y}"),
                    Some(Gesture::SwipeRight) => println!("Wisch links->rechts (nächste)"),
                    Some(Gesture::SwipeLeft) => println!("Wisch rechts->links (vorherige)"),
                    None => {}
                }
            }
            Err(e) => println!("Touch nicht erreichbar: {:?}", e),
        }
        delay.delay_millis(10);
    }
}

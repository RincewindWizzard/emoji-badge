//! Pinbelegung laut Waveshare-Referenz für ESP32-S3-Touch-AMOLED-1.75.
//! UNVERIFIZIERT für das gekaufte Modell "1.75C" – bei Abweichung nur hier ändern.

pub const LCD_D0: u8 = 4;
pub const LCD_D1: u8 = 5;
pub const LCD_D2: u8 = 6;
pub const LCD_D3: u8 = 7;
pub const LCD_CS: u8 = 12;
pub const LCD_TE: u8 = 13;
pub const LCD_PCLK: u8 = 38;
pub const LCD_RST: u8 = 39;

pub const I2C_SCL: u8 = 14;
pub const I2C_SDA: u8 = 15;

pub const TP_INT: u8 = 11;
pub const TP_RST: u8 = 40;

pub const BOOT_BUTTON: u8 = 0;

pub const I2C_ADDR_TCA9554: u8 = 0x20;
pub const I2C_ADDR_AXP2101: u8 = 0x34;
pub const I2C_ADDR_ES7210: u8 = 0x40;
pub const I2C_ADDR_PCF85063: u8 = 0x51;
pub const I2C_ADDR_CST9217: u8 = 0x5A;
pub const I2C_ADDR_QMI8658: u8 = 0x6B;
pub const I2C_ADDR_ES8311: u8 = 0x18;

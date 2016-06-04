//! GPIO interface for RPi Model B

use core::intrinsics::{volatile_load, volatile_store};

const GPIO_BASE: u32 = 0x20200000;

const FUNCTION_SELECT_BASE: *const u32 = GPIO_BASE as *const u32;
const OUTPUT_CLEAR_BASE: *const u32 = (GPIO_BASE + 0x1C) as *const u32;
const OUTPUT_SET_BASE: *const u32 = (GPIO_BASE + 0x28) as *const u32;

#[repr(C)]
pub enum MODE {
    INPUT = 0b000,
    OUTPUT = 0b001,
    FUNCTION_0 = 0b100,
    FUNCTION_1 = 0b101,
    FUNCTION_2 = 0b110,
    FUNCTION_3 = 0b111,
    FUNCTION_4 = 0b011,
    FUNCTION_5 = 0b010,
}

#[repr(C)]
pub enum PIN {
    SDA0 = 0,
    SCL0,
    SDA1,
    SCL1,
    GPIO_GCLK,
    CAM_CLK,
    LAN_RUN,
    SPI_CE1_N,
    SPI_CE0_N,
    SPI_MISO,
    SPI_MOSI,
    SPI_SCLK,
    TXD0 = 14,
    RXD0,
    STATUS_LED_N,
    GPIO_GEN0,
    GPIO_GEN1,
    GPIO_GEN2 = 21,
    GPIO_GEN3,
    GPIO_GEN4,
    GPIO_GEN5,
    GPIO_GEN6,
    CAM_GPIO,
    CONFIG0,
    CONFIG1,
    CONFIG2,
    CONFIG3,
    PWM0_OUT = 40,
    PWM1_OUT = 45,
    HDMI_HPD_P,
    SD_CARD_DET,
    SD_CLK_R,
    SD_CMD_R,
    SD_DATA0_R,
    SD_DATA1_R,
    SD_DATA2_R,
    SD_DATA3_R,
}

#[inline(always)]
pub fn set_mode(pin: PIN, mode: MODE) {
    let index = pin as u32;
    let col = index / 10;
    let row = index % 10;
    let pins_col = unsafe { FUNCTION_SELECT_BASE.offset(col as isize) as *mut u32 };

    unsafe {
        volatile_store(pins_col, (mode as u32) << (row * 3));
    }
}

#[inline(always)]
pub fn set_pin(pin: PIN) {
    let index = pin as u32;
    let col = index / 32;
    let row = index % 32;
    let pins_col = unsafe { OUTPUT_SET_BASE.offset(col as isize) as *mut u32 };

    unsafe {
        volatile_store(pins_col, 1 << row);
    }
}

pub fn get_pin(pin: PIN) -> bool {
    let index = pin as u32;
    let col = index / 32;
    let row = index % 32;
    let pins_col = unsafe { OUTPUT_SET_BASE.offset(col as isize) as *mut u32 };

    let word = unsafe { volatile_load(pins_col) };

    (word & (1 << row)) != 0
}

#[inline(always)]
pub fn clear_pin(pin: PIN) {
    let index = pin as u32;
    let col = index / 32;
    let row = index % 32;
    let pins_col = unsafe { OUTPUT_CLEAR_BASE.offset(col as isize) as *mut u32 };

    unsafe {
        volatile_store(pins_col, 1 << row);
    }
}

#[inline(always)]
pub fn led_on() {
    set_pin(PIN::STATUS_LED_N);
}

#[inline(always)]
pub fn led_off() {
    clear_pin(PIN::STATUS_LED_N);
}

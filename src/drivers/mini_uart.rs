use core::ptr;

use crate::drivers::{
    MMIO_BASE,
    gpio::{GPIOPinFunc, Gpio},
};

const AUX_BASE: usize = MMIO_BASE + 0x21_5000;

const AUX_ENABLES: *mut u32 = (AUX_BASE + 0x04) as *mut u32;
const AUX_MU_IO: *mut u32 = (AUX_BASE + 0x40) as *mut u32;
const AUX_MU_IER: *mut u32 = (AUX_BASE + 0x44) as *mut u32;
const AUX_MU_LCR: *mut u32 = (AUX_BASE + 0x4c) as *mut u32;
const AUX_MU_MCR: *mut u32 = (AUX_BASE + 0x50) as *mut u32;
const AUX_MU_LSR: *mut u32 = (AUX_BASE + 0x54) as *mut u32;
const AUX_MU_CNTL: *mut u32 = (AUX_BASE + 0x60) as *mut u32;
const AUX_MU_BAUD: *mut u32 = (AUX_BASE + 0x68) as *mut u32;

const TXD_PIN: u8 = 14;
const RXD_PIN: u8 = 15;

pub struct MiniUart {
    gpio: Gpio,
}

impl MiniUart {
    pub fn new() -> Self {
        // configure gpio pins to use mini uart
        let gpio = Gpio;

        gpio.set_pin_func(TXD_PIN, GPIOPinFunc::Alt5);
        gpio.set_pin_func(RXD_PIN, GPIOPinFunc::Alt5);

        gpio.enable_pin(TXD_PIN);
        gpio.enable_pin(RXD_PIN);

        unsafe {
            ptr::write_volatile(AUX_ENABLES, 1); // enable mini uart peripheral
            ptr::write_volatile(AUX_MU_CNTL, 0); // disable txd and rxd
            ptr::write_volatile(AUX_MU_IER, 0); // disable interrupts
            ptr::write_volatile(AUX_MU_LCR, 1); // set data size to 8-bit mode
            ptr::write_volatile(AUX_MU_MCR, 3); // set modem control to normal
            ptr::write_volatile(AUX_MU_BAUD, 541); // set baud rate (115200 baud)
            ptr::write_volatile(AUX_MU_CNTL, 0b11); // re-enable txd and rxd
        }

        MiniUart { gpio }
    }

    pub fn send_char(&self, c: char) {
        unsafe {
            // wait until fifo buffer is ready to receive data
            while (ptr::read_volatile(AUX_MU_LSR) & 0x20) == 0 {}

            // send char
            ptr::write_volatile(AUX_MU_IO, c as u32);
        }
    }

    pub fn send_str(&self, s: &str) {
        for c in s.chars() {
            // send \r if sending \n to fix new line start
            if c == '\n' {
                self.send_char('\r');
            }

            self.send_char(c);
        }
    }

    pub fn recv() -> char {
        unsafe {
            // wait until there is a byte in the fifo
            while (core::ptr::read_volatile(AUX_MU_LSR) & 1) == 0 {}

            // read, convert and return the byte to char
            (core::ptr::read_volatile(AUX_MU_IO) & 0xff) as u8 as char
        }
    }
}

use crate::drivers::MMIO_BASE;
use core::ptr;

const GPIO_BASE: usize = MMIO_BASE + 0x20_0000;
const GPIO_PUP_PDN0: *mut u32 = (GPIO_BASE + 0xE4) as *mut u32;
const GPFSEL0: *mut u32 = GPIO_BASE as *mut u32;

pub enum GPIOPinFunc {
    Input = 0b000,
    Output = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010,
}

pub struct Gpio;

impl Gpio {
    pub fn enable_pin(&self, pin_num: u8) {
        // return if pin_num is invalid
        if pin_num > 57 {
            return;
        }

        let reg_num = (pin_num / 16) as usize; // calculate pin registry number
        let bit_start = (pin_num % 16) * 2; // calculate pin bits start position

        unsafe {
            let reg_addr = GPIO_PUP_PDN0.add(reg_num * 4); // calculate pin registry address
            let mut reg_val = ptr::read_volatile(reg_addr); // read entire pin GPIO_PUP_PDN address

            reg_val &= !(0b11 << bit_start); // clear pin PUP_PDN bits

            ptr::write_volatile(reg_addr, reg_val); // write new PUP_PDN address value
        }
    }

    pub fn set_pin_func(&self, pin_num: u8, func: GPIOPinFunc) {
        // return if pin_num is invalid
        if pin_num > 57 {
            return;
        }

        let reg_num = (pin_num / 10) as usize; // calculate pin registry number
        let bit_start = (pin_num % 10) * 3; // calculate pin bits start position

        unsafe {
            let reg_addr = GPFSEL0.add(reg_num * 4); // calculate pin registry address
            let mut reg_val = ptr::read_volatile(reg_addr); // read entire pin GPFSEL address

            reg_val &= !(0b111 << bit_start); // clear pin function bits
            reg_val |= (func as u32) << bit_start; // set function bits to desired function

            ptr::write_volatile(reg_addr, reg_val); // write new GPFSEL address value
        }
    }
}

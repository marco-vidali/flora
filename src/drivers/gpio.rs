use crate::drivers::MMIO_BASE;
use core::ptr;

const GPIO_BASE: usize = MMIO_BASE + 0x20_0000;
const GPIO_PUP_PDN0: *mut u32 = (GPIO_BASE + 0xE4) as *mut u32;
const GPFSEL0: *mut u32 = GPIO_BASE as *mut u32;

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
}

use core::arch::asm;

pub fn get_current_el() -> u8 {
    let mut cur_el: u64;

    unsafe {
        asm!("mrs {}, CurrentEL", out(reg) cur_el); // get current exception level from system register
    }

    ((cur_el >> 2) & 0b11) as u8 // isolate exception level (bits 2-3)
}

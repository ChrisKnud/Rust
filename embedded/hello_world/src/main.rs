#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

const RCC_BASE_ADDR: u32 = 0x4002_1000; // RCC Clock register adress
const GPIOA_AHB2_ENR: *mut u32 = (RCC_BASE_ADDR + 0x4C) as *mut u32; // AHB2ENR register address

const GPIOA_BASE_ADDR: u32 = 0x4800_0000; // GPIOA base address
const GPIOA_MODER: *mut u32 = (GPIOA_BASE_ADDR + 0x00) as *mut u32; // GPIOA MODER address

// BSRR Register
const GPIOA_BSSR: *mut u32 = (GPIOA_BASE_ADDR + 0x18) as *mut u32; // GPIOA BSRR register address

#[entry]
fn main() -> ! {
    unsafe { 
        // Enable GPIOA clock
        hprintln!("Clock before. {:b}", *GPIOA_AHB2_ENR).unwrap(); // Print the value of GPIOA_AHB2_ENR as binary
        core::ptr::write_volatile(GPIOA_AHB2_ENR, 1 << 0); // Set bit 0 to 1
        hprintln!("Clock after. {:b}", *GPIOA_AHB2_ENR).unwrap(); // Print the value of GPIOA_AHB2_ENR as binary

        // Set pin to output
        hprintln!("Moder before. {:b}", *GPIOA_MODER).unwrap();
        *GPIOA_MODER &= !(0b11 << 8);
        *GPIOA_MODER |= 0b01 << 8;
        hprintln!("Moder after. {:b}", *GPIOA_MODER).unwrap();

        loop {
            hprintln!("Entering loop.").unwrap();

            hprintln!("Register enabled. {:b}", *GPIOA_AHB2_ENR).unwrap();

            hprintln!("Turning on LED. BSSR: {:b}", *GPIOA_BSSR).unwrap();
            // Turn on led
            core::ptr::write_volatile(GPIOA_BSSR, 1 << 4); // Set BS0 to 1.
            hprintln!("BSSR: {:b}", *GPIOA_BSSR).unwrap();
            for _ in 0..100 {
                cortex_m::asm::nop();
            }

            hprintln!("Turning off LED.").unwrap();
            // Turn of led
            core::ptr::write_volatile(GPIOA_BSSR, 1 << 20); // Set BR0 to 1.

            for _ in 0..100 {
                cortex_m::asm::nop();
            }
        }
    }
}

#[panic_handler]
fn panic_handler_crash(info: &core::panic::PanicInfo) -> ! {
    hprintln!("Panic! Error: {}", info).unwrap();
    loop {}
}

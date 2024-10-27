#![no_std]
#![no_main]

use stm32f4::stm32f411;
use core::ptr::{read_volatile, write_volatile};
use cortex_m::asm;
use cortex_m_rt::entry;
// use cortex_m_semihosting::hprintln;

// select a panic handler; i.e. pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM supporc
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

#[entry]
fn main() -> ! {

    const PERIPH_BASE: u32 = 0x40000000;
    const AHB1PERIPH_OFFSET: u32 = 0x00020000;
    const AHB1PERIPH_BASE: u32 = PERIPH_BASE + AHB1PERIPH_OFFSET; 
    
    const GPIOC_OFFSET: u32 = 0x800;
    const GPIOC_BASE: u32 = AHB1PERIPH_BASE + GPIOC_OFFSET;

    const RCC_OFFSET: u32 = 0x3800;
    const RCC_BASE: u32 = AHB1PERIPH_BASE + RCC_OFFSET;

    const AHB1EN_R_OFFSET: u32 = 0x30;
    const RCC_AHB1EN_R: *mut u32 = (RCC_BASE + AHB1EN_R_OFFSET) as *mut u32; 
   
    const MODE_R_OFFSET: u32 = 0x00;
    const GPIOC_MODE_R: *mut u32 = (GPIOC_BASE + MODE_R_OFFSET) as *mut u32; 

    const OD_R_OFFSET: u32 = 0x14;
    const GPIOC_OD_R: *mut u32 = (GPIOC_BASE + OD_R_OFFSET) as *mut u32; 

    const GPIOCEN: u32 = 1 << 2;  // 0b 0000 0000 0000 0000 0000 0000 0000 0100
   
    unsafe {
        // enable clock access to PC13 (GPIOC pin 13)
        let mut val: u32 = read_volatile(RCC_AHB1EN_R);
        val |= GPIOCEN; 
        write_volatile(RCC_AHB1EN_R, val);

        // each pin in mode register is configured using two bits
        // set PC13 as output pin
        let mut val: u32 = read_volatile(GPIOC_MODE_R);  // read current value of mode register
        val |= (1 << 26);                                // set bit 26 equal to 1
        write_volatile(GPIOC_MODE_R, val);
        let mut val: u32 = read_volatile(GPIOC_MODE_R);  // read current values of mode register
        val &= !(1 << 27);                               // clear bit 27
        write_volatile(GPIOC_MODE_R, val);
    }

    // PC13
    const LED_PIN: u32 = 1 << 13;
    let mut is_on: bool = false;
    loop {
        unsafe {
            let val: u32 = if is_on { LED_PIN } else { 0 };
            write_volatile(GPIOC_OD_R, val);
        }
        
        // simple delay
        for _ in 0..400_000 {
            asm::nop();  // no operation
        }

        is_on = !is_on;
    }

}


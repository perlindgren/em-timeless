#![no_std]
#![no_main]

use cortex_m::{
    interrupt::{InterruptNumber, Nr},
    peripheral::{DWT, NVIC},
};
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{debug, hprintln};
use lm3s6965::{interrupt, Interrupt, Interrupt::GPIOA};
use panic_semihosting as _;

use cortex_m::peripheral::Peripherals;
use em_timeless::*;

#[entry]
fn main() -> ! {
    hprintln!("start");

    let mut p = Peripherals::take().unwrap();

    let timer = DwtSystick::<FREQ>::new(&mut p.DCB, p.DWT, p.SYST, FREQ);
    unsafe {
        MONO = Some(timer);
        NVIC::unmask(lm3s6965::Interrupt::GPIOA);
    }

    loop {}
}
const FREQ: u32 = 1000_0000;
static mut MONO: Option<DwtSystick<FREQ>> = None;

#[interrupt]
fn GPIOA() {
    hprintln!("gpioa");
}

#[exception]
fn SysTick() {
    static mut NR: u32 = 0;
    if *NR == 10 {
        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
    }
    hprintln!("tick {:?}", DWT::get_cycle_count()).ok();
    NVIC::pend(GPIOA);
    *NR += 1;
}

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

    // let syst = DwtSystick::<FREQ>::new(&mut p.DCB, p.DWT, p.SYST, FREQ);

    unsafe {
        //   MONO = Some(syst);
        NVIC::unmask(lm3s6965::Interrupt::GPIOA);
    }

    let mut timer = unsafe { &mut TIMER };
    timer.compare = 5;
    timer.enable = true;
    timer.init(&mut p.DCB, p.DWT, p.SYST);

    loop {}
}
const FREQ: u32 = 1000_0000;
static mut MONO: Option<DwtSystick<FREQ>> = None;

// Compare interrupt
#[interrupt]
fn GPIOA() {
    let mut timer = unsafe { &mut TIMER };
    hprintln!("compare interrupt @{}", timer.counter);

    debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
}

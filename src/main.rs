#![no_std]
#![no_main]
#![allow(unused_imports)]

use cortex_m::delay::Delay;
use defmt::*;
use defmt_rtt as _;

use embedded_hal::digital::OutputPin;
use hal::{clocks::init_clocks_and_plls, entry, gpio::Pins, pac, Clock, Sio, Watchdog};
use panic_probe as _;
use rp2040_hal as hal;

const EXT_XTAL_F: u32 = 12_000_000u32; // RP-Pico external crystal oscilltor (12MHz).

#[link_section = ".boot2"]
#[used]

pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H; // Bootloader. Some flash chips needs a different bootloader.

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap(); // Peripheral Access Controller
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG); // Watchdog
    let sio = Sio::new(pac.SIO); // Single-cycle IO

    let clocks = init_clocks_and_plls(
        // Init the clocks for the delay function.
        EXT_XTAL_F,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.gpio25.into_push_pull_output();

    loop {
        let _ = led_pin.set_high(); // This function actually returns a 'Result', but I don't care about it.
        delay.delay_ms(1000);
        let _ = led_pin.set_low();
        delay.delay_ms(1000);
    }
}

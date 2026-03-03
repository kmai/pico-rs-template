#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rp2040_hal::{
    clocks::{init_clocks_and_plls, Clock},
    gpio::Pins,
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

// Puya P25Q16H compatible boot2 — required for clone Pico boards.
// See CLAUDE.md for details.
#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

#[entry]
fn main() -> ! {
    // Acquire PAC-level peripheral singletons
    let mut pac = pac::Peripherals::take().unwrap();
    let _core = pac::CorePeripherals::take().unwrap();

    // Initialise watchdog — required by init_clocks_and_plls
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    // Bring up the PLL at 125 MHz from the 12 MHz crystal
    let _clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Initialise SIO and GPIO bank
    let sio = Sio::new(pac.SIO);
    let _pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Application code goes here
    loop {
        cortex_m::asm::nop();
    }
}

use core::panic::PanicInfo;
use kernel::debug;
use kernel::hil::led;
use nrf52840::gpio::Pin;

/// Panic.
#[cfg(not(test))]
#[no_mangle]
#[panic_handler]
pub unsafe extern "C" fn panic_fmt(_pi: &PanicInfo) -> ! {
    const LED1_PIN: Pin = Pin::P0_13;
    let led = &mut led::LedLow::new(&mut nrf52840::gpio::PORT[LED1_PIN]);
    debug::panic_blink_forever(&mut [led])
}
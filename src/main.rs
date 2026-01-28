#![no_std]
#![no_main]

use cortex_m::asm::delay;
use teensy4_bsp as bsp;
use panic_halt as _;

#[bsp::rt::entry]
fn main() -> ! {
    let bsp::board::Resources { mut gpio2, pins, .. } =
        bsp::board::t41(bsp::board::instances());
    let led = bsp::board::led(&mut gpio2, pins.p13);

    loop {
        led.toggle();
        delay(300_000_000);
    }
}

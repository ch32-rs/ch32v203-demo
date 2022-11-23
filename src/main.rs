#![no_std]
#![no_main]

use riscv_rt::entry;
// provide implementation for critical-section
use panic_halt as _;
use riscv as _;

use ch32v2::ch32v20x as pac;

#[entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let rcc = peripherals.RCC;

    // rcc.apb2prstr.modify(|_, w| w.iopbrst().set_bit());
    rcc.apb2pcenr.modify(|_, w| w.iopben().set_bit());

    let gpiob = peripherals.GPIOB;

    // Output max 50MHz
    // Push-pull
    unsafe {
        gpiob
            .cfghr
            .modify(|_, w| w.cnf8().bits(0b00).mode8().bits(0b11))
    };

    // gpiob.outdr.modify(|_, w| w.odr8().set_bit());
    //         gpiob.bshr.write(|w| w.br8().set_bit());

    // HSI 8MHz
    // 4 opcodes to do a nop sleep here
    let cycle = 8_000_000 / 4;
    loop {
        gpiob.outdr.modify(|_, w| w.odr8().set_bit());
        for _ in 0..cycle {
            unsafe {
                riscv::asm::nop();
            }
        }

        gpiob.outdr.modify(|_, w| w.odr8().clear_bit());
        for _ in 0..cycle {
            unsafe {
                riscv::asm::nop();
            }
        }
    }
}

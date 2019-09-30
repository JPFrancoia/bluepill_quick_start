// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m as cm;
extern crate cortex_m_rt as rt;
extern crate panic_itm;
extern crate stm32f1xx_hal as hal;

// #[macro_use(block)]
extern crate nb;


use nb::block;

use cm::iprintln;
use hal::prelude::*;
use hal::{
    delay::Delay,
    serial::{self, Serial},
    stm32,
};

use rt::entry;

#[entry]
fn main() -> ! {
    // Get control of the PC13 pin
    let device_peripherals = stm32::Peripherals::take().unwrap();
    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();

    let mut rcc = device_peripherals.RCC.constrain();
    let mut flash = device_peripherals.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = device_peripherals.GPIOA.split(&mut rcc.apb2);
    let mut gpioc = device_peripherals.GPIOC.split(&mut rcc.apb2);

    // // Won't work without a USB adapter, see:
    // // https://blog.japaric.io/itm/
    // let mut itm = cortex_peripherals.ITM;

    // Prepare the alternate function I/O registers
    let mut afio = device_peripherals.AFIO.constrain(&mut rcc.apb2);

    // USART1
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    //let serial = Serial::usart1(
    //device_peripherals.USART1,
    //(tx, rx),
    //&mut afio.mapr,
    //serial::Config::default()
    //.baudrate(9600.bps())
    //.stopbits(serial::StopBits::STOP2)
    //.parity_odd(),
    //clocks,
    //&mut rcc.apb2,
    //);

    let serial = Serial::usart1(
        device_peripherals.USART1,
        (tx, rx),
        &mut afio.mapr,
        9600.bps(),
        clocks,
        &mut rcc.apb2,
    );

    // Split the serial struct into a receiving and a transmitting part
    let (mut tx, _rx) = serial.split();

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let half_period = 100_u16;
    let mut delay = Delay::new(cortex_peripherals.SYST, clocks);

    // We can only send byte per byte with the serial
    let sent = b'U';

    loop {
        led.set_high();
        delay.delay_ms(half_period);
        led.set_low();
        delay.delay_ms(half_period);
        // block!(tx.write(sent)).ok();
        tx.write(sent).ok();
    }
}

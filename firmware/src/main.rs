#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::InterruptHandler;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

mod usb;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let usb_future = usb::usb(p.USB, Irqs);

    let tick_future = async {
        loop {
            info!("tick");
            Timer::after_micros(500_000).await;
            info!("tock");
            Timer::after_micros(500_000).await;
        }
    };

    join(usb_future, tick_future).await;
}

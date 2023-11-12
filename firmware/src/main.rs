#![no_std]
#![no_main]
#![feature(const_option)]
#![feature(type_alias_impl_trait)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::InterruptHandler;
use embassy_time::Timer;
use rp2040_flash::flash;
use {defmt_rtt as _, panic_probe as _};

mod usb;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let _jedec_id: u32 = unsafe { cortex_m::interrupt::free(|_cs| flash::flash_jedec_id(true)) };
    let mut unique_id = [0u8; 8];
    unsafe { cortex_m::interrupt::free(|_cs| flash::flash_unique_id(&mut unique_id, true)) };
    let usb_future = usb::usb(p.USB, Irqs, &unique_id);

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

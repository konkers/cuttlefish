#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
        let _p = embassy_rp::init(Default::default());
    loop {
        info!("tick");
        Timer::after_micros(500_000).await;
        info!("tock");
        Timer::after_micros(500_000).await;
    }
}

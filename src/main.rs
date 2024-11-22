#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use {defmt_rtt as _, panic_probe as _}; // global logger

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Roborust!");
    let _p = embassy_stm32::init(Config::default());
}

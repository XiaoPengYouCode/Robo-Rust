#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use {defmt_rtt as _, panic_probe as _}; // global logger

use roborust::bored_resources::*;
use roborust::split_resources;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Roborust!");
    let p = embassy_stm32::init(Config::default());
    let _r = split_resources!(p);
}

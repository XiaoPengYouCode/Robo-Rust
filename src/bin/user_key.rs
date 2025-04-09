#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _}; // global logger

use roborust::bored::bored_config::dm02_bored_config;
use roborust::bored::bored_resources::*;
use roborust::split_resources;
use roborust::tasks::task_user_key::task_user_key;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    debug!("Roborust!");
    let p = embassy_stm32::init(dm02_bored_config());
    let r = split_resources!(p);

    spawner.spawn(task_user_key(r.user_key)).unwrap();
}

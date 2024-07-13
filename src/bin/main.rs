#![no_std]
#![no_main]

use embassy_executor::Spawner;
use defmt::*;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Roborust!")
}
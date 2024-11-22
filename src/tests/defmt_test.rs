#![cfg(test)]
use super::*;
use embassy_executor::Spawner;
use embassy_stm32::Config;

#[embassy_executor::test]
async fn test_initialization(_spawner: Spawner) {
    let _p = embassy_stm32::init(Config::default());
    // Add assertions or checks here to verify initialization
    assert!(true); // Placeholder assertion
}

#[embassy_executor::test]
async fn test_logging(_spawner: Spawner) {
    info!("Test logging!");
    // Add assertions or checks here to verify logging
    assert!(true); // Placeholder assertion
}
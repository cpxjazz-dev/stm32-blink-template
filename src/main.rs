#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

// Blue Pill（STM32F103C8T6）最小 Blink：板载 LED 在 PC13，低电平亮。
// 初始 Level::High = 灯灭，toggle() 每 500ms 翻转一次。
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    info!("Blue Pill blink started");
    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}

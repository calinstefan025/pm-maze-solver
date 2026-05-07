#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // Configurare Motor Stanga (conectat la OUT1/OUT2)
    // ENA - Control viteza
    let mut _ena = Output::new(p.PB3, Level::High, Speed::Low);
    // IN1 si IN2 - Control directie
    let mut in1 = Output::new(p.PB5, Level::Low, Speed::Low);
    let mut in2 = Output::new(p.PB4, Level::Low, Speed::Low);

    // Configurare Motor Dreapta (conectat la OUT3/OUT4)
    // ENB - Control viteza
    let mut _enb = Output::new(p.PB10, Level::High, Speed::Low);
    // IN3 si IN4 - Control directie
    let mut in3 = Output::new(p.PA7, Level::Low, Speed::Low);
    let mut in4 = Output::new(p.PA6, Level::Low, Speed::Low);

    info!("Test motoare initiat");

    loop {
        // Test Mers Inainte
        info!("Mers inainte");
        in1.set_high();
        in2.set_low();

        in3.set_high();
        in4.set_low();
        Timer::after_millis(2000).await;

        // Pauza
        info!("Stop");
        in1.set_low();
        in2.set_low();

        in3.set_low();
        in4.set_low();
        Timer::after_millis(1000).await;

        // Test Mers Inapoi
        info!("Mers inapoi");
        in1.set_low();
        in2.set_high();

        in3.set_low();
        in4.set_high();
        Timer::after_millis(2000).await;

        // Pauza
        info!("Stop");
        in1.set_low();
        in2.set_low();

        in3.set_low();
        in4.set_low();
        Timer::after_millis(1000).await;
    }
}

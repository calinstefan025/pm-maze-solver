#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Pull};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // output senzor -> PA0
    let senzor_ir = Input::new(p.PA0, Pull::None);

    info!("Incepem citirea senzorului IR...");

    loop {
        // Senzorul scoate LOW cand detecteaza un obstacol (lumina reflectata)
        if senzor_ir.is_low() {
            info!("OBSTACOL DETECTAT!");
        } else {
            // Senzorul scoate HIGH cand nu se reflecta lumina inapoi
            info!("Drum liber...");
        }

        Timer::after_millis(100).await;
    }
}
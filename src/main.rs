#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_time::{Instant, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // pini pentru senzorii IR (laterale - A0 si A1)
    let ir_stanga = Input::new(p.PA0, Pull::None);
    let ir_dreapta = Input::new(p.PA1, Pull::None);

    // pini pentru senzorul ultrasonic
    let mut trig = Output::new(p.PC7, Level::Low, Speed::VeryHigh);
    let echo = Input::new(p.PC8, Pull::None);

    info!("Pornire sistem senzori: IR x2 + Ultrasonic...");

    loop {
        // 1. citire status senzori IR
        let stare_stanga = if ir_stanga.is_low() {
            "OBSTACOL"
        } else {
            "Liber"
        };
        let stare_dreapta = if ir_dreapta.is_low() {
            "OBSTACOL"
        } else {
            "Liber"
        };

        // 2. emitere puls ultrasonic
        trig.set_high();
        Timer::after_micros(10).await;
        trig.set_low();

        // 3. asteptam startul ecoului cu o plasa de siguranta (TIMEOUT)
        let wait_start = Instant::now();
        let mut echo_primit = true;

        while echo.is_low() {
            // daca asteapta mai mult de 50 de milisecunde, e clar o problema fizica
            if wait_start.elapsed().as_micros() > 50000 {
                echo_primit = false;
                break;
            }
        }

        // daca firul ECHO nu a raspuns, dam eroare dar nu blocam programul
        if !echo_primit {
            info!(
                "Stanga: {} | Dreapta: {} | Fata: EROARE CITIRE (Verifica firul ECHO din D10!)",
                stare_stanga, stare_dreapta
            );
            Timer::after_millis(500).await;
            continue; // sare peste restul buclei si o ia de la capat
        }

        // 4. asteptam intoarcerea ecoului cu o limita de siguranta
        let start_time = Instant::now();
        while echo.is_high() {
            if start_time.elapsed().as_micros() > 30000 {
                break;
            }
        }
        let end_time = Instant::now();

        // 5. calculare timp si distanta
        let duration = end_time.duration_since(start_time).as_micros();

        // 6. afisarea tuturor senzorilor simultan in consola
        if duration < 30000 {
            let distance_cm = duration / 58;
            info!(
                "Stanga: {} | Dreapta: {} | Fata: {} cm",
                stare_stanga, stare_dreapta, distance_cm
            );
        } else {
            info!(
                "Stanga: {} | Dreapta: {} | Fata: Liber (fara obstacol)",
                stare_stanga, stare_dreapta
            );
        }

        // pauza pentru lizibilitate in terminal
        Timer::after_millis(500).await;
    }
}

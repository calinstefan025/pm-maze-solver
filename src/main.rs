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

    // Configurare Motor Stanga (conectat la OUT1/OUT2)
    let mut _ena = Output::new(p.PB3, Level::High, Speed::Low);
    let mut in1 = Output::new(p.PB5, Level::Low, Speed::Low);
    let mut in2 = Output::new(p.PB4, Level::Low, Speed::Low);

    // Configurare Motor Dreapta (conectat la OUT3/OUT4)
    let mut _enb = Output::new(p.PB10, Level::High, Speed::Low);
    let mut in3 = Output::new(p.PA7, Level::Low, Speed::Low);
    let mut in4 = Output::new(p.PA6, Level::Low, Speed::Low);

    // pini pentru senzorii IR (laterale - A0 si A1)
    let ir_stanga = Input::new(p.PA0, Pull::None);
    let ir_dreapta = Input::new(p.PA1, Pull::None);

    // pini pentru senzorul ultrasonic
    let mut trig = Output::new(p.PC7, Level::Low, Speed::VeryHigh);
    let echo = Input::new(p.PC8, Pull::None);

    info!("Pornire sistem in 5 secunde, pune masina pe jos...");
    Timer::after_secs(5).await;
    info!("Pornire sistem senzori si motoare!");

    loop {
        // 1. citire status senzori IR
        let stare_stanga_liber = ir_stanga.is_high(); // In general 1 = liber, dar in cod era: if is_low() -> "OBSTACOL"
        let stare_dreapta_liber = ir_dreapta.is_high();

        // 2. emitere puls ultrasonic
        trig.set_high();
        Timer::after_micros(10).await;
        trig.set_low();

        // 3. asteptam startul ecoului cu o plasa de siguranta (TIMEOUT)
        let wait_start = Instant::now();
        let mut echo_primit = true;

        while echo.is_low() {
            if wait_start.elapsed().as_micros() > 50000 {
                echo_primit = false;
                break;
            }
        }

        if !echo_primit {
            info!("Eroare senzor ultrasonic - se opresc motoarele");
            in1.set_low(); in2.set_low();
            in3.set_low(); in4.set_low();
            Timer::after_millis(100).await;
            continue;
        }

        // 4. asteptam intoarcerea ecoului
        let start_time = Instant::now();
        while echo.is_high() {
            if start_time.elapsed().as_micros() > 30000 {
                break;
            }
        }
        let end_time = Instant::now();
        let duration = end_time.duration_since(start_time).as_micros();

        let mut fata_liber = true;
        let mut current_distance_cm = 999;
        if duration < 30000 {
            current_distance_cm = duration / 58;
            if current_distance_cm < 10 {
                fata_liber = false;
            }
        }

        if fata_liber {
            info!("Fata libera ({} cm) -> Mergem inainte", current_distance_cm);
            // Mergem inainte
            in1.set_high();
            in2.set_low();
            in3.set_high();
            in4.set_low();
            Timer::after_millis(50).await; // O mica asteptare
        } else {
            info!("Obstacol detectat in fata ({} cm) -> Oprim si asteptam 1s", current_distance_cm);
            // Fata blocata, oprim si verificam dreapta
            in1.set_low(); in2.set_low();
            in3.set_low(); in4.set_low();
            
            // Asteptam 1 secunda inainte de a face rotirea
            Timer::after_millis(1000).await;
            
            if stare_dreapta_liber {
                info!("Dreapta libera -> Ne rotim la dreapta");
                // Rotire la dreapta ~90 grade (ajusteaza timpul la nevoie)
                in1.set_low(); in2.set_high();
                in3.set_high(); in4.set_low();
                Timer::after_millis(500).await;
                
                info!("Rotire terminata -> Oprim motoarele temporar");
                // Oprim motoarele
                in1.set_low(); in2.set_low();
                in3.set_low(); in4.set_low();
            } else {
                info!("Obstacol si in dreapta -> Nu ne rotim, mai asteptam");
                // Optional: rotire stanga daca vrem, dar in cerinta doar dreapta
                Timer::after_millis(100).await;
            }
        }
    }
}

use rppal::gpio::{Gpio};
use std::time::{Duration};
use std::thread;

mod sensor;

use crate::sensor::Sensor;

const POLL_TIME: u64 = 500;

fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let mut sensor = Sensor::new(Gpio::new()?.get(18)?.into_output(), Gpio::new()?.get(24)?.into_input());

    loop {
        println!("{}", sensor.is_triggered());
        thread::sleep(Duration::from_millis(POLL_TIME));
    }
}

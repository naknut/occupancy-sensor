use rppal::gpio::{Gpio};
use std::time::{Duration};
use std::thread;

mod sensor;

use crate::sensor::Sensor;

const POLL_TIME: u64 = 500;

fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let trigger = Gpio::new()?.get(18)?.into_output();
    let echo = Gpio::new()?.get(24)?.into_input();
    let mut sensor = Sensor::new(trigger, echo);

    loop {
        println!("{}", sensor.is_triggered());
        thread::sleep(Duration::from_millis(POLL_TIME));
    }
}

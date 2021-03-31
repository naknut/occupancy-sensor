use rppal::gpio::{Gpio, OutputPin, InputPin};
use std::time::{Duration, Instant};
use std::thread;

const POLL_TIME: u64 = 500;

fn echo_time(trigger_pin: &mut OutputPin, echo_pin: &InputPin) -> u128 {
    trigger_pin.set_high();
    thread::sleep(Duration::from_nanos(10000));
    trigger_pin.set_low();

    let mut start_time = Instant::now();
    let mut elapsed = start_time.elapsed();
    
    while echo_pin.is_low() {
        start_time = Instant::now()
    }

    while echo_pin.is_high() {
        elapsed = start_time.elapsed()
    }

    return elapsed.as_millis()
}

fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let mut trigger_pin = Gpio::new()?.get(18)?.into_output();
    let echo_pin = Gpio::new()?.get(24)?.into_input();

    let mut trigger = false;
    let normal_echo_time = echo_time(&mut trigger_pin, &echo_pin);

    loop {
        thread::sleep(Duration::from_millis(POLL_TIME));
        let echo_time = echo_time(&mut trigger_pin, &echo_pin);
        trigger = echo_time < normal_echo_time / 2;
        println!("{}", trigger);
    }
}

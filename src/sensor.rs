use rppal::gpio::{OutputPin, InputPin};
use std::thread;
use std::time::{Duration, Instant};
use std::cell::Cell;

pub struct Sensor {
    trigger: Cell<OutputPin>,
    echo: InputPin,
    calibration: u128
}

impl Sensor {
    fn echo_time(trigger: &mut OutputPin, echo: &InputPin) -> u128 {
        trigger.set_high();
        thread::sleep(Duration::from_nanos(10000));
        trigger.set_low();
    
        let mut start_time = Instant::now();
        let mut elapsed = start_time.elapsed();
        
        while echo.is_low() {
            start_time = Instant::now()
        }
    
        while echo.is_high() {
            elapsed = start_time.elapsed()
        }
    
        return elapsed.as_millis()
    }

    pub fn is_triggered(&mut self) -> bool {
        let measurement = Sensor::echo_time(self.trigger.get_mut(), &self.echo);
        println!("Calibration: {}, Measurement: {}", self.calibration, measurement);
        return measurement < self.calibration / 2 || measurement > self.calibration * 2;
    }

    pub fn new(trigger: OutputPin, echo: InputPin) -> Sensor {
        let mut trigger = Cell::new(trigger);
        let calibration = Sensor::echo_time(trigger.get_mut(), &echo);
        Sensor {trigger, echo, calibration}
    }
}

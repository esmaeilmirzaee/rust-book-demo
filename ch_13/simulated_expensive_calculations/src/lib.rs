use std::thread;
use std::time::Duration;

pub fn simulated_expensive_calculations(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(5));
    intensity
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("Today, do {} pushups.", simulated_expensive_calculations(intensity));
        println!("Next, do {} situps.", simulated_expensive_calculations(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break, remember to stay hydrated!");
        } else {
            println!("Remember to run for {} minutes!", simulated_expensive_calculations
                (intensity));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_generate_workout() {
        assert_eq!(25, simulated_expensive_calculations(25));
    }
}

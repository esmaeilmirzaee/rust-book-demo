use std::thread;
use std::time::Duration;

struct CacheMap<T> {
    key: Option<T>,
    value: Option<T>,
}

pub struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: CacheMap<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: CacheMap {
                key: None,
                value: None,
            },
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.key {
            Some(v) => {
                if v == arg {
                    return v;
                } else {
                    let v = (self.calculation)(arg);
                    self.value = CacheMap {
                        key: Some(v),
                        value: Some(v),
                    };
                    v
                }
            }
            None => {
                let v = (self.calculation)(arg);
                self.value = CacheMap {
                    key: Some(v),
                    value: Some(v),
                };
                v
            }
        }
    }
}

pub fn simulated_expensive_calculations(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(5));
    intensity
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculations(intensity);
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups.", expensive_result.value(intensity));
        println!("Next, do {} situps.", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break, remember to stay hydrated!");
        } else {
            println!("Remember to run for {} minutes!", expensive_result.value(intensity));
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

    #[test]
    fn call_with_different_value() {
        let mut c = Cacher::new(|x| x);

        let v1 = c.value(1);
        let v2 = c.value(2);
        println!("V1: {}\nV2: {}", v1, v2);
        assert_eq!(2, v2);
    }
}

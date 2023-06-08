use std::thread;
use std::time::Duration;

fn main() {
    let simualated_intensity = 10;
    let simulated_random_intensity = 7;
    generate_workout(simualated_intensity, simulated_random_intensity)
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached_value = Cacher::new(|n| {
        println!("Slowly calculating...");
        thread::sleep(Duration::from_secs(2));
        n
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", cached_value.value(intensity));

        println!("Next, do {} situps!", cached_value.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remeber to stay hydrated");
        } else {
            println!("Today, run for {} minutes!", cached_value.value(intensity));
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

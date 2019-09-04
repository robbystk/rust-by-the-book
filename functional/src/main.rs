use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly . . .");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(simulated_expensive_calculation);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity),
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity),
        );
    } else {
        if random_number == 3 {
            println!("Take a break today, you've earned it.");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity),
            );
        }
    }
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    function: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(func: T) -> Cacher<T> {
        Cacher {
            function: func,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.function)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );
}

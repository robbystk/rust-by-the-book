use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;

fn simulated_expensive_calculation(intensity: &u32) -> u32 {
    println!("calculating slowly . . .");
    thread::sleep(Duration::from_secs(2));
    *intensity
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

struct Cacher<T, A, V>
    where
        T: Fn(&A) -> V,
        A: Hash + Eq,
        V: Copy,
{
    function: T,
    values: HashMap<A, V>
}

impl<T, A, V> Cacher<T, A, V>
    where
        T: Fn(&A) -> V,
        A: Hash + Eq,
        V: Copy,
{
    fn new(func: T) -> Cacher<T, A, V> {
        Cacher {
            function: func,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: A) -> V {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.function)(&arg);
                self.values.insert(arg, v);
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

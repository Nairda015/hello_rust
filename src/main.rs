use std::{thread, time::Duration, collections::HashMap};

fn main() {
    let intensivity = 10;
    let random_number = 7;

    generate_workout(intensivity, random_number);
}

fn generate_workout(intensivity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensivity < 25 {
        println!("Today, do {} pushups!", cached_result.value(intensivity));
        println!("Next, do {} situps!", cached_result.value(intensivity));
    } else {
        if random_number == 3 {
            println!("Take a break")
        } else {
            println!(
                "Today, run for {} minutes!",
                cached_result.value(intensivity)
            );
        }
    }

    println!("{:#?}", cached_result.values)
}

#[derive(Debug)]
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calcualtion: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calcualtion: calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let value = (self.calcualtion)(arg);
                self.values.insert(arg, value);
                value
            }
        }
    }
}

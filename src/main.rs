use std::{thread, time::Duration, collections::{HashMap, hash_map::Entry}, hash::Hash};

fn main() {
    let intensity = 10;
    let random_number = 7;

    generate_workout(intensity, random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cached_result.value(intensity));
        println!("Next, do {} situps!", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break")
        } else {
            println!(
                "Today, run for {} minutes!",
                cached_result.value(intensity)
            );
        }
    }

    println!("{:#?}", cached_result.values)
}

#[derive(Debug)]
struct Cacher<T, K, V>
{
    calculation: T,
    values: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Hash + Eq + Clone,
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> &V {
        let value = match self.values.entry(arg.clone()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert((self.calculation)(arg)),
        };

        value
    }
}

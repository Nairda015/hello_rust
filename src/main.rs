use std::{thread, time::Duration, collections::HashMap, hash::Hash};

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
struct Cacher<T, K, V>
where
    T: Fn(&K) -> &V,
    K: Hash + Eq,
    V: Copy
{
    calcualtion: T,
    values: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
    T: for<'a> Fn(&'a K) -> &'a V,
    K: Hash + Eq + Copy,
    V: Copy
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calcualtion: calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let key = arg.clone();
                let value = (self.calcualtion)(&arg);
                self.values.insert(key, *value);
                *value
            }
        }
    }
}

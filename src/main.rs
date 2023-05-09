fn main() {
    println!("Hello world!");

    let number_list = vec![1, 13, 5, 7];
    let largest = get_largest(number_list);
    println!("{}", largest);

    let number_list = vec!['a', 'z', 'g', '4'];
    let largest = get_largest(number_list);
    println!("{}", largest);

    let p1 = Point { x: 5, y: 6 };
    println!("{:#?}", p1);

    let p2 = Point {x: 5.1, y: 12.3};
    println!("{}", p2.normal())
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn normal(&self) -> f64 {
        (self.x.powi(2)  * self.y.powi(2)).sqrt()
    }
}

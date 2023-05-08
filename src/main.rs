fn main() {
    println!("Hello world!");

    let four = IpAddresKind::V4;
    let six = IpAddresKind::V6;

    let adress = IpAddresKind::V4(127, 0, 0, 1);
    println!("{:?}", adress);

    let some_number = Some(3);
    let none_number: Option<i32> = None;
    let numer = 5;
    let sum = some_number.unwrap() + none_number.unwrap_or(0) + numer;
    println!("{:?}", sum);

    let message = Message::ChangeColor(31, 12, 25);
    log_message(message);

    println!("{}", plus_one(some_number).unwrap_or(0));

    match some_number {
        Some(3) => println!("three"),
        _ => (),
    };

    if let Some(3) = some_number {
        println!("three")
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn log_message(message: Message) {
    match message {
        Message::Quit => println!("Empty"),
        Message::Move { x, y } => println!("x: {}, y: {}", x, y),
        Message::Write(val) => println!("val {}", val),
        Message::ChangeColor(r, b, g) => println!("r: {}, b: {}, g: {}", r, b, g),
    }
}

#[derive(Debug)]
enum IpAddresKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: IpAddresKind) {}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some() {
        println!("A new message appeared");
    }
}

//Option

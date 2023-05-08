use std::collections::HashMap;

fn main() {
    let a = [1, 2, 3];

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let mut v2 = vec![1, 2, 3, 4, 5];

    let third = &v2[2];
    println!("The third el is: {}", third);

    match v2.get(20) {
        Some(third) => print!("The third el is: {}", third),
        None => (),
    }

    for i in &mut v2 {
        *i += 50;
    }

    for i in &v2 {
        println!("{}", i)
    }

    let mut row = vec![
        SpreadsheetCell::Int(2),
        SpreadsheetCell::Float(2.1),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not a int")
    }

    row.push(SpreadsheetCell::Int(4));


    let blue = "blue".to_string();
    let yellow = "yellow".to_string();

    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let score = scores.get("blue");
    println!("{}", score.unwrap_or(&0));

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace()
    {
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }
    println!("{:?}", map)
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
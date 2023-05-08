fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = "six";
    println!("The value of x is: {}", x);

    //const SUB_COUNT: u32 = 100_000;
    let sum = my_func(1, 2);
    println!("Value of x + y: {}", sum);

    let coll = [1, 2, 3, 4];
    for e in coll.iter() {
        println!("{}", e);
    }
}

fn my_func(x: i32, y: i32) -> i32 {
    println!("Value of x: {}", x);
    println!("Value of y: {}", y);
    x + y
}

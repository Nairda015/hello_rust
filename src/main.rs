fn main() {
    println!("Hello world!");

    let s1 = String::from("abc");
    let s2 = String::from("abcd");

    let result = longest(s1.as_str(), s2.as_str());
    println!("{}", result);

    let s1 = String::from("abc");
    let result;
    {
        let s2 = String::from("abcd");
        result = longest2(s1.as_str(), s2.as_str());
    }
    println!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

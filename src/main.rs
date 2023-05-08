fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{:#?}", rect);

    println!("Area of rect is: {}", rect.area());

    println!("Can hold self: {}", rect.can_hold(&rect));

    let square = Rectangle::square(30);

    println!("{:#?}", square);

}

//fn area(rect: &Rectangle) -> u32 {
//    rect.width * rect.height
//}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

//Structs intro:

fn user_fun() {
    let mut user1 = User {
        email: String::from("test@gmail.com"),
        username: String::from("test"),
        sign_in_count: 1,
        active: true,
    };

    let name = user1.username;

    user1.username = String::from("test-final");

    println!("Hello {}", name);

    let user2 = build_user(user1.email, user1.username);

    let user3 = User {
        email: String::from("final@gmail.com"),
        username: String::from("final"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

struct Color(i32, i32, i32);
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

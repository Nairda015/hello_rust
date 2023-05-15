use hello_rust::{Button, Draw, Screen};

fn main() {
    let select_box = Box::new(SelectBox {
        width: 10,
        height: 5,
        options: vec![
            String::from("yes"),
            String::from("no"),
            String::from("maybe"),
        ],
    });

    let button = Box::new(Button {
        width: 10,
        height: 5,
        label: String::from("ok"),
    });

    let screen = Screen {
        components: vec![select_box, button],
    };

    screen.run()
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{:?}", self.options)
    }
}

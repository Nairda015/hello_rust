mod front_of_the_house;

use crate::front_of_the_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    front_of_the_house::hosting::add_to_waitlist();

    let mut meal = back_of_the_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
}

fn serve_order() {}

mod back_of_the_house {

    //enums have public fields by default
    pub struct Breakfast {
        pub toast: String,
        seasonal_friut: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_friut: String::from("banana"),
            }
        }
    }

    fn fix_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

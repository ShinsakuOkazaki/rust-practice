mod front_of_house;
    
mod back_of_house {
    // struct Breakfast with private seasonal_fruit field
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    // method to set feilds of struct 
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    // if enum is public, it's variants are also public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}



// use self::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }


// use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

pub fn eat_at_restaurant() {
    // Absolute  path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);

    //If we try to access seasonal_fruit, it throws error 
    //meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
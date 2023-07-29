// library structure. note that all rooted under the implicit `crate` module
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

mod front_of_house {
    pub mod hosting {  // needs to be pub, items in parent module can't access stuff in descendant
        pub fn add_to_waitlist() {} // pub: if you can access my parent, you can access me

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// this fn would not compile without hosting and add_to_waitlist being pub
// because this fn is defined in crate, so it can see items in crate (i.e.
// front_of_house) but not stuff defined inside modules inside crate
pub fn eat_at_restaurant() {
    // Absolute path. use if we expect to move definition
    // code separately from each other (more common)
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path. use if we expect ot move definition
    // code together
    front_of_house::hosting::add_to_waitlist();
}



fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // can use super to refer to parent module in a relative path
        super::deliver_order();
    }

    fn cook_order() {}

    // structs can be pub, but fields must be pubbed on a case-by-case basis
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // making an enum pub pubs all of its variants
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

pub fn eat_at_restaurant_3() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

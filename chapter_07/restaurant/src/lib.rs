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

// use allows us to not have to qualify the whole path (just last component)
// updated to pub use on line 135
// use crate::front_of_house::hosting;
pub fn eat_at_restaurant_4() {
    hosting::add_to_waitlist();
}

// would not compile: crate::front_of_house is only brought into the
// scope of the use. not inside this module where it is not brought in
// mod customer {
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();
//     }
// }

// this works, but is not idiomatic. leaving the parent module
// clarifies that that the function is not locally defined
use crate::front_of_house::hosting::add_to_waitlist;
pub fn eat_at_restaurant_5() {
    add_to_waitlist();
}

// however, for structs, enums, and other items, it is idiomatic
// to specify the full path...
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// ...unless you have duplicate names, and need to disasmbiguate
use std::fmt;
use std::io;
fn function1(f: &mut fmt::Formatter<'_>) -> fmt::Result {    
    write!(f, "{}", 4)
}
fn function2() -> io::Result<()> {
    Ok(())
}

// pub use: re-export the path to other scopes so the item is
// available to be used, as if it was `use`d in the other scope
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant_6() {
    hosting::add_to_waitlist();
}

// additional benefit: code that calls this as a library can now
// just use restaurant::hosting instead of restaurant::front_of_house::hosting
// better encapsulation (can restructure without breaking API)

// use std::cmp::Ordering;
// use std::io;
// can be replaced with
// use std::{io, cmp::Ordering};

// use std::io;
// use std::io::Write;
// can be replaced with
// use std::io::{self, Write};

// possible but not recommended
// use std::collections::*;
// can make it unclear what names have been brought into scope


// separating library into multiple files
mod front_of_house2; // loads the file front_of_house2.rs (or front_of_house2/mod.rs)
pub use crate::front_of_house2::hosting as hosting2;
pub fn eat_at_restaurant_7() {
    // add_to_waitlist is defined in front_of_house2
    hosting2::add_to_waitlist(); 
}

mod front_of_house3;
pub use crate::front_of_house3::hosting as hosting3;
pub fn eat_at_restaurant_8() {
    hosting3::add_to_waitlist(); 
}
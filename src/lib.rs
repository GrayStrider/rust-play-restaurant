#![allow(dead_code)]
#![allow(unused_variables)]

mod front_of_house;

// Bringing the function’s parent module into scope
// with use so we have to specify the parent module
// when calling the function makes it clear that the
// function isn’t locally defined while still minimizing repetition of the full path
use crate::front_of_house::hosting;
use self::front_of_house::hosting::add_to_waitlist; // not ideomatic for functions

pub fn eat_at_restaurant() {
	// Absolute path
	crate::front_of_house::hosting::add_to_waitlist();
	
	// Relative path
	front_of_house::hosting::add_to_waitlist();
	
	// or put the whole thing in scope
	hosting::add_to_waitlist();
	hosting::add_to_waitlist();
	hosting::add_to_waitlist();
	
	// Order a breakfast in the summer with Rye toast
	let mut meal = back_of_house::Breakfast::summer("Rye");
	// Change our mind about what bread we'd like
	meal.toast = String::from("Wheat");
	println!("I'd like {} toast please", meal.toast);
	
	// The next line won't compile if we uncomment it; we're not allowed
	// to see or modify the seasonal fruit that comes with the meal
	// meal.seasonal_fruit = String::from("blueberries");
	
	let order1 = back_of_house::Appetizer::Soup;
	let order2 = back_of_house::Appetizer::Salad;
	
}

fn serve_order() {}

mod back_of_house {
	fn fix_incorrect_order() {
		cook_order();
		super::serve_order();
	}
	
	fn cook_order() {}
	
	pub struct Breakfast {
		pub toast: String,
		seasonal_fruit: String,
	}
	
	pub enum Appetizer {
		Soup,
		Salad,
	}
	
	impl Breakfast {
		pub fn summer(toast: &str) -> Breakfast {
			Breakfast {
				toast: String::from(toast),
				seasonal_fruit: String::from("peaches"),
			}
		}
	}
}


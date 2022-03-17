mod back_of_house {
  pub enum Appetizer {
    Soup,
    Salad,
  }

  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String, // the availability of fruit changes so chef decides to choose; it is defined private intentionally
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

mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
    fn seat_at_table() {}
  }

  mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
  }
}

// using absolute path to bring hosting into the scope
use crate::front_of_house::hosting;

// using relative path to bring hosting into the scope
use self::front_of_hose::hosting; 

pub fn eat_at_restaurant() {
  // absolute path
  //crate::front_of_house::hosting::add_to_waitlist(); // hosting is in the scope
  hosting::add_to_waitlist();
  
  // relative path
  //front_of_house::hosting::add_to_waitlist(); // hosting is in the scope
  hosting::add_to_waitlist();

  let mut meal = back_of_house::Breakfast::summer("Rye");
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please. What's the seasonal fruit? It's {}.", meal.toast, meal.seasonal_fruit);
}


// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
  Lemon,
  Strawberry,
  Orange
}

struct Drink {
  flavor: Flavor,
  capacity: f64
}

fn print_flavor(flavor: Flavor) {
  match flavor {
    Flavor::Lemon => println!("Flavor: Lemon"),
    Flavor::Strawberry => println!("Flavor: Strawberry"),
    Flavor::Orange => println!("Flavor: Orange"),
  }
}

fn print_drink(drink: Drink) {
  print_flavor(drink.flavor);
  println!("Capacity: {:?} fl oz", drink.capacity);
}

fn main() {
  let my_drink = Drink {
    flavor: Flavor::Strawberry,
    capacity: 0.6
  };

  print_drink(my_drink);
}

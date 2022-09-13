// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
  let mut stock = HashMap::new();
  stock.insert("Chair", 5);
  stock.insert("Bed", 3);
  stock.insert("Table", 2);
  stock.insert("Couche", 0);

  let mut total_items = 0;

  for (item, quantity) in stock.iter() {
    println!("{:?}", item);
    match quantity {
      0 => println!("out of stock"),
      _ => println!("quantity: {:?}", quantity)
    }
    total_items += quantity;
  }

  println!("Total of items: {:?}", total_items);
}

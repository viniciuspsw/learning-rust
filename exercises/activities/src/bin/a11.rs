// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
  id: i32,
  quantity: i32,
}

fn display_id(grocery_item: &GroceryItem) {
  println!("ID: {:?}", grocery_item.id);
}

fn display_quantity(grocery_item: &GroceryItem) {
  println!("Quantity: {:?}", grocery_item.quantity);
}

fn main() {
  let eggs = GroceryItem {
    id: 3,
    quantity: 12
  };

  display_id(&eggs);
  display_quantity(&eggs);
}

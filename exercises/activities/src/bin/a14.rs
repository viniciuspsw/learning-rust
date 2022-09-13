// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
  name: String,
  age: i32,
  favorite_colour: String
}

impl Person {
  fn print(&self) {
    println!("name: {:?}, colour: {:?}", self.name, self.favorite_colour);
  }
}

fn main() {
  let people = vec![
    Person { name: "Bob".to_owned(), age: 12, favorite_colour: "green".to_owned() },
    Person { name: "Alice".to_owned(), age: 9, favorite_colour: "blue".to_owned() },
    Person { name: "Vinicius".to_owned(), age: 10, favorite_colour: "yellow".to_owned() },
  ];

  for person in people {
    if person.age <= 10 {
      person.print();
    }
  }
}

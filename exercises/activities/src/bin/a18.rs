// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
  age: i32
}

impl Customer {
  fn can_purchase(&self) -> Result<(), String> {
    match self.age >= 21 {
      true => Ok(()),
      false => Err("Cannot purchase due age restrictions".to_owned())
    }
  }
}

fn main() {
  let customer = Customer { age: 21 };

  match customer.can_purchase() {
    Ok(_) => println!("Purchase approved!"),
    Err(error) => println!("Purchase repproved: {:?}", error)
  }
}

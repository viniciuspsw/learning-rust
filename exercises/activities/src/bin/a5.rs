// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
  let mut counter = 1;
  
  loop {
    println!("{:?}", counter);
    counter += 1;
    if counter > 4 {
      break;
    }
  }
}

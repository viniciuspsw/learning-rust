// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

#[derive(Debug)]
struct Student {
  name: String,
  locker: Option<i32>
}

fn main() {
  let students = vec![
    Student { name: "Bob".to_owned(), locker: Some(12) },
    Student { name: "Alice".to_owned(), locker: None },
  ];

  println!("{:?}", students);

  for student in students {
    println!("Name: {:?}", student.name);
    match student.locker {
      Some(locker) => println!("Locker ID: {:?}", locker),
      None => println!("Student has no locker")
    }
  }
}

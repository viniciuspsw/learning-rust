// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
  Brown,
  Black,
  White
}

impl Color {
  fn print(&self) {
    match self {
      Color::Brown => println!("Color: Brown"),
      Color::Black => println!("Color: Black"),
      Color::White => println!("Color: White"),
    }
  }
}

struct Dimensions {
  width: f64,
  height: f64,
  length: f64
}

impl Dimensions {
  fn create(width: f64, height: f64, length: f64) -> Self {
    Self { width, height, length }
  }

  fn print(&self) {
    println!("Width: {:?}cm / Height: {:?}cm / Length {:?}cm", self.width, self.height, self.length);
  }
}

struct ShippingBox {
  dimensions: Dimensions,
  weight: f64,
  color: Color
}

impl ShippingBox {
  fn create(weight: f64, dimensions: Dimensions, color: Color) -> Self {
    Self { weight, dimensions, color }
  }

  fn print_characteristics(&self) {
    println!("Weight: {:?}kg", self.weight);
    self.dimensions.print();
    self.color.print();
  }
}

fn main() {
  let small_dimensions = Dimensions::create(5.5, 5.0, 3.0);
  let shipping_box = ShippingBox::create(2.34, small_dimensions, Color::White);
  shipping_box.print_characteristics();
}

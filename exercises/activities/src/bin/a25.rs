// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimiter {
    fn calc(&self) -> i32;
}

fn calc_perimiter(shape: impl Perimiter) {
    println!("Perimiter: {:?}", shape.calc())
}

struct Square {
    side_size: i32
}

impl Perimiter for Square {
    fn calc(&self) -> i32 {
        return self.side_size * 4;
    }
}

struct Triangle {
    a_size: i32,
    b_size: i32,
    c_size: i32
}

impl Perimiter for Triangle {
    fn calc(&self) -> i32 {
        return self.a_size + self.b_size + self.c_size;
    }
}

fn main() {
    calc_perimiter(Square { side_size: 4 });
    calc_perimiter(Triangle { a_size: 3, b_size: 6, c_size: 4 });
}

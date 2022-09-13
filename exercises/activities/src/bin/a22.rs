// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }

    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn clamp_first_test() {
        let result = clamp(1, 2, 10);
        assert_eq!(result, 2);
    }

    #[test]
    fn clamp_second_test() {
        let result = clamp(5, 1, 3);
        assert_eq!(result, 3);
    }

    #[test]
    fn clamp_third_test() {
        let result = clamp(5, 1, 10);
        assert_eq!(result, 5);
    }
    #[test]
    fn div_first_test() {
        let result = div(10, 5);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn div_second_test() {
        let result = div(1, 2);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn div_third_test() {
        let result = div(1, 0);
        assert_eq!(result, None);
    }

    #[test]
    fn concat_first_test() {
        let result = concat(&"foo", &"bar");
        assert_eq!(result, "foo bar");
    }
}

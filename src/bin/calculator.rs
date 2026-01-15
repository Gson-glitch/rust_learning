/* UNIT TESTING BASICS:
    - #[cfg(test)]: Tells Rust to only compile this block when running 'cargo test'.
    - #[test]: Marks a function as a test case.
    - assert_eq!: Checks if two values are equal.
    - #[should_panic]: Ensures the code crashes as expected.
*/

pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero!");
    }
    a / b
}

fn main() {
    println!("Run 'cargo test --bin calculator' to see the tests in action!");
}

#[cfg(test)]
mod tests {
    use super::*; // Import functions from outer scope (this is needed for divide function to be used)

    #[test]
    fn test_basic_division() {
        assert_eq!(divide(10, 2), 5);
    }

    #[test]
    #[should_panic(expected = "Division by zero!")]
    fn test_divide_by_zero_panics() {
        divide(10, 0);
    }

    #[test]
    fn test_with_custom_message() {
        let result: i32 = divide(20, 4);
        // The 3rd argument is a custom error message for when the test fails
        assert_eq!(result, 5, "The calculator should return 5 for 20/4");
    }
}

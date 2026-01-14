/* ERROR HANDLING CORE:
    - panic!: Stops program immediately (unrecoverable).
    - Result<T, E>: Enum for data that might fail (Ok or Err).
    - unwrap/expect: Quick but dangerous ways to get data.
    - match: The safest way to handle both outcomes.
    - ?: Propagates errors up to the caller.
*/
fn main() {
    let nums: Vec<usize> = vec![1, 2, 3, 4];
    println!("Second element in nums: {:?}", nums[1]);
    // println!("Trying to access element at out of bounds index: {:?}", nums[99]);  // This panicks
    // Safely accessing elements
    let index: usize = 99;
    let element: Option<&usize> = nums.get(index);
    println!(
        "Trying to access element at out of bounds index: {:?}",
        element
    );

    // Explicitly using panic
    match element {
        Some(n) => println!("Element found: {}", n),
        // Using println! so the program won't crash
        None => println!("Index out of bounds!"),
        // None => panic!("Index out of bounds!"),  // This will crash the program with the error message
    }

    // Result<T, E>
    let a: f64 = 10.0;
    let b: f64 = 0.0;
    let res: Result<f64, String> = divide(a, b);
    println!("{} / {} = {:?}", a, b, res);

    let _ = run_division();
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    match b == 0.0 {
        true => Err(String::from("Error! Division by zero")),
        false => Ok(a / b),
    }
}

fn run_division() -> Result<(), String> {
    let a: f64 = 8.0;
    let b: f64 = 4.0;
    let res: Result<f64, String> = divide(a, b);
    println!("Division Successful! {} / {} = {:?}", a, b, res);
    Ok(())
}

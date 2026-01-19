/* Closures
    - Syntax: |param1, param2| { body }
    - Capturing: Closures can "see" variables in their parent scope.
    - Type Inference: You usually don't need to type the parameters or return value.
    - Use Case: Passing logic to iterators (map, filter) or storing short-lived logic.
*/

use std::vec;

fn main() {
    // Closure with no params and return values
    let say_hello = || println!("Hello");
    say_hello();

    // Closure with params
    let greet = |name: &str| println!("Hello, {}", name);
    greet("Gson");

    // Closure with params + return values
    let add = |a: i32, b: i32| a + b;
    println!("{} + {} = {}", 4, 4, add(4, 4));

    // Multi-line closures
    let x: i32 = 8;
    let compute = |a, b| {
        let result = a + b;
        result * x
    };
    println!("Result: {}", compute(4, 8));

    // Closure with Iterators (Most common use)
    let nums: Vec<i32> = vec![1, 2, 3, 4];
    let doubled: Vec<i32> = nums.iter().map(|n| n * 2).collect();
    println!("Doubled: {:?}", doubled);

    // Moving ownership of a variable
    let data: Vec<i32> = vec![1, 2, 3, 4];
    let take_ownership = move || println!("I now own: {:?}", data);
    take_ownership();
    // println!("{:?}", data);  // ❌ ERROR: borrow of moved value: `data`
}

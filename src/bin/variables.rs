fn main() {
    let x = 5;
    println!("The value of x is {}", x);

    // In Rust, variables defined using the `let` keyword are immutable by default
    // Therefore, reassigning x = 6 will throw an error
    // x = 6;

    // To go around this, we have to define the variable as mutable by adding `mut` after `let`
    let mut y = 2;
    println!("The value of y is {}", y);

    y = 4;
    println!("The new value of y is {}", y);

    // IMPORTANT: We can only mutate the value or a variable not the type.
    // So if we try to change the variable y to a string, Rust will throw an error.
    // y = "Hello, world";

    // Shadowing is basically re-initializing a variable with the same name
    let name = "Gson";
    println!("Name: {}", name);

    let name = "Glitch";
    println!("Shadowed name: {}", name);

    // PS: Shadowing is scoped. For example:
    {
        let name = "Gson-glitch";
        println!("Name from inner scope: {}", name);
    }

    println!("Name from outer scope: {}", name); // Prints Glitch from the outer scope

    // We can change types with shadowing
    let name = 2;
    println!("Name with int type: {}", name);

    // Constants - Must be SCREAMING_SNAKE_CASE and variable type must be provided.
    // Immutable meaning we can't shadow
    const MY_CONSTANT: f32 = 10_0000.44;
    println!("MY_CONSTANT: {}", MY_CONSTANT)
}

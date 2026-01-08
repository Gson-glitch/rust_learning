fn main() {
    /*
        In Rust, borrowing is the act of creating a reference to a value.
        It allows you to use data without taking ownership of it.
        Borrowing solves two main problems:
            1. Efficiency: It avoids copying large amounts of data (no need for .clone()).
            2. Reusability: It allows multiple parts of your code to access the same data without transferring ownership.
    */

    let mut s: String = String::from("hello"); // must use `mut` because .push_str() modifies inplace
    s.push_str(", world");
    println!("s: {}", s);

    /*
        Immutable References (&T)
        By default, borrows are immutable. You can read the data, but you cannot change it.
        You can have as many immutable borrows as you want at the same time.
    */
    let s1: String = String::from("hello");
    let s1_len: usize = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, s1_len);
    println!("s1: '{}' is still valid after function call!", s1);

    /*
        Mutable References (&mut T)
        If you need to change the data you are borrowing, you use a mutable reference.
        You can have only one mutable reference to a specific piece of data in a particular scope.
    */
    let mut s: String = String::from("Hello");
    let mut_ref_s: &mut String = &mut s;
    change(mut_ref_s); // Pass a mutable reference
    println!("Changed s: '{}'", s);

    // You cannot have a mutable reference if you already have immutable ones.
    let name: String = String::from("Gson");
    let name_ref1: &String = &name; // fine
    let name_ref2: &String = &name; // fine
    // let name_ref3: &mut String = &mut name; // ❌ ERROR! Cannot borrow as mutable because it's already borrowed as immutable.
    println!(
        "Name: {}, Ref1 Name: {}, Ref2 Name: {}",
        name, name_ref1, name_ref2
    );
}

fn calculate_length(s: &str) -> usize {
    // s is a reference to a String
    s.len()
} // s goes out of scope, but because it doesn't own the data, nothing is dropped.

fn change(s: &mut String) {
    s.push_str(", world!");
}

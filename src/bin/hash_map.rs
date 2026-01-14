/*
    Hashmaps in Rust are like dictionaries in Python.
    Hashing & Performance
        By default, Rust uses a cryptographically secure hashing algorithm (SipHash).
        It's resistant to DoS attacks but slightly slower.
        If you are doing extreme high-performance math, you might switch to 'FxHash'.

    Random Order
        HashMaps do NOT preserve the order you inserted things.
        If you run a loop over a map twice, the order might be different!
*/

use std::collections::HashMap;
fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    // Adding elements to a HashMap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Updating an existing element.
    scores.insert(String::from("Blue"), 25);

    // Checkinf for Membership
    println!(
        "scores contains key Blue?: {}",
        scores.contains_key(&String::from("Blue"))
    );

    // Accessing elements
    let team_name: String = String::from("Blue");

    // .get() returns Option<&V>. It doesn't crash if the key is missing!
    match scores.get(&team_name) {
        Some(score) => println!("Blue Team Score: {score}"),
        None => println!("Team not found!"),
    }

    // .entry (The "Industry Standard" for Updates)
    // "Check if 'Red' exists. If not, insert 0. Then give me a mutable reference to the value."
    let red_score: &mut i32 = scores.entry(String::from("Red")).or_insert(0);
    *red_score += 10; // We must dereference (*) to update the value in place

    // Iterating
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // The Gotchas!
    // Ownership
    let key: String = String::from("Favorite Color");
    let val: String = String::from("Rust Orange");
    let mut map: HashMap<String, String> = HashMap::new();

    map.insert(key, val);
    // println!("{}", key); // ❌ ERROR! 'key' was moved into the map.
    /*
        If you need to access the variables later after insertion, you have two options:
        1. Insert a clone
            map.insert(key.clone(), value);
            println!("{}", key);   // Now this works
        2. Use references
            let mut map<&String, &String> = Hashmap::new();
            map.insert(&key, &value);
            println!("{}", key);   // Now this works
    */

    // The Borrow Checker
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("A", 1);

    let r1: Option<&i32> = map.get("A"); // Immutable borrow starts
    // map.insert("B", 2); // ❌ ERROR! Cannot borrow `map` as mutable because it is also borrowed as immutable
    println!("{:?}", r1);
}

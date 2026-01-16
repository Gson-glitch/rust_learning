/* Generics, Traits, and Lifetimes in Rust:
    1. Generics (<T>): Code templates that work with any type.
    2. Traits (trait): Interfaces that define shared behavior (rules).
    3. Lifetimes ('a): Metadata ensuring references don't outlive their data.
*/

// Generics
// Let's say we want to define a Point that can be of any type (int, float, char).
// That's what generics do, they define a custom type.
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// We need 'PartialOrd' trait because not all types can be compared (e.g., how do you compare two colors?)
// We are passing the actual values (NOT references with &) as params
// because the args are scalar types (int, float, char) that implement the Copy trait.
fn find_largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// Traits
// Traits are like interfaces in Java.
trait Car {
    fn features(&self) -> String;
}

struct Mercedes {
    horse_power: f64,
    engine_cc: f64,
    year_of_manufacture: i32,
    make: String,
}

#[allow(clippy::upper_case_acronyms)]
struct BMW {
    horse_power: f64,
    engine_cc: f64,
    year_of_manufacture: i32,
    make: String,
    drive_terrain: String,
}

impl Car for Mercedes {
    fn features(&self) -> String {
        format!(
            "Mercedes AMG C63 Features:\n\tHP: {}, Engine CC: {}, YoM: {}, Make: {}",
            self.horse_power, self.engine_cc, self.year_of_manufacture, self.make
        )
    }
}

impl Car for BMW {
    fn features(&self) -> String {
        format!(
            "BMW X4M Competition Features:\n\tHP: {}, Engine CC: {}, YoM: {}, Make: {}, drive_terrain: {}",
            self.horse_power,
            self.engine_cc,
            self.year_of_manufacture,
            self.make,
            self.drive_terrain
        )
    }
}

// Lifetimes (Memory Safety)
// 'a ensures the returned reference is valid as long as both inputs are.
fn get_longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

fn main() {
    // Generics
    let point_int: Point<i32> = Point { x: 1, y: 2 };
    let point_float: Point<f64> = Point { x: 1.0, y: 2.0 };
    let point_char: Point<char> = Point { x: 'a', y: 'b' };
    let _point_string: Point<String> = Point {
        x: "hello".to_string(),
        y: "world".to_string(),
    };
    println!(
        "Points: {:?}, {:?}, {:?}",
        point_int, point_float, point_char
    );

    println!(
        "The largest integer between {} and {} is {}",
        point_int.x,
        point_int.y,
        find_largest(point_int.x, point_int.y)
    );

    // Traits
    let mercedes: Mercedes = Mercedes {
        horse_power: 503.0,
        engine_cc: 4000.0,
        year_of_manufacture: 2022,
        make: String::from("C 63 S Coupe"),
    };
    let bmw: BMW = BMW {
        horse_power: 503.0,
        engine_cc: 3000.0,
        year_of_manufacture: 2022,
        make: String::from("X4 M Competition"),
        drive_terrain: String::from("M xDrive (AWD)"),
    };
    println!("{}", mercedes.features());
    println!("{}", bmw.features());

    // Lifetimes
    let s1: &str = "abc";
    let longer_str: &str;
    {
        let s2: &str = "abcdef";
        longer_str = get_longer(s1, s2);
        println!("The longer string is {}", longer_str);
    } // s2 dies here, so trying to access it outside this scope would panic!
    // println!("{}", s2); ❌ ERROR: cannot find value `s2` in this scope
}

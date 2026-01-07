fn main() {
    /*
    Data Types in Rust can either be Scalar Types (int, String) or Compound (Array, Tuple)
    SCALAR TYPES
       - Integer
       - Float
       - Character
       - String
       - Boolean

    COMPOUND TYPES
       - Primitive Compound
            * Tuples
            * Arrays
       - User-Defined Compound
            * Struct
            * Enum
    */

    // Scalars
    let num_int: i8 = 8;
    let num_float: f32 = 10_000.54;
    let is_married: bool = true;
    let my_char: char = 'c';
    let name: &str = "Gson";

    println!(
        "Integer: {}, Float: {}, Boolean: {}, Char: {}, String: {}",
        num_int, num_float, is_married, my_char, name
    );

    // Primitive Compound Types
    // Tuples - Fixed size, Can mix different types
    let tup: (i8, f32, &str) = (25, 53.35, "John Doe");
    let full_name: &str = tup.2; // Access by index
    let (age, _, _) = tup; // Destructing

    println!("Full Name: {}, Age: {}", full_name, age);

    // Arrays - Fixed size, Must have same types
    let nums: [i8; 4] = [10, 20, 30, 40];
    let num_1: i8 = nums[0];

    println!("Nums: {:?}\nNums[0]: {}", nums, num_1);
    println!("Nums Elements:");
    for n in nums {
        println!("\tElement: {}", n);
    }

    // User-Defined Compound
    // Enums
    #[derive(Debug)]
    #[allow(dead_code)] // This tells the compiler: "I know I'm not using all of these yet."
    enum RoleLevel {
        Junior,
        Mid,
        Senior,
        TeamLead,
    }

    // Struct
    // #[derive(Debug)] // This allows the struct to be printed for debugging
    struct Employee {
        name: String,
        email: String,
        role: String,
        level: RoleLevel,
        age: i8,
    }

    let dev: Employee = Employee {
        name: String::from("Foo Bar"),
        email: String::from("foo.bar@example.com"),
        role: String::from("AI Researcher"),
        level: RoleLevel::Junior,
        age: 25,
    };

    println!("Developer Details");
    println!("\tName: {}", dev.name);
    println!("\tEmail: {}", dev.email);
    println!("\tRole: {}", dev.role);
    println!("\tLevel: {:?}", dev.level);
    println!("\tAge: {}", dev.age);
}

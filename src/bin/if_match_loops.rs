fn main() {
    // IF
    let level: &str = "junior";
    if level == "junior" {
        println!("You are a Junior Developer!")
    } else if level == "mid" {
        println!("You are a Middle Level Developer!")
    } else if level == "senior" {
        println!("You are a Senior Developer!")
    } else {
        println!("Congrats! You must be a Team Lead!")
    }
    // Match
    let is_married: bool = true;
    match is_married {
        true => println!("Married"),
        false => println!("Single"),
    }
    // Loops
    // For Loop
    // Exclusive range [..] (0, 1, 2, 3, 4)
    println!("Exclusive Range (0..5)");
    for i in 0..5 {
        println!("i: {}", i)
    }

    // Inclusive range [..=] (0, 1, 2, 3, 4, 5)
    println!("Inclusive Range (0..=5)");
    for i in 0..=5 {
        println!("i: {}", i)
    }

    let nums: [i8; 5] = [1, 2, 3, 4, 5];
    for n in nums {
        println!("Num: {}", n);
    }

    // While Loop
    let mut counter: i8 = 1;
    while counter <= 10 {
        println!("Counter: {}", counter);
        counter += 1;
    }

    // Loop - Loops forever until a condition is met. Returns a value
    let mut counter: i8 = 1;
    let loop_res: i8 = loop {
        if counter == 10 {
            break counter * 2; // if counter == 10, return 10*2 and break
        }
        counter += 1;
    };

    println!("Loop Result: {}", loop_res);

    // loop {}
    for n in 0..21 {
        if n % 3 == 0 && n % 5 == 0 {
            println!("FuzzyBuzz");
        } else if n % 3 == 0 {
            println!("Fuzzy");
        } else if n % 5 == 0 {
            println!("Buzz")
        } else {
            println!("N: {}", n);
        }
    }
}

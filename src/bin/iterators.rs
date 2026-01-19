fn main() {
    // .iter() - Returns an immutable reference (read-only view)
    println!(".iter()");
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    for n in nums.iter() {
        println!("\tNum: {}", n)
    }

    // .iter_mut() - Returns a mutable reference (read and write) - Used to modify an iterable in place
    println!(".iter_mut()");
    let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Nums before looping: {:?}", nums);
    for n in nums.iter_mut() {
        *n += 1 // we must dereference using * before updating
    }
    println!("Nums after looping: {:?}", nums);

    // .into_iter() - This takes ownership of the iterable. Similar to directly using nums without reference
    println!(".into_iter()");
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    for n in nums.into_iter() {
        println!("\tNum: {}", n);
    }
    // println!("{:?}", nums); // ❌ ERROR: borrow of moved value: `nums`

    // Functional programming in Rust
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    // .map()
    println!(".map()");
    let doubled: Vec<_> = nums.iter().map(|&x| x * 2).collect();
    println!("Doubled Nums: {:?}", doubled);

    // .filter()
    println!(".filter()");
    let even: Vec<_> = nums.iter().filter(|&x| x % 2 == 0).collect();
    println!("Even Nums: {:?}", even);

    // .fold() - Similar to .reduce() in Python
    println!(".fold()");
    #[allow(clippy::unnecessary_fold)]
    let sum_nums: i32 = nums.iter().fold(0, |acc, &x| acc + x);
    println!("Sum of Nums: {}", sum_nums);
}

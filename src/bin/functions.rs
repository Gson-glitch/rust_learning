fn add(num_1: i8, num_2: i8) {
    let res: i8 = num_1 + num_2;
    println!("{} + {} = {}", num_1, num_2, res);
}

fn sub(num_1: i8, num_2: i8) -> i8 {
    let res: i8 = num_1 - num_2;
    res
}

fn main() {
    let num_1: i8 = 1;
    let num_2: i8 = 3;

    add(num_1, num_2);
    println!("{} - {} = {}", num_1, num_2, sub(num_1, num_2));
}

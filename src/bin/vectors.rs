/*
    Vectors are arrays that can have dynamic sizes
*/
#[derive(Debug)]
enum Transaction {
    Deposit(f64),
    Withdrawal(f64),
    Interest(f64),
}

fn main() {
    let mut scores: Vec<i32> = Vec::new();
    println!("Adding elements to a vector");
    scores.push(10);
    scores.push(20);
    scores.push(30);
    scores.push(40);
    println!("First element in scores: {}", &scores[0]);
    println!("Scores: {:?}", scores);
    println!("Removing elements from a vector");
    scores.pop(); //Removes element in the last index
    scores.remove(0); // Index to remove
    println!("Scores: {:?}", scores);
    println!("Length of scores: {:?} is {}", scores, scores.len());
    println!("Is scores empty? {}", scores.is_empty());

    // A vector of Transaction enums
    let mut history: Vec<Transaction> = vec![
        Transaction::Deposit(20.0),
        Transaction::Withdrawal(10.0),
        Transaction::Interest(8.0),
    ];

    history.push(Transaction::Deposit(80.0));

    for trans in &history {
        match trans {
            Transaction::Deposit(amt) => println!("Deposited: {}", amt),
            Transaction::Withdrawal(amt) => println!("Withdrew: {}", amt),
            Transaction::Interest(amt) => println!("Earned interest: {}", amt),
        }
    }
    println!("{:?}", history);
}

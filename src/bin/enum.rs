/*
    Enums with implementation.
*/
#[allow(dead_code)]
enum RoleLevel {
    Junior,
    Mid,
    Senior,
}

impl RoleLevel {
    fn description(&self) -> String {
        match self {
            Self::Junior => String::from("Entry-level developer learning the ropes."),
            Self::Mid => String::from("Independent contributor handling complex tasks."),
            Self::Senior => String::from("Tech lead overseeing architecture and mentoring."),
        }
    }
}

/*
    Enums with Data.
*/
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Shutting down..."),
            Message::Move { x, y } => println!("Moving to x: {}, y: {}.", x, y),
            Message::Write(text) => println!("Text message: {}", text),
        }
    }
}

/*
    Option enum
    Rust does not have null. Instead, it uses the Option<T> enum.
    What is Option<T>?
        Option is a standard library enum defined like this:
        enum Option<T> {
            None,    // Represents None (absence of a value)
            Some(T), // Represents "something" of type T (can be any data type)
        }
*/

fn greet(name: Option<String>) {
    match name {
        Some(n) => println!("Hello, {}!", n),
        None => println!("Hello stranger"),
    }
}

fn main() {
    let role: RoleLevel = RoleLevel::Junior;
    println!("Role: {}", role.description());

    let msg: Message = Message::Move { x: 4, y: 8 };
    msg.call();

    let name: Option<String> = Some(String::from("Gson"));
    let no_name: Option<String> = None;
    greet(name);
    greet(no_name);
}

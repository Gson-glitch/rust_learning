struct Rectangle {
    width: f32,
    height: f32,
}

// Fields (Attributes) in Rust
struct Player {
    name: String, // field
    stats: f32,
}

// In Rust, impl (implementation) is the keyword used to define the behavior for a specific type.
// Think of it this way, if struct is a blueprint for data, then impl is the actual definition
// incorporating methods, attributes, etc. all which belong to that specific struct
impl Player {
    // Associated Functions
    // This are equivalents to @classmethods in Python.
    // Usually used for constructors (like new).
    // Called by the :: notation e.g Player::new(...)
    fn new(name: String, stats: f32) -> Self {
        Self { name, stats }
    }

    // Methods (instance methods, must have self as the first parameter)
    // Called by the . notation, e.g Player.get_stats()
    fn get_stats(&self) -> &f32 {
        &self.stats
    }

    fn print_player(&self) {
        println!(
            "Player Name: {}\nPlayer Stats: {}\n",
            self.name,
            self.get_stats()
        );
    }
}

fn main() {
    #[derive(Debug)]
    struct Point {
        x: i8,
        y: i8,
    }

    #[derive(Debug)]
    struct Color {
        r: f32,
        g: f32,
        b: f32,
    }

    let coord: Point = Point { x: 4, y: 8 };
    let red: Color = Color {
        r: 255.0,
        g: 0.0,
        b: 0.0,
    };

    println!("Coordinates: {:?}\nx: {}, y: {}", coord, coord.x, coord.y);
    println!(
        "Color Red: {:?}\nr: {}, g: {}, b: {}",
        red, red.r, red.g, red.b
    );

    // Area of Rectangle
    let rec1: Rectangle = Rectangle {
        width: 20.0,
        height: 40.0,
    };

    println!("The area of 'rec1' = {}", calc_area(&rec1));
    println!("'rec1' dimensions: {}, {}", rec1.width, rec1.height);

    // Player Stats
    let player1: Player = Player::new(String::from("John Doe"), 90.4);
    player1.print_player();
}

fn calc_area(rec: &Rectangle) -> f32 {
    rec.width * rec.height
}

fn main() {
    /*
        In Rust, Slices are a special kind of reference that let you refer to a contiguous sequence
        of elements in a collection rather than the entire collection itself. Think of it like
        slicing in Python.
        In Rust, slices are references.
    */
    let s: String = String::from("hello world");
    let slice_s1: &str = &s[0..5]; // same as ..5
    let slice_s2: &str = &s[6..11]; // same as 6..
    // let slice = s[..5]; // ❌ ERROR: "size for values of type `str` cannot be known at compilation time"

    println!(
        "Original String:  {}, slice_1: {}, slice_2: {}",
        s, slice_s1, slice_s2
    );

    let vector: Vec<i8> = vec![10, 20, 30, 40, 50];
    let slice: &[i8] = &vector[1..4];

    println!("Vector slice: {:?}", slice);
}

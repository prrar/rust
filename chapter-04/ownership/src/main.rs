fn main() {
    let mut s1 = String::from("hello");
    let s2 = s1.clone();
    s1 = String::from("bye");
    println!("s1 = {s1}, s2 = {s2}");

    let mut x = 5;
    let y = x;
    {
        x = 10;
    }
    println!("x = {x}, y = {y}");

    let s3 = String::from("hello");
    let (s4, len) = calculate_length(s3);
    println!("The length of '{s4}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
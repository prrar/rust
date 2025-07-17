fn main() {
    let myword = String::from("hello world");
    let x = first_word(&myword);
    println!("{x}");
    let x = "paulo rogerio";
    let x = first_word(x);
    println!("{x}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
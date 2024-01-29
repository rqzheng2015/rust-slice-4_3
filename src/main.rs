fn main() {
    let my_string = String::from("hello world");
    let word = first_world(&my_string[..]);
    // let word = first_world(&s).to_string(); // Copy the first word
    // my_string.clear();
    // print out the first word index
    println!("{}", word);
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return the first word
            return &s[0..i];
        }
    }
    // return whole string
    &s[..]
}

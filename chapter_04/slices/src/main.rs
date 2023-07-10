// Consider the function below, returning index of the end of the first word
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }

// Sample usage (problem)
// fn main() {
//     let mut s = String::from("hello world");
//
//     let word = first_word(&s); // word will get the value 5
//
//     s.clear(); // this empties the String, making it equal to ""
//
//     // word still has the value 5 here, but there's no more string that
//     // we could meaningfully use the value 5 with. word is now totally invalid!
//     // want some type that represents part of a string
// }

fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let mut s2 = String::from("hello");
    s2.push_str(" world");
    let first = first_word(&s2);
    // s2.clear(); // won't work: mut borrow alongside immut borrow
    println!("{}", first);

    // string literals are slices pointing to some part of the binary
    let string_literal: &str = "hello world";
    println!("{}", string_literal);

    // array slice
    let a = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// fn first_word(s: &String) -> &str { // &str: string slice
fn first_word(s: &str) -> &str { // preferred signature, since &str can be a &String or string literal
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

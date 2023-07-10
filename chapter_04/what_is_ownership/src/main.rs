fn main() {
    // mutable strings on heap
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // heap objects get moved
    let s1 = String::from("hello");
    let s2 = s1; // s2 owns the heap object now, s1 invalidated
    // println!("{}, world!", s1); // won't work
    println!("{}, world!", s2);
    let s3 = s2.clone(); // deep copy (copies heap memory)
    println!("{}, world!", s3);

    let x = 5;
    let y = x; // copied since i32 has Copy trait
    println!("{} {}", x, y); // fine since copied

    let s4 = String::from("hello");
    takes_ownership(s4);
    // s4 invalidated
    // println!("{}", s4); // won't work
    // takes_ownership(s4); // won't work

    makes_copy(x); // i32 has Copy trait
    println!("{}", x); // still valid
    makes_copy(x); // still valid

    let s5 = gives_ownership(); // ownership of string given by return
    let s6 = takes_and_gives_back(s5); // ownership of string given by return
    // println!("{}", s5); // s5 invalidated, won't work
    println!("{}", s6);

    let mut s6 = gives_ownership(); // mut so we can reassign on next line
    s6 = takes_and_gives_back(s6); 
    println!("{}", s6);
    
    // passing back ownership for every parameter is annoying/clunky
    // next section: references
    let (s7, len) = calculate_length(s6);
    println!("The length of '{}' is {}.", s7, len);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

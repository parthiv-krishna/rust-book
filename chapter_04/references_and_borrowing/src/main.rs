fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // must declare mut since `change` mutates
    let mut s2 = String::from("hello");
    change(&mut s2); // reference also must explicitly be mut
    println!("{}", s2);


    // REFERENCES: single mut OR multiple immutable
    let mut s = no_dangle();

    // works: single mut live at a time
    let r1 = &mut s;
    println!("{}", r1);
    let r2 = &mut s;
    println!("{}", r2);

    // won't work: cannot have two live &mut
    // println!("{}, {}", r1, r2);

    // works: can have multiple & live
    let r3 = &s;
    let r4 = &s;
    // let r5 = &mut s; // won't work: cannot have &mut live with &
    println!("{} {}", r3, r4);

}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a string
    s.len()
    // s does not own the string so string is not dropped
}

fn change(some_string: &mut String) { // won't compile without mut (mutates object)
    some_string.push_str(", world");
}


// won't work: dangling reference
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// fine, transfers ownership
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
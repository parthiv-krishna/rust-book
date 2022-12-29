use std::io;

fn floats() {
    println!("floats");
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    dbg!(x);
    dbg!(y);
}

fn operations() {
    println!("operations");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    dbg!(sum, difference, product, quotient, truncated, remainder);
}

fn bools() {
    println!("bools");

    let t = true;
    let f: bool = false;

    dbg!(t, f);
}


fn chars() {
    println!("chars");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    dbg!(c, z, heart_eyed_cat);
}

fn tuples() {
    println!("tuples");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    dbg!(x);
    dbg!(x.0, x.1, x.2);
}

fn arrays() {
    println!("arrays");
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [3; 5];

    dbg!(a, b);
    dbg!(a[4], b[0]);
}

fn invalid_array_access() {
    println!("invalid_array_access");
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}

fn main() {
    floats();
    operations();
    bools();
    chars();
    tuples();
    arrays();
    invalid_array_access();
}

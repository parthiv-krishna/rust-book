fn main() {
    // don't need to specify type, i32 inferred as default int
    let v1 = vec![1, 2, 3]; // vec! macro to create a vector with some values

    // mut in order to push
    // need to specify type since it cannot be inferred
    let mut v2: Vec<i32> = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);


    // 2 ways to get elements from a vector
    let v3 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v3[2]; // will crash program if OOB
    println!("The third element is {third}");
    let third: Option<&i32> = v3.get(2); // will give None if OOB
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v3[100]; // will panic/crash
    let does_not_exist = v3.get(100);
    // does_not_exist will be None. more graceful handling

    // mut and non-mut refs cannot exist at the same time
    let mut v4 = vec![1, 2, 3, 4, 5];
    let first = &v4[0]; // immutable reference
    // v4.push(6); // requires a mutable reference
    println!("The first element is: {first}");
    v4.push(6); // works here

    // immutable for loop
    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{i}");
    }

    // mutable for loop
    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 {
        *i += 50; // need to deref to access 
    }

    // in both for loops, borrow checker validates that we don't
    // add or remove elements. the reference in for loop init
    // owns the whole vector, so prevents modification
    // of the whole vector (just allows modifying references)
    // for i in &v6 {
    //     v6.push(5);
    // }    

    // you can store elements of different types with enums
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // can use match to determine the subtype that you are working with

}
// vectors (and their elements) get dropped when they go out of scope
// borrow checker will validate that all references to elements in the
// vector are only used when the vector itself is valid

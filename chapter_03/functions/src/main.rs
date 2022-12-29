fn blocks() {
    println!("blocks");
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // no semicolon (would make it a "statement" rather than an "expression")
    // statement: has no value
    // expression: has a value
    x + 1 
}

fn simple_functions() {
    let x = five();

    println!("The value of x is: {x}");

    let six = plus_one(five());
    dbg!(six);
}


fn main() {
    blocks();
    simple_functions();
}

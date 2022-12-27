const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn mut_declaration() {
    println!("mut_declaration");

    // needs to be declared as mut, otherwise x = 6 will error at compile time
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn constant() {
    println!("constant");
    println!("{}", THREE_HOURS_IN_SECONDS);
}

fn shadowing() {
    println!("shadowing");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}


fn main() {
    mut_declaration();
    constant();
    shadowing();
}
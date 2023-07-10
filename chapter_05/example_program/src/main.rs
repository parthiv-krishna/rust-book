#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // bad: these variables are not obviously related
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area_separate(width1, height1)
    );

    // better: tuple, so they're associated at least
    let tup = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(tup)
    );

    // best: named struct so it's clear what the data is
    let rect = Rectangle {
        width: 30, 
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_rectangle(&rect)
    );
    
    // works because we derive the Debug trait: #[derive(Debug)]
    // to use Debug, do {:?}
    println!("rect is {:?}", rect);
    println!("rect is {:#?}", rect); // nicer spacing

    dbg!(rect); // takes ownership, prints file/lineno/debug info, returns ownership
}

fn area_separate(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_rectangle(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

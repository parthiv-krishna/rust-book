use crate::garden::vegetables::Asparagus;

pub mod garden; // include code from src/garden.rs

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
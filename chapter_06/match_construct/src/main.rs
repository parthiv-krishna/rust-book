#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Option<UsState>),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            match state {
                Some(state) => {
                    println!("State quarter from {:?}!", state)
                }
                None => () // do nothing
                // other options to make exhaustive:
                // _ => println!("Boring quarter!")
                // var_name => do_smth_with(var_name) // in this case, would always be None
            };
            25
        }
    }
}


fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter1 = Coin::Quarter(Some(UsState::Alabama));
    let quarter2 = Coin::Quarter(Some(UsState::Alaska));
    let quarter3 = Coin::Quarter(None);

    print_coin_value(penny);
    print_coin_value(nickel);
    print_coin_value(dime);
    print_coin_value(quarter1);
    print_coin_value(quarter2);
    print_coin_value(quarter3);
}

fn print_coin_value(coin: Coin) {
    println!("{} cents", value_in_cents(coin));
}
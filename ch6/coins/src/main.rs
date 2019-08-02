enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}

fn plus_one(x: Option<i32>) -> i32 {
    match x {
        None => 0,
        Some(i) => i + 1
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Dime));
    println!("{:?}", plus_one(Some(6)));
    println!("{:?}", plus_one(None));
}

fn five() -> i32 {
    5
}

fn main() {
    let y = 6;
    println!("Hello, world!");
    let x = five();

    another_function(x, y);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {} and the value of y is {}", x, y);
}

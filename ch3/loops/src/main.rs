fn main() {
    loop_loop();
    while_loop();
    for_loop();
}

fn loop_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("Liftoff!");
}

fn for_loop() {
    let arr = [10, 20, 30, 40, 50];

    for num in arr.iter() {
        println!("the number is {}", num);
    }
}

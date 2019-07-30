fn main() {
    let s1 = String::from("Hello");
    let mut s2 = String::from("Hi there");

    let len = calculate_length(&s1);
    change(&mut s2);

    println!("The length of '{}' is {}", s1, len);
    println!("{}", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" developers!");
}

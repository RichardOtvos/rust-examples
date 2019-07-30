fn main() {
    let s1 = String::from("Hello there!");
    println!("{}", first_word(&s1));
    println!("{}", first_word("ahoy there"));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }

    return &s[..]
}

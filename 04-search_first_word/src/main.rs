fn main() {
    let text = String::from("Hello world!");
    let word = find_word(&text);
    assert_eq!(word, "Hello");

    println!("The first word is {}", word);
}

fn find_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

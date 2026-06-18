fn position(c: char) -> String {
    let s: String = ('a'..='z').collect();
    for (ind, el) in s.chars().enumerate() {
        if c == el {
            return format!("Position of alphabet: {}", ind + 1);
        }
    }
    "".to_string()
}
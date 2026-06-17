fn contamination(text: &str, character: &str) -> String {
    if text.is_empty() || character.is_empty() {
        return "".to_string();
    }
    let length: u32 = text.len() as u32;
    let mut ans: String = "".to_string();
    for _i in 1..=length {
        ans += &character.to_string();
    }
    ans
}

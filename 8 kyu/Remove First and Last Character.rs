pub fn remove_char(s: &str) -> String {
    let length: u32 = s.len() as u32;
    if length == 2 {
        return "".to_string();
    }
    s[1..s.len() - 1].to_string()
}

fn smash(words: &[&str]) -> String {
    let length: u32 = words.len() as u32;
    let mut ans: String = "".to_string();
    for ind in 0..length {
        if ind < length - 1 {
            ans += &words[ind as usize].to_string();
            ans += &" ".to_string();
        } else {
            ans += &words[ind as usize].to_string();
        }
    }
    ans
}
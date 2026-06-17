fn bin_to_decimal(inp: &str) -> i32 {
    let length: u32 = inp.len() as u32;
    let mut ans: u32 = 0;
    for (ind, c) in inp.chars().enumerate() {
        if let Some(res) = c.to_digit(10) {
            ans += res * 2u32.pow(length - ind as u32 - 1);
        };
    }
    ans as i32
}
fn feast(beast: &str, dish: &str) -> bool {
    let mut b_chars = beast.chars();
    let b_first = b_chars.next().unwrap_or(' ');
    let b_last = b_chars.last().unwrap_or(b_first);
    let b_ans = format!("{}{}", b_first, b_last);

    let mut d_chars = dish.chars();
    let d_first = d_chars.next().unwrap_or(' ');
    let d_last = d_chars.last().unwrap_or(d_first);
    let d_ans = format!("{}{}", d_first, d_last);

    b_ans == d_ans
}
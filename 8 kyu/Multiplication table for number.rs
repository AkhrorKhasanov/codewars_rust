fn multi_table(n: u64) -> String {
    let mut ans = String::new();
    for i in 1..=10 {
        let line = format!("{} * {} = {}", i, n, i * n);
        ans.push_str(&line);
        if i < 10 {
            ans.push_str("\n");
        }
    }
    ans
}
fn positive_sum(slice: &[i32]) -> i32 {
    let mut res: i32 = 0;
    for s in slice.iter() {
        if s > &0 {
            res += s;
        }
    }
    res
}
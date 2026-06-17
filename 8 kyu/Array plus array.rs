fn slice_plus_slice(xs: &[i32], ys: &[i32]) -> i32 {
    let mut res: i32 = 0;
    for x in xs.iter() {
        res += x;
    }
    for y in ys.iter() {
        res += y;
    }
    res
}
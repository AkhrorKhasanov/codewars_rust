fn grow(nums: Vec<i32>) -> i32 {
    let mut res: i32 = 1;
    for num in nums.iter() {
        res *= num;
    }
    res
}
fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    let mut pos: i32 = 0;
    let mut neg: i32 = 0;
    for num in input.iter() {
        if num > &0 {
            pos += 1;
        } else {
            neg += num;
        }
    }
    if input.is_empty() {
        return [].to_vec();
    }
    [pos, neg].to_vec() 
}
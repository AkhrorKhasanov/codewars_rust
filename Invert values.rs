fn invert(values: &[i32]) -> Vec<i32> {
    if values.is_empty() {
        return Vec::new();
    }
    let mut new_values: Vec<i32> = Vec::new();
    let length: u32 = values.len() as u32;
    for ind in 0..length {
        new_values.push(-values[ind as usize]);
    }
    new_values
}
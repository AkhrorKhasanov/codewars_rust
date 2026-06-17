fn get_average(marks: &[i32]) -> i32 {
    (marks.iter().sum::<i32>() as f32 / marks.len() as f32).floor() as i32
}
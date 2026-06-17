fn better_than_average(class_points: &[u16], your_points: u16) -> bool {
    let mut ans: u16 = your_points;
    let mut n: u16 = 1; 
    for class_point in class_points.iter() {
        ans += class_point;
        n += 1;
    }
    your_points as f32 > ans as f32 / n as f32
}
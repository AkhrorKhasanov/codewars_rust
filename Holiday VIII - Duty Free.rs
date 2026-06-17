fn duty_free(price: i32, discount: i32, holiday_cost: i32) -> i32 {
    (holiday_cost as f32 / (price as f32 * 0.01 * discount as f32)).floor() as i32
}
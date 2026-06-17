fn combat(health: f32, damage: f32) -> f32 {
    if health - damage > 0.0 {
        return health - damage;
    }
    0.0 
}
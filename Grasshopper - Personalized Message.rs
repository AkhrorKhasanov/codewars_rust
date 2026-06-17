fn greet(name: &str, owner: &str) -> String {
    if name == owner {
        return "Hello boss".to_string();
    }
    "Hello guest".to_string()
}
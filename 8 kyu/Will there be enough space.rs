fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    let answer = cap - on - wait;
    if answer < 0 {
        return -answer;
    } else {
        return 0;
    }
}
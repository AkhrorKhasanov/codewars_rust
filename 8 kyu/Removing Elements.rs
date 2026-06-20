fn remove_every_other(arr: &[u8]) -> Vec<u8> {
    arr.iter().enumerate().filter(|(i, _)| i % 2 != 1).map(|(_, &val)| val).collect()
}

fn difference_in_ages(ages: &[u8]) -> (u8, u8, u8) {
    let mini: u8 = *ages.iter().min().unwrap();
    let maks: u8 = *ages.iter().max().unwrap();
    let dif: u8 = maks - mini;
    (mini, maks, maks - mini)
}
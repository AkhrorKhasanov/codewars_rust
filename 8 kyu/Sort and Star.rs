fn two_sort(arr: &[&str]) -> String {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    let first = &sorted_arr[0];
    first.chars().map(|c| c.to_string()).collect::<Vec<String>>().join("***")
}
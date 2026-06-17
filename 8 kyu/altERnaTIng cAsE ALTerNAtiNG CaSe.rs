fn to_alternating_case(s: &str) -> String {
  let new_str: String = s.chars().map(|c| {
    if c.is_uppercase() {
        c.to_lowercase().next().unwrap()
    } else {
        c.to_uppercase().next().unwrap()
    }
  }).collect();
  return new_str;
}
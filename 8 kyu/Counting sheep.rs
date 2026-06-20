fn count_sheep(sheep: &[bool]) -> u8 {
  sheep.iter().filter(|&&e| e).count() as u8
}
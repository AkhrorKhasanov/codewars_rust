fn no_space(x : String) -> String{
  x.chars().filter(|&c| c != ' ').collect()
}
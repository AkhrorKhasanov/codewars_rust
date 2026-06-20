fn reverse_words(words: &str) -> String {
    words.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}
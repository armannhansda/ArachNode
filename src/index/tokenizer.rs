pub fn tokenizer(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|word| word.to_lowercase())
        .filter(|word| word.len() > 2)
        .collect()
}

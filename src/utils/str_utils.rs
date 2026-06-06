pub fn remove_words(text: &str, words: &[&str]) -> String {
    let mut text = text.to_string();
    words.iter().for_each(|w| text = text.replace(*w, ""));
    text
}
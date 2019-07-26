use std::collections::HashMap;

pub fn most_common(paragraph: String, banned: Vec<&str>) -> &str {
    let mut count_words = HashMap::new();

    paragraph.split_whitespace().for_each(|word| {
        let count = count_words.entry(word).or_insert(0);
        *count += 1;
    });

   
    "hola"
}

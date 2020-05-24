use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    let mut word_chars = Vec::new();
    for x in word.to_lowercase().chars() {
        word_chars.push(x);
    }

    let mut result: HashSet<&'a str> = HashSet::new();
    for item in possible_anagrams.to_owned() {
        if item.len() != word.len() || item.to_lowercase() == word || item.to_uppercase() == word.to_uppercase() {
            continue
        }

        let mut tmp_word_chars = word_chars.clone();
        let mut valid = true;
        for letter in item.to_lowercase().chars() {
            if !tmp_word_chars.contains(&letter) {
                valid = false;
                break;
            } else {
                let mut index_to_remove: Option<usize> = None;
                for (index, value) in tmp_word_chars.iter().enumerate() {
                    if *value == letter {
                        index_to_remove = Some(index);
                        break;
                    }
                }
                if let Some(x) = index_to_remove {
                    tmp_word_chars.remove(x);
                }
            }
        }

        if valid {
            result.insert(item);
        }
    }

    result
}
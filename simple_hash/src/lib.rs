use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut map = HashMap::new();

    for &word in words {
        *map.entry(word).or_insert(0) += 1;
    }
    map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_word() {
        let input = vec!["hello"];
        let freq = word_frequency_counter(&input);
        assert_eq!(freq.get("hello"), Some(&1));
        assert_eq!(nb_distinct_words(&freq), 1);
    }

    #[test]
    fn test_multiple_words() {
        let input = vec!["this", "is", "a", "test", "this", "test"];
        let freq = word_frequency_counter(&input);
        assert_eq!(freq.get("this"), Some(&2));
        assert_eq!(freq.get("test"), Some(&2));
        assert_eq!(freq.get("is"), Some(&1));
        assert_eq!(nb_distinct_words(&freq), 4);
    }

    #[test]
    fn test_empty_input() {
        let input: Vec<&str> = vec![];
        let freq = word_frequency_counter(&input);
        assert!(freq.is_empty());
        assert_eq!(nb_distinct_words(&freq), 0);
    }

    #[test]
    fn test_punctuation() {
        let input = vec!["hello,", "world!", "hello,"];
        let freq = word_frequency_counter(&input);
        assert_eq!(freq.get("hello,"), Some(&2));
        assert_eq!(freq.get("world!"), Some(&1));
        assert_eq!(nb_distinct_words(&freq), 2);
    }

    #[test]
    fn test_unicode_words() {
        let input = vec!["café", "café", "niño", "niña"];
        let freq = word_frequency_counter(&input);
        assert_eq!(freq.get("café"), Some(&2));
        assert_eq!(freq.get("niño"), Some(&1));
        assert_eq!(freq.get("niña"), Some(&1));
        assert_eq!(nb_distinct_words(&freq), 3);
    }
}

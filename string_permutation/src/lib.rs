use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut freq1 = HashMap::new();
    for c in s1.chars() {
        *freq1.entry(c).or_insert(0) += 1;
    }

    let mut freq2 = HashMap::new();
    for c in s2.chars() {
        *freq2.entry(c).or_insert(0) += 1;
    }

    freq1 == freq2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations_true() {
        assert!(is_permutation("abc", "cba"));
        assert!(is_permutation("aabbcc", "baccab"));
        assert!(is_permutation("123", "321"));
        assert!(is_permutation("", ""));
        assert!(is_permutation("thought", "thougth"));
    }

    #[test]
    fn test_permutations_false() {
        assert!(!is_permutation("abc", "abcd"));
        assert!(!is_permutation("abc", "abx"));
        assert!(!is_permutation("abc", ""));
        assert!(!is_permutation("aabbcc", "abccba "));
        assert!(!is_permutation("thought", "thoughts"));
    }
}

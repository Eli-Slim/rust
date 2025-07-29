pub fn edit_distance(source: &str, target: &str) -> usize {
    let s1: Vec<char> = source.chars().collect();
    let s2: Vec<char> = target.chars().collect();
    let len1 = s1.len();
    let len2 = s2.len();

    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        for j in 0..=len2 {
            dp[i][j] = if i == 0 {
                j
            } else if j == 0 {
                i
            } else if s1[i - 1] == s2[j - 1] {
                dp[i - 1][j - 1]
            } else {
                1 + std::cmp::min(
                    dp[i - 1][j - 1],
                    std::cmp::min(
                        dp[i][j - 1],
                        dp[i - 1][j],
                    ),
                )
            };
        }
    }

    dp[len1][len2]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_strings() {
        assert_eq!(edit_distance("rust", "rust"), 0);
    }

    #[test]
    fn test_insertions() {
        assert_eq!(edit_distance("cat", "cart"), 1);
        assert_eq!(edit_distance("", "hello"), 5);
    }

    #[test]
    fn test_deletions() {
        assert_eq!(edit_distance("crate", "cat"), 2);
        assert_eq!(edit_distance("world", ""), 5);
    }

    #[test]
    fn test_substitutions() {
        assert_eq!(edit_distance("kitten", "sitten"), 1);
        assert_eq!(edit_distance("flaw", "flap"), 1);
    }

    #[test]
    fn test_mixed_operations() {
        assert_eq!(edit_distance("intention", "execution"), 5);
        assert_eq!(edit_distance("alignment", "assignment"), 2);
        assert_eq!(edit_distance("gumbo", "gambol"), 2);
    }

    #[test]
    fn test_case_sensitive() {
        assert_eq!(edit_distance("Rust", "rust"), 1);
    }

    #[test]
    fn test_unicode_characters() {
        assert_eq!(edit_distance("café", "coffee"), 4);
    }
}

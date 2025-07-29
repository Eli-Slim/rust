use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    h.values()
        .filter(|&&v| v > 0)
        .max()
        .cloned()
        .unwrap_or(0)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_bigger_with_positive_numbers() {
        let hash = HashMap::from([
            ("Daniel", 122),
            ("Ashley", 333),
            ("Katie", 334),
            ("Robert", 14),
        ]);
        assert_eq!(bigger(hash), 334);
    }

    #[test]
    fn test_bigger_with_all_negative_numbers() {
        let hash = HashMap::from([
            ("A", -10),
            ("B", -20),
            ("C", -30),
        ]);
        // No positive number, expect 0
        assert_eq!(bigger(hash), 0);
    }

    #[test]
    fn test_bigger_with_mixed_numbers() {
        let hash = HashMap::from([
            ("A", -10),
            ("B", 20),
            ("C", 15),
            ("D", 0),
        ]);
        assert_eq!(bigger(hash), 20);
    }

    #[test]
    fn test_bigger_with_empty_hashmap() {
        let hash: HashMap<&str, i32> = HashMap::new();
        assert_eq!(bigger(hash), 0);
    }

    #[test]
    fn test_bigger_with_zero_only() {
        let hash = HashMap::from([
            ("A", 0),
            ("B", 0),
        ]);
        assert_eq!(bigger(hash), 0);
    }
}

use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec();
    sorted.sort_unstable();
    let len = sorted.len();
    if len % 2 == 0 {
        let mid1 = sorted[len / 2 - 1];
        let mid2 = sorted[len / 2];
        (mid1 + mid2) / 2
    } else {
        sorted[len / 2]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut freq_map = HashMap::new();
    for &num in list {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut mode = list[0];

    for (&num, &count) in &freq_map {
        if count > max_count {
            max_count = count;
            mode = num;
        }
    }

    mode
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_basic() {
        let list = [4, 7, 5, 2, 5, 1, 3];
        let result = mean(&list);
        let expected = 3.857142857142857;
        let tolerance = 1e-9;
        assert!((result - expected).abs() < tolerance);
    }

    #[test]
    fn test_mean_single() {
        let list = [10];
        assert_eq!(mean(&list), 10.0);
    }

    #[test]
    fn test_median_odd() {
        let list = [1, 3, 2];
        assert_eq!(median(&list), 2);
    }

    #[test]
    fn test_median_even() {
        let list = [1, 4, 2, 3];
        assert_eq!(median(&list), (2 + 3) / 2);
    }

    #[test]
    fn test_median_sorted() {
        let list = [10, 20, 30, 40, 50];
        assert_eq!(median(&list), 30);
    }

    #[test]
    fn test_mode_basic() {
        let list = [1, 2, 2, 3, 4];
        assert_eq!(mode(&list), 2);
    }

    #[test]
    fn test_mode_multiple_max() {
        let list = [1, 1, 2, 2];
        let result = mode(&list);
        assert!(result == 1 || result == 2); // any valid mode is acceptable
    }

    #[test]
    fn test_mode_single() {
        let list = [42];
        assert_eq!(mode(&list), 42);
    }
}

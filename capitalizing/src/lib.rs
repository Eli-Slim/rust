pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

pub fn title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| capitalize_first(word))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_uppercase().collect::<String>()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_first() {
        assert_eq!(capitalize_first("hello"), "Hello");
        assert_eq!(capitalize_first("world"), "World");
        assert_eq!(capitalize_first(""), "");
        assert_eq!(capitalize_first("r"), "R");
        assert_eq!(capitalize_first("ßharp"), "SSharp"); // ß becomes SS when uppercased
    }

    #[test]
    fn test_title_case() {
        assert_eq!(title_case("hello world"), "Hello World");
        assert_eq!(title_case("jill is leaving A"), "Jill Is Leaving A");
        assert_eq!(title_case(""), "");
        assert_eq!(title_case("rust"), "Rust");
        assert_eq!(title_case("i am groot"), "I Am Groot");
    }

    #[test]
    fn test_change_case() {
        assert_eq!(change_case("Hello"), "hELLO");
        assert_eq!(change_case("heLLo THere"), "HEllO thERE");
        assert_eq!(change_case("123"), "123");
        assert_eq!(change_case("Rust 2025!"), "rUST 2025!");
        assert_eq!(change_case(""), "");
    }
}

//! Misc utilities

/// Additional methods for strings
pub trait StringExt {
    /// Checks if the string is all lowercase
    fn is_lowercase(&self) -> bool;

    /// Checks if a string starts with a lowercase character
    fn starts_with_lowercase(&self) -> bool;

    /// Returns a new string starting with a first character in lowercase
    fn to_lowercase_first(&self) -> String;

    /// Returns a new string starting with a first character in uppercase
    fn to_uppercase_first(&self) -> String;
}

impl StringExt for str {
    fn is_lowercase(&self) -> bool {
        self.chars().all(|c| c.is_lowercase())
    }

    fn starts_with_lowercase(&self) -> bool {
        match self.chars().next() {
            None => false,
            Some(c) => c.is_lowercase(),
        }
    }

    fn to_lowercase_first(&self) -> String {
        self.chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    c.to_lowercase().collect::<String>()
                } else {
                    c.to_string()
                }
            })
            .collect::<String>()
    }

    fn to_uppercase_first(&self) -> String {
        self.chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    c.to_uppercase().collect::<String>()
                } else {
                    c.to_string()
                }
            })
            .collect::<String>()
    }
}

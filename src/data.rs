use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t() {
        // Check if data file exist.
        fs::read_to_string(String::from(""));
    }
}

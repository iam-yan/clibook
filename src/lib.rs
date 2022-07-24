mod backlog;
mod deck;
mod parser;
mod wordbook;

use std::{fs, io::ErrorKind};

pub fn update_wordbook(input: &str, path: &str) {
    // Generate book from input content.
    let b = wordbook::Book::from_article(input);

    //
    match fs::read_to_string(path) {
        Ok(content) => {
            // convert content to book
            // merge 2 books
            // save merged book
        }
        _ => {
            fs::write(path, "hi").unwrap();
        }
    }
}

pub fn load_wordbook(path: &str) -> Option<String> {
    let res = match fs::read_to_string(path) {
        Ok(str) => Some(str),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => None,
            _ => Some(String::from("error")),
        },
    };

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE_NOT_EXIST: &str = ".test/ghost.json";
    const I_AM_HERE_TXT: &str = ".test/iamhere.txt";

    #[test]
    fn can_check_the_absence_of_wordbook() {
        assert_eq!(load_wordbook(FILE_NOT_EXIST), None);
    }

    #[test]
    fn can_load_the_saved_wordbook() {
        assert_eq!(load_wordbook(I_AM_HERE_TXT), Some(String::from("iamhere")));
    }
}

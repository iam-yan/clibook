mod parser;
mod study_book;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use study_book::StudyBook;

use std::{fs, io::ErrorKind};

pub fn update_wordbook(input: &str, path: &str) {
    // Generate book from input content.
    let b = study_book::StudyBook::from_article(input);

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

pub fn load_wordbook(path: &str) -> Result<StudyBook> {
    let res = match fs::read_to_string(path) {
        Ok(str) => {
            match serde_json::from_str(&str) {
                Ok(book) => Ok(book),
                Err(err) => Err(err),
            }
        },
        Err(err) => Err(err),
    };

    res
}

// book::from_json -> Err
// load -> Option
// how to hierachy it?

#[cfg(test)]
mod tests {
    use super::*;

    const FILE_NOT_EXIST: &str = ".test/ghost.json";
    const I_AM_HERE_TXT: &str = ".test/iamhere.txt";
    const VALID_BOOK_JSON: &str = ".test/study_book.json";

    #[test]
    fn can_check_the_absence_of_wordbook() {
        assert_eq!(load_wordbook(FILE_NOT_EXIST), None);
    }

    #[test]
    fn can_load_the_saved_wordbook() {
        assert_eq!(load_wordbook(I_AM_HERE_TXT), Some(String::from("iamhere")));
    }
}

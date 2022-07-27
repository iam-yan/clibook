mod parser;
mod study_book;
pub mod ui;

use serde::{Deserialize, Serialize};
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

pub fn load_study_book(path: &str) -> Result<Option<StudyBook>, &'static str> {
    let res = match fs::read_to_string(path) {
        Ok(str) => match serde_json::from_str(&str) {
            Ok(book) => Ok(Some(book)),
            Err(_) => Err("The source file is invalid."),
        },
        Err(err) => match err.kind() {
            ErrorKind::NotFound => Ok(None),
            _ => Err("Failed to load the source file."),
        },
    };

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE_NOT_EXIST: &str = ".test/ghost.json";
    const I_AM_HERE_TXT: &str = ".test/iamhere.txt";
    const VALID_BOOK_JSON: &str = ".test/study_book.json";

    #[test]
    fn can_check_the_absence_of_book() {
        if let Some(_) = load_study_book(FILE_NOT_EXIST).unwrap() {
            panic!();
        }
        // assert_eq!(load_study_book(FILE_NOT_EXIST).unwrap(), None);
    }

    #[test]
    fn can_load_the_saved_book() {
        let b = load_study_book(VALID_BOOK_JSON).unwrap().unwrap();

        if let Some(_) = b.sentences.achived {
            panic!();
        }

        assert_eq!(
            b.sentences
                .backlog
                .unwrap()
                .get("1")
                .unwrap()
                .sentence
                .sentence(),
            "ロシアへの経済制裁が強まる中、日本の自動車メーカーに影響が広がっています。"
        );

        assert_eq!(
            b.words
                .backlog
                .unwrap()
                .get("122")
                .unwrap()
                .annotation
                .as_ref()
                .unwrap(),
            "to spread out"
        );
    }
}

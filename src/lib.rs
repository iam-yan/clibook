mod parser;
mod wordbook;
mod deck;

use std::fs;

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

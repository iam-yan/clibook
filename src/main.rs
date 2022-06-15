// <todo> Store the loaded json as bac.json on load, on update?
// <todo> Store the history logs of inputting article.

use std::{
    fs,
    io::{self, ErrorKind},
    process,
};

use learn_jp::update_wordbook;

fn main() {
    // <todo> Introduce the concept of user to bring some customization.
    let user = "Yan";

    let data_file = String::from("wordbook.json");

    // Initial check on whether we've got saved book...
    match fs::read_to_string(data_file) {
        // ...- Yes -> create book and decks from the file.
        Ok(content) => {
            println!("Aha you are back, {}. My good boy!", user);
        }
        Err(err) => match err.kind() {
            // ...- No -> create book and decks from the first input.
            ErrorKind::NotFound => {
                // 1. Ask for input...
                println!("Hey man, a great adventure is waiting for you. But first, let's create you the first book.");
                println!("But first, let's create you the first book.");
                println!("No worries it's easy. Just throw me something in the wizard format.");

                let mut input = String::new();

                loop {
                    // 2. Get input and store it in a variable.
                    io::stdin().read_line(&mut input).unwrap_or_else(|err| {
                        // Handle the error of reading input.
                        eprintln!("Failed to read line with err: {}", err);
                        process::exit(1);
                    });

                    let input = input.trim(); // Clean leading and trailing whitespace.

                    // [ ] Convert it to book
                    // [ ] Succeed -> Save input into book.txt
                    // [ ] Failed -> Ask for input again.
                    update_wordbook(input, "wordbook.txt");
                }
            }
            // ...- Handle error on checking the file.
            _ => {}
        },
    }
    // A. Initial check
    //      - No -> Ask for first sentences input -> B.
    //      - Yes -> Read the file to create book and decks

    // B. Take string of sentences and convert it to book and decks.
    //      1. String -> Book * Decks
    //      2. Hanlde invalid input

    // C. Save book as json
    //      1. No file -> Create file first
    //      2. Save as json file
}

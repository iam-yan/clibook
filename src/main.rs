use std::process;

use learn_jp::{
    load_study_book,
    study_book::{self, status::Status, StudyBook},
    ui::{self, NextStep},
};

// [todo] Introduce the concept of user to bring some customization.
const USER_NAME: &str = "Yan";

const SAVE_PATH: &str = ".prod/book.json";

fn main() {
    println!("Hi, {}", USER_NAME);

    // Initialize study_book with either saved book or user's first input,
    //  to get a book with words in the backlog
    let mut b = match load_study_book(SAVE_PATH) {
        Ok(book_opt) => match book_opt {
            // Find saved book -> Check whether there are words in the backlog
            Some(book) => {
                // If have no words in the backlog -> Ask for input
                if book.no_words_in_backlog() {
                    println!("Good job! There is no words in your backlog. Now let's add more.");
                    let input: String = ui::request_raw_content().unwrap();

                    study_book::StudyBook::from_article(&input)
                }
                // Else, return the book directly
                else {
                    book
                }
            }
            // There is no saved book -> ask for initial input
            None => {
                println!("Welcome. To start the advanture, let's add some words into the backlog.");
                let input: String = ui::request_raw_content().unwrap();

                study_book::StudyBook::from_article(&input)
            }
        },
        Err(err) => {
            // Handle error:
            //  - Print the error msg
            //  - End the thread
            println!("Oops something went wrong: {}.", err);
            process::exit(1);
        }
    };

    loop {
        // Save
        if let Err(err) = b.save_json(SAVE_PATH) {
            println!("Oops something went wrong: {}.", err);
            process::exit(1);
        };

        // Report the initial status
        let s = b.get_status();
        println!(
            "Now we have {} words of {} sentences to work on.",
            s.w_backlog, s.s_backlog
        );

        // Ask for the next step
        println!("Should we start learning?");
        match ui::study_or_add_more() {
            Ok(decision) => {
                // Response based on users' decision
                // to fix: here should be a loop for users to keep adding contents
                match decision {
                    NextStep::AddMore => {
                        // Add more
                        let input: String = ui::request_raw_content().unwrap();
                        // Merge
                        b = StudyBook::merge(
                            b,
                            StudyBook::from_article(&input),
                            Some(|s_add: Status, _| {
                                println!(
                                    "You have just added {} words of {} new sentences.",
                                    s_add.w_backlog, s_add.s_backlog
                                )
                            }),
                        );
                        // and then repeat the loop
                    }
                    NextStep::Study => {
                        break;
                    }
                }
            }
            Err(err) => {
                println!("Oops something went wrong: {}.", err);
                process::exit(1);
            }
        }
    }

    // Now book is ready, let's start learning!
    println!("Ok. Let's start learning.");

    loop {
        // [todo] Force input when no deck can be created,
        //  i.e. there is no words in the backlog

        // Generate a random study deck
        let d = b.get_deck().unwrap();

        // Test users with each word
        for w in d {
            let w_id = &w.word.id();

            // Get and store senetence first, because by answering the question correctly,
            //  the sentence could be moved to achived
            let s_id = &w.sentence_id;
            let s = if let Some(s_map) = &b.sentences.backlog {
                s_map.get(s_id).unwrap().sentence.sentence()
            } else {
                ""
            };
            // The book will be brorrowed as a mut ref during level changing,
            //  so we store the s as a local var to drop the immutable ref of book
            let s = String::from(s);

            match ui::exam_word(&w) {
                Ok(correct) => {
                    let (res, msg) = if correct {
                        (b.level_down_word(&w_id), String::from("Correct!"))
                    } else {
                        (
                            b.level_up_word(&w_id),
                            format!("Oops, the answer should be {}.", &w.word.word()),
                        )
                    };

                    // Handle err of changing level
                    if let Err(err) = res {
                        println!("Oops something went wrong: {}.", err);
                        process::exit(1);
                    } else {
                        // Print message based on whether users answer the test correctly
                        println!("{}", &msg);

                        // Print all the relevant info of this word for user to memorize
                        println!(
                            "Kanji \"{}\" is from the sentence \"{}\"",
                            &w.word.word(),
                            &s
                        );
                        println!("Hiragana: {}", &w.hiragana);
                        if let Some(anno) = &w.annotation {
                            println!("Annotation: {}", anno);
                        }
                    }
                }
                Err(err) => {
                    println!("Oops something went wrong: {}.", err);
                    process::exit(1);
                }
            }
        }
    }
}

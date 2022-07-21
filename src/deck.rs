use crate::wordbook::{Book, Word, WordEntries, WordEntry};
use std::collections::HashMap;

pub struct Deck<'a> {
    pub level: u8,
    pub wordEntries: Vec<&'a mut WordEntry>, // Deck should have a method for update level
}

pub struct Backlog<'a> {
    pub decks: HashMap<u8, Deck<'a>>,
}

impl<'a> Backlog<'a> {
    pub fn from_word_entries(word_entries: &mut WordEntries) -> Backlog {
        // Create an empty mutable Backlog
        let mut b: Backlog = Backlog {
            decks: HashMap::new(),
        };

        // Generate decks
        for (_, w) in word_entries {
            let lv = w.level;

            match b.decks.get_mut(&lv) {
                // Find deck of this level -> insert new word entry
                Some(d) => {
                    d.wordEntries.push(w);
                }
                // No deck of this level -> Create one and initialize with this word entry
                None => {
                    b.decks.insert(
                        lv,
                        Deck {
                            level: lv,
                            wordEntries: vec![w],
                        },
                    );
                }
            }
        }

        b
    }

    // Deck of leve?
}

struct Tes {
    a: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_backlog_from_book() {
        let fake_sentence_id = "123";

        let mut word_entries: WordEntries = HashMap::new();

        word_entries.insert(
            String::from("経済制裁"),
            WordEntry {
                level: 1,
                annotation: None,
                hiragana: String::from("けいざいせいさい"),
                sentence_id: String::from(fake_sentence_id),
                word: Word::from("経済制裁"),
            },
        );
        word_entries.insert(
            String::from("強"),
            WordEntry {
                level: 3,
                annotation: None,
                hiragana: String::from("つよ"),
                sentence_id: String::from(fake_sentence_id),
                word: Word::from("強"),
            },
        );
        word_entries.insert(
            String::from("自動車"),
            WordEntry {
                level: 2,
                annotation: None,
                hiragana: String::from("じどうしゃ"),
                sentence_id: String::from(fake_sentence_id),
                word: Word::from("自動車"),
            },
        );
        word_entries.insert(
            String::from("影響"),
            WordEntry {
                level: 1,
                annotation: None,
                hiragana: String::from("えいきょう"),
                sentence_id: String::from(fake_sentence_id),
                word: Word::from("影響"),
            },
        );
        word_entries.insert(
            String::from("広がっています"),
            WordEntry {
                level: 3,
                annotation: Some(String::from("to spread out")),
                hiragana: String::from("ひろがる"),
                sentence_id: String::from(fake_sentence_id),
                word: Word::from("広がっています"),
            },
        );
        word_entries.insert(
            String::from("工場"),
            WordEntry {
                level: 2,
                annotation: None,
                hiragana: String::from("こうじょう"),
                sentence_id: String::from(fake_sentence_id),
                word: Word::from("工場"),
            },
        );
        word_entries.insert(
            String::from("稼働"),
            WordEntry {
                level: 2,
                annotation: Some(String::from("operation of a machine")),
                hiragana: String::from("かどう"),
                sentence_id: String::from(fake_sentence_id),
                word: Word::from("稼働"),
            },
        );

        let mut b = Backlog::from_word_entries(&mut word_entries);

        match b.decks.get(&1) {
            Some(d) => {
                assert_eq!(d.wordEntries.len(), 2);
            }
            None => {
                panic!();
            }
        }

        match b.decks.get_mut(&2) {
            Some(d) => {
                assert_eq!(d.wordEntries.len(), 3);
                if let Some(w) = d.wordEntries.get_mut(0) {
                    // how to assert eq between a value with an union?
                } else {
                    panic!();
                }
            }
            None => {
                panic!();
            }
        }

        match b.decks.get(&3) {
            Some(d) => {
                assert_eq!(d.wordEntries.len(), 2);
            }
            None => {
                panic!();
            }
        }
    }
}

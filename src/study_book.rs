use crate::parser::Parser;
use sentence::{Sentence, SentenceEntry, SentenceEntryMap};
use serde::{Deserialize, Serialize};
use serde_json;
use status::Status;
use std::{collections::HashMap, fs, hash::Hash};
use word::{Word, WordEntry, WordEntryMap};

pub mod sentence;
pub mod status;
pub mod word;

#[derive(Serialize, Deserialize)]
pub struct StudyObjectCollection<T> {
    pub achived: Option<T>,
    pub backlog: Option<T>,
}

#[derive(Serialize, Deserialize)]
pub struct StudyBook {
    pub words: StudyObjectCollection<WordEntryMap>,
    pub sentences: StudyObjectCollection<SentenceEntryMap>,
}

impl StudyBook {
    pub fn from_article(article: &str) -> StudyBook {
        let mut backlog_w = HashMap::new();
        let mut backlog_s = HashMap::new();

        // Start parsing
        let p = Parser::new();

        // Get iter of sentences
        let s_iter = p.cap_sentences_iter(article);

        for s in s_iter {
            // Get cleaned sentence
            let clean_s = p.clean_sentence(s);
            let clean_s = Sentence::from(&clean_s);
            let mut wordentry_ids = Vec::new();

            // Get iter of entry_strs
            let entries_iter = p.cap_entries_iter(s);

            for e in entries_iter {
                // Get iter of entry's fields
                let mut f_iter = p.cap_fields_iter(e);

                // Build the entry
                let word = Word::from(f_iter.next().unwrap());
                wordentry_ids.push(word.id()); // Add entry's id to the relevant stentence struct

                let hiragana = f_iter.next().unwrap().to_owned();

                let annotation = f_iter.next().map(String::from);

                // Insert the word entry into the word backlog
                backlog_w.insert(
                    word.id(),
                    WordEntry {
                        word,
                        hiragana,
                        annotation,
                        sentence_id: clean_s.id(),
                        level: 1,
                    },
                );
            }

            // Insert the sentence entry into the sentence backlog
            backlog_s.insert(
                clean_s.id(),
                SentenceEntry {
                    backlog_volumn: wordentry_ids.len() as u8,
                    sentence: clean_s,
                    wordentry_ids,
                },
            );
        }

        let no_word = backlog_w.len() == 0;

        StudyBook {
            words: StudyObjectCollection {
                achived: None,
                backlog: if no_word { None } else { Some(backlog_w) },
            },
            sentences: StudyObjectCollection {
                achived: None,
                backlog: if no_word { None } else { Some(backlog_s) },
            },
            // status: None,
        }
    }

    pub fn get_status(&self) -> Status {
        fn get_size<K, V>(map: &Option<HashMap<K, V>>) -> usize {
            match map {
                Some(m) => m.len(),
                None => 0,
            }
        }

        Status {
            w_archived: get_size(&self.words.achived),
            w_backlog: get_size(&self.words.backlog),
            s_archived: get_size(&self.sentences.achived),
            s_backlog: get_size(&self.sentences.backlog),
        }
    }

    pub fn merge<F>(book1: StudyBook, book2: StudyBook, cb: Option<F>) -> StudyBook
    where
        F: Fn(Status, Status),
    {
        fn merge_map<K: Hash + Eq, V>(
            map1: Option<HashMap<K, V>>,
            map2: Option<HashMap<K, V>>,
        ) -> Option<HashMap<K, V>> {
            let mut new_map: HashMap<K, V> = HashMap::new();

            if let Some(m1) = map1 {
                for (k, v) in m1 {
                    new_map.insert(k, v);
                }
            }

            if let Some(m2) = map2 {
                for (k, v) in m2 {
                    new_map.insert(k, v);
                }
            }

            match new_map.len() {
                0 => None,
                _ => Some(new_map),
            }
        }

        let s_add = book2.get_status();

        let new_book = StudyBook {
            words: StudyObjectCollection {
                achived: merge_map(book1.words.achived, book2.words.achived),
                backlog: merge_map(book1.words.backlog, book2.words.backlog),
            },
            sentences: StudyObjectCollection {
                achived: merge_map(book1.sentences.achived, book2.sentences.achived),
                backlog: merge_map(book1.sentences.backlog, book2.sentences.backlog),
            },
        };

        let s_new = new_book.get_status();

        if let Some(cb) = cb {
            cb(s_add, s_new);
        }

        new_book
    }

    pub fn no_words_in_backlog(&self) -> bool {
        match &self.words.backlog {
            None => true,
            _ => false,
        }
    }

    pub fn to_json(&self) -> Result<String, &'static str> {
        match serde_json::to_string(self) {
            Ok(json) => Ok(json),
            Err(_) => Err("Failed to convert the book into json"),
        }
    }

    pub fn save_json(&self, path: &str) -> Result<(), &'static str> {
        match self.to_json() {
            Ok(json) => match fs::write(path, json) {
                Ok(_) => Ok(()),
                Err(_) => Err("Failed to save the book in the target path."),
            },
            Err(msg) => Err(msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARTICLE: &str = r"ロシアへの<<経済制裁・けいざいせいさい>>が<<強・つよ>>まる<<中・なか>>、日本の<<自動車・じどうしゃ>>メーカーに<<影響・えいきょう・>>が<<広がっています・ひろがる・to spread out>>。トヨタ自動車はあすからロシアにある<<工場・こうじょう>>の<<稼働・かどう・operation of a machine, running>>を<<停止・ていし>>すると<<発表・はっぴょう>>しました。";

    const A_1: &str = r"ロシアへの<<経済制裁・けいざいせいさい>>が<<強・つよ>>まる<<中・なか>>、日本の<<自動車・じどうしゃ>>メーカーに<<影響・えいきょう・>>が<<広がっています・ひろがる・to spread out>>。";
    const A_2: &str = r"トヨタ自動車はあすからロシアにある<<工場・こうじょう>>の<<稼働・かどう・operation of a machine, running>>を<<停止・ていし>>すると<<発表・はっぴょう>>しました。";

    #[test]
    fn can_detect_no_word_in_backlog() {
        let b = StudyBook::from_article("へへへへへへ");

        assert!(b.no_words_in_backlog());
    }

    #[test]
    fn can_gen_book_from_article() {
        let b = StudyBook::from_article(ARTICLE);

        let backlog_w = b.words.backlog.unwrap();
        let backlog_s = b.sentences.backlog.unwrap();

        assert_eq!(backlog_w.len(), 10);
        assert_eq!(backlog_s.len(), 2);

        let w = Word::from("稼働");
        let w_id = w.id();

        let entry_w = backlog_w.get(&w_id).unwrap();
        assert_eq!(entry_w.word.word(), "稼働");
        assert_eq!(entry_w.hiragana, "かどう");
        assert_eq!(
            entry_w.annotation.to_owned().unwrap(),
            "operation of a machine, running"
        );

        let entry_s = backlog_s.get(&entry_w.sentence_id).unwrap();
        assert_eq!(
            entry_s.sentence.sentence(),
            "トヨタ自動車はあすからロシアにある`工場`の`稼働`を`停止`すると`発表`しました。"
        );

        let w = Word::from("停止");
        assert!(entry_s.wordentry_ids.contains(&w.id()));

        let entry_w = backlog_w.get(&w.id()).unwrap();
        assert_eq!(entry_w.annotation, None);
    }

    #[test]
    fn can_report_correct_status() {
        let s = StudyBook::from_article(ARTICLE).get_status();

        assert_eq!(s.s_archived, 0);
        assert_eq!(s.w_archived, 0);
        assert_eq!(s.s_backlog, 2);
        assert_eq!(s.w_backlog, 10);
    }

    #[test]
    fn can_merge_books() {
        let b1 = StudyBook::from_article(A_1);
        let b2 = StudyBook::from_article(A_2);

        StudyBook::merge(
            b1,
            b2,
            Some(|s_add: Status, s_new: Status| {
                assert_eq!(s_add.s_archived, 0);
                assert_eq!(s_add.w_archived, 0);
                assert_eq!(s_add.s_backlog, 1);
                assert_eq!(s_add.w_backlog, 4);

                assert_eq!(s_new.s_archived, 0);
                assert_eq!(s_new.w_archived, 0);
                assert_eq!(s_new.s_backlog, 2);
                assert_eq!(s_new.w_backlog, 10);
            }),
        );
    }

    #[test]
    fn can_save_json() {
        let path = ".test/test.json";
        let mini_article = r"ロシアへの<<経済制裁・けいざいせいさい>>が<<強・つよ>>。";
        let mini_book = StudyBook::from_article(mini_article);
        mini_book.save_json(path).unwrap();

        let saved_book = fs::read_to_string(path).unwrap();
        let saved_book: StudyBook = serde_json::from_str(&saved_book).unwrap();

        let s = saved_book.get_status();

        assert_eq!(s.s_archived, 0);
        assert_eq!(s.w_archived, 0);
        assert_eq!(s.s_backlog, 1);
        assert_eq!(s.w_backlog, 2);
    }
}

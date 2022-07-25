use crate::parser::Parser;
use sentence::{Sentence, SentenceEntry, SentenceEntryMap};
use status::Status;
use std::collections::HashMap;
use word::{Word, WordEntry, WordEntryMap};

pub mod sentence;
pub mod status;
pub mod word;

struct StudyObjectCollection<T> {
    pub achived: Option<T>,
    pub backlog: Option<T>,
}

pub struct WordBook {
    pub words: StudyObjectCollection<WordEntryMap>,
    pub sentences: StudyObjectCollection<SentenceEntryMap>,
    status: Option<Status>,
}

impl WordBook {
    pub fn from_article(article: &str) -> WordBook {
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

        WordBook {
            words: StudyObjectCollection {
                achived: None,
                backlog: Some(backlog_w),
            },
            sentences: StudyObjectCollection {
                achived: None,
                backlog: Some(backlog_s),
            },
            status: None,
        }
    }

    // pub fn get_status(&self) -> Status {
    //     // Cache
    //     if let None = self.status {
    //         self.status = Some(Status {
    //             entries: self.entries.len() as u16,
    //             sentences: self.sentences.len() as u16,
    //         });
    //     }

    //     self.status.as_ref().unwrap()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARTICLE: &str = r"ロシアへの<<経済制裁・けいざいせいさい>>が<<強・つよ>>まる<<中・なか>>、日本の<<自動車・じどうしゃ>>メーカーに<<影響・えいきょう・>>が<<広がっています・ひろがる・to spread out>>。トヨタ自動車はあすからロシアにある<<工場・こうじょう>>の<<稼働・かどう・operation of a machine, running>>を<<停止・ていし>>すると<<発表・はっぴょう>>しました。";

    #[test]
    fn parse_book() {
        let b = WordBook::from_article(ARTICLE);

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

    // #[test]
    // fn report_status() {
    //     let mut b = Book::from_article(ARTICLE);
    //     let s = b.status();

    //     assert_eq!(s.entries, 10);
    //     assert_eq!(s.sentences, 2);
    // }
}

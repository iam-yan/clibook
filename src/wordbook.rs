use crate::parser::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Word {
    word: String,
}
impl Word {
    pub fn from(v: &str) -> Word {
        Word { word: v.to_owned() }
    }

    pub fn id(&self) -> String {
        base64::encode(&self.word)
    }

    pub fn word(&self) -> &str {
        &self.word
    }
}

#[derive(Serialize, Deserialize)]
pub struct Sentence {
    sentence: String,
}
impl Sentence {
    pub fn from(v: &str) -> Sentence {
        Sentence {
            sentence: v.to_owned(),
        }
    }

    pub fn id(&self) -> String {
        let re = Regex::new("`").unwrap();
        let s = re.replace_all(&self.sentence, "").into_owned();

        base64::encode(s)
    }

    pub fn sentence(&self) -> &str {
        &self.sentence
    }
}

#[derive(Serialize, Deserialize)]
pub struct WordEntry {
    pub annotation: Option<String>,
    pub hiragana: String,
    pub level: u8,
    pub sentence_id: String,
    #[serde(flatten)]
    pub word: Word,
}

type WordEntries = HashMap<String, WordEntry>; // {id: entry}

#[derive(Serialize, Deserialize)]
pub struct SentenceEntry {
    #[serde(flatten)]
    pub sentence: Sentence,
    pub entry_ids: Vec<String>,
}

type SentenceEntries = HashMap<String, SentenceEntry>; // {id: sentence}

struct Status {
    entries: u16,
    sentences: u16,
}

pub struct Book {
    entries: WordEntries,
    sentences: SentenceEntries,
    status: Option<Status>,
}
impl Book {
    pub fn new() -> Book {
        Book {
            entries: HashMap::new(),
            sentences: HashMap::new(),
            status: None,
        }
    }

    pub fn from_article(article: &str) -> Book {
        // Create an empty mutable book
        let mut b = Book::new();

        // Start parsing
        let p = Parser::new();

        // Get iter of sentences
        let s_iter = p.cap_sentences_iter(article);

        for s in s_iter {
            // Get cleaned sentence as IdSrc
            let clean_s = p.clean_sentence(s);
            let clean_s = Sentence::from(&clean_s);
            let mut entry_ids = Vec::new();

            // Get iter of entry_strs
            let entries_iter = p.cap_entries_iter(s);

            for e in entries_iter {
                // Get iter of entry's fields
                let mut f_iter = p.cap_fields_iter(e);

                // Build the entry
                let word = f_iter.next().unwrap();
                let word = Word::from(word);
                entry_ids.push(word.id()); // Add entry's id to the relevant stentence struct

                let hiragana = f_iter.next().unwrap().to_owned();

                let annotation = f_iter.next().map(String::from);

                // Insert the entry into the entries map
                b.entries.insert(
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

            // b.sentences.insert(clean_s.id(), v)
            b.sentences.insert(
                clean_s.id(),
                SentenceEntry {
                    sentence: clean_s,
                    entry_ids,
                },
            );
        }

        b
    }

    pub fn status(&mut self) -> &Status {
        // Cache
        if let None = self.status {
            self.status = Some(Status {
                entries: self.entries.len() as u16,
                sentences: self.sentences.len() as u16,
            });
        }

        self.status.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARTICLE: &str = r"ロシアへの<<経済制裁・けいざいせいさい>>が<<強・つよ>>まる<<中・なか>>、日本の<<自動車・じどうしゃ>>メーカーに<<影響・えいきょう・>>が<<広がっています・ひろがる・to spread out>>。トヨタ自動車はあすからロシアにある<<工場・こうじょう>>の<<稼働・かどう・operation of a machine, running>>を<<停止・ていし>>すると<<発表・はっぴょう>>しました。";

    #[test]
    fn parse_book() {
        let b = Book::from_article(ARTICLE);

        assert_eq!(b.sentences.len(), 2);
        assert_eq!(b.entries.len(), 10);

        let w = Word::from("稼働");
        let w_id = w.id();

        let e = b.entries.get(&w_id).unwrap();
        assert_eq!(e.word.word(), "稼働");
        assert_eq!(e.hiragana, "かどう");
        assert_eq!(
            e.annotation.to_owned().unwrap(),
            "operation of a machine, running"
        );

        let s = b.sentences.get(&e.sentence_id).unwrap();
        assert_eq!(
            s.sentence.sentence(),
            "トヨタ自動車はあすからロシアにある`工場`の`稼働`を`停止`すると`発表`しました。"
        );

        let w = Word::from("停止");
        assert!(s.entry_ids.contains(&w.id()));

        let e = b.entries.get(&w.id()).unwrap();
        assert_eq!(e.annotation, None);
    }

    #[test]
    fn report_status() {
        let mut b = Book::from_article(ARTICLE);
        let s = b.status();

        assert_eq!(s.entries, 10);
        assert_eq!(s.sentences, 2);
    }
}

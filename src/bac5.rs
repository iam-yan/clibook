fn cut_out_sentences(article: &str) -> Vec<&str> {
    article.split('。').filter(|res| res.len() > 0).collect()
}

pub mod wordbook {
    use serde::{Deserialize, Serialize};
    use std::{collections::HashMap, hash::Hash};

    #[derive(Deserialize, Serialize)]
    pub struct Entry {
        word: String,
        level: u8,
        hiragana: String,
        sentence_id: String,
    }

    impl Entry {
        pub fn hiragana(&self) -> &str {
            &self.hiragana
        }
    }

    pub type Entries = HashMap<String, Entry>; // {id: Entry}

    pub type Deck<'a> = Vec<&'a Entry>; // A collection of entries with the same level
    pub type Decks<'a> = HashMap<u8, Deck<'a>>; // {level:Deck}

    pub struct Book<'a> {
        entries: Entries,
        decks: Option<Decks<'a>>,
    }

    impl<'a> Book<'a> {
        pub fn from(json: String) -> Book<'a> {
            let mut book = Book {
                entries: serde_json::from_str(&json).unwrap(),
                decks: None,
            };

            let decks = book.gen_decks();

            if let Some(ds) = decks {
                book.decks = Some(ds);
            }

            // book.decks = book.gen_decks();

            book
        }

        // Get the reference of an entry by its word value.
        pub fn entry_of_word(&self, word: &str) -> Option<&Entry> {
            let entry_id = base64::encode(&word);
            self.entries.get(&entry_id)
        }

        pub fn gen_deck_from_entries(entries: &Entries, lv: u8) -> Option<Deck> {
            let deck: Deck = entries.values().filter(|entry| entry.level == lv).collect();

            match deck.len() {
                0 => None,
                _ => Some(deck),
            }
        }

        pub fn gen_decks_from_entries(entries: &Entries) -> Option<Decks> {
            match entries.len() {
                0 => None,
                _ => {
                    let mut entries_count = entries.len();
                    let mut decks = HashMap::new();
                    let mut lv: u8 = 1;

                    loop {
                        let deck = Book::gen_deck_from_entries(entries, lv);

                        if let Some(d) = deck {
                            let catched_entries = d.len();
                            decks.insert(lv, d);
                            entries_count -= catched_entries;
                        }

                        if entries_count == 0 {
                            break;
                        } else {
                            lv += 1;
                        }
                    }

                    Some(decks)
                }
            }
        }

        pub fn gen_deck(&self, lv: u8) -> Option<Deck> {
            // stack::arg  -> stack::self -> heap::HashMap
            // stack::lv
            // stack::deck -> heap::Vec<item??> <- where do Vec store?
            let deck: Deck = self
                .entries
                .values()
                .filter(|entry| entry.level == lv)
                .collect();

            match deck.len() {
                0 => None,
                _ => Some(deck),
            }
        }

        pub fn gen_decks(&self) -> Option<Decks> {
            // stack::arg  -> stack::self -> heap::HashMap
            match self.entries.len() {
                0 => None,
                _ => {
                    let mut entries = self.entries.len();
                    let mut decks = HashMap::new();
                    let mut lv: u8 = 1;

                    loop {
                        let deck = self.gen_deck(lv);

                        if let Some(d) = deck {
                            let catched_entries = d.len();
                            decks.insert(lv, d);
                            entries -= catched_entries;
                        }

                        if entries == 0 {
                            break;
                        } else {
                            lv += 1;
                        }
                    }

                    Some(decks)
                }
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub const ARTICLE: &str = r#"ロシアへの#経済制裁(けいざいせいさい)#が#強(つよ)#まる#中(なか)#、日本の#自動車(じどうしゃ)#メーカーに#影響(えいきょう)が#広がっています(ひろがる・to spread out)。トヨタ自動車はあすからロシアにある#工場(こうじょう)の#稼働(かどう・operation [of a machine], running)#を#停止(ていし)#すると#発表(はっぴょう)#しました。"#;

    pub const WORDBOOK_META: &str = r#"{
        "57WM5riI5Yi26KOB": {
            "level": 2,
            "word": "経済制裁",
            "hiragana":"けいざいせいさい",
            "sentence_id":"44GR44GE44GW44GE44Gb44GE44GV44GE"
        },
        "5YGc5q2i": {
            "level": 1,
            "word": "停止",
            "hiragana":"ていし",
            "sentence_id":"44Gm44GE44GX"
        },
        "5LqM6Lyq6LuK": {
            "level": 1,
            "word": "二輪車",
            "hiragana":"にりんしゃ",
            "sentence_id":"44Gm44GE44GX"
        }
    }"#;

    #[test]
    fn split_article_with_maru() {
        let sentences = cut_out_sentences(ARTICLE);
        assert_eq!(sentences.len(), 2);
        assert_eq!(sentences[1],"トヨタ自動車はあすからロシアにある#工場(こうじょう)の#稼働(かどう・operation [of a machine], running)#を#停止(ていし)#すると#発表(はっぴょう)#しました");
    }

    #[test]
    fn create_book_from_json() {
        let book = wordbook::Book::from(WORDBOOK_META);

        // Test meta
        assert_eq!(
            book.entry_of_word("経済制裁").unwrap().hiragana(),
            "けいざいせいさい"
        );
        assert_eq!(book.entry_of_word("停止").unwrap().hiragana(), "ていし");

        // Test decks
        // let decks = wb.decks();

        // assert_eq!(decks.len(), 2);

        // let deck_lv1 = decks.get(&1).unwrap();
        // let deck_lv2 = decks.get(&2).unwrap();

        // assert_eq!(deck_lv1.len(), 2);
        // assert_eq!(deck_lv2.len(), 1);
    }

    #[test]
    fn gen_deck() {
        let book = wordbook::Book::from(WORDBOOK_META);
        let deck_lv1 = book.gen_deck(1).unwrap();
        let deck_lv2 = book.gen_deck(2).unwrap();

        assert_eq!(deck_lv1.len(), 2);
        assert_eq!(deck_lv2.len(), 1);
    }

    #[test]
    fn gen_decks() {
        let books = wordbook::Book::from(WORDBOOK_META);
        let decks = books.gen_decks().unwrap();

        assert_eq!(decks.len(), 2);

        let deck_lv1 = decks.get(&1).unwrap();
        let deck_lv2 = decks.get(&2).unwrap();

        assert_eq!(deck_lv1.len(), 2);
        assert_eq!(deck_lv2.len(), 1);
    }
}

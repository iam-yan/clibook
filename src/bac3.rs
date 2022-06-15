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
        pub hiragana: String,
        sentence_id: String,
    }

    pub type Meta = HashMap<String, Entry>;

    pub type Deck<'a> = Vec<&'a Entry>; // Bind the data of entry in a deck with its meta book.

    pub type Decks<'a> = HashMap<u8, Deck<'a>>; // {level: &Entry}

    pub struct Book<'a> {
        meta: Meta,
        decks: Decks<'a>,
        t: HashMap<u8, &'a str>,
    }

    impl<'a> Book<'a> {
        // Initialize a wordbook with json string.
        pub fn from_json(json_str: &str) -> Book {
            let mut wb = Book {
                meta: serde_json::from_str(json_str).unwrap(),
                decks: HashMap::new(),
                t: HashMap::new(),
            };

            let decks = wb.gen_decks();
            wb.decks = decks;

            // let d2 = decks;
            // wb.set_decks(decks);
            // let mut t = HashMap::new();
            // t.insert(1, "test1");

            // wb.t = t;

            wb
        }

        // Get the reference of an entry by its word value.
        pub fn entry_of_word(&self, word: &str) -> Option<&Entry> {
            let entry_id = base64::encode(&word);
            self.meta.get(&entry_id)
        }

        pub fn gen_deck_of_level(&self, lv: u8) -> Option<Deck> {
            let deck: Deck = self
                .meta
                .values()
                .filter(|entry| entry.level == lv)
                .collect();

            match deck.len() {
                0 => None,
                _ => Some(deck),
            }
        }

        pub fn gen_decks(&self) -> Decks {
            let mut entries = self.meta.len();
            let mut decks = HashMap::new();
            let mut lv: u8 = 1;

            loop {
                if entries == 0 {
                    break;
                }

                let deck = self.gen_deck_of_level(lv);

                if let Some(d) = deck {
                    let catched_entries = d.len();
                    decks.insert(lv, d);
                    entries -= catched_entries;
                }

                lv += 1;
            }

            decks
        }

        pub fn decks(&self) -> &Decks {
            &self.decks
        }

        fn set_decks(&mut self, decks: Decks<'a>) {
            self.decks = decks;
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
        let sentences = cut_out_sentences(&ARTICLE);
        assert_eq!(sentences.len(), 2);
        assert_eq!(sentences[1],"トヨタ自動車はあすからロシアにある#工場(こうじょう)の#稼働(かどう・operation [of a machine], running)#を#停止(ていし)#すると#発表(はっぴょう)#しました");
    }

    #[test]
    fn create_book_from_json() {
        let wb = wordbook::Book::from_json(WORDBOOK_META);

        // Test meta
        assert_eq!(
            wb.entry_of_word("経済制裁").unwrap().hiragana,
            "けいざいせいさい"
        );
        assert_eq!(wb.entry_of_word("停止").unwrap().hiragana, "ていし");

        // Test decks
        let decks = wb.decks();

        assert_eq!(decks.len(), 2);

        let deck_lv1 = decks.get(&1).unwrap();
        let deck_lv2 = decks.get(&2).unwrap();

        assert_eq!(deck_lv1.len(), 2);
        assert_eq!(deck_lv2.len(), 1);
    }

    #[test]
    fn gen_deck() {
        let wb = wordbook::Book::from_json(WORDBOOK_META);
        let deck_lv1 = wb.gen_deck_of_level(1).unwrap();
        let deck_lv2 = wb.gen_deck_of_level(2).unwrap();

        assert_eq!(deck_lv1.len(), 2);
        assert_eq!(deck_lv2.len(), 1);
    }

    #[test]
    fn gen_decks() {
        let wb = wordbook::Book::from_json(WORDBOOK_META);
        let decks = wb.gen_decks();

        assert_eq!(decks.len(), 2);

        let deck_lv1 = decks.get(&1).unwrap();
        let deck_lv2 = decks.get(&2).unwrap();

        assert_eq!(deck_lv1.len(), 2);
        assert_eq!(deck_lv2.len(), 1);
    }

    // #[test]
    // fn t() {
    //     let t = vec![1, 2, 3];
    //     let y = t;

    //     assert_eq!(t.len(), 3);
    // }
}

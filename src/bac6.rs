pub mod wordbook {
    use regex::Regex;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Deserialize, Serialize)]
    pub struct Entry {
        annotation: Option<String>,
        hiragana: String,
        level: u8,
        sentence_id: String,
        word: String,
    }
    impl Entry {
        pub fn annotation(&self) -> &Option<String> {
            &self.annotation
        }

        pub fn hiragana(&self) -> &str {
            &self.hiragana
        }

        pub fn level(&self) -> &u8 {
            &self.level
        }

        pub fn word(&self) -> &str {
            &self.word
        }
    }

    struct Sentence {
        id: String,
        content: String,
        entry_ids: Vec<String>,
    }

    type Entries = HashMap<String, Entry>; // {id: entry}

    pub struct Book {
        entries: Entries,
    }
    impl Book {
        pub fn from(json: &str) -> Book {
            Book {
                entries: serde_json::from_str(json).unwrap(),
            }
        }

        pub fn entry(&self, word: &str) -> Option<&Entry> {
            let id = base64::encode(word);
            self.entries.get(&id)
        }

        pub fn gen_backlog(&self) -> Backlog {
            Backlog {
                decks: self.gen_decks(),
            }
        }

        pub fn gen_deck(&self, lv: u8) -> Option<Deck> {
            let d: Deck = self.entries.values().filter(|e| e.level == lv).collect();

            match d.len() {
                0 => None,
                _ => Some(d),
            }
        }

        pub fn gen_decks(&self) -> Option<Decks> {
            let mut entries_count = self.entries.len();
            match entries_count {
                0 => None,
                _ => {
                    let mut decks = HashMap::new();
                    let mut lv: u8 = 1;

                    loop {
                        let deck = self.gen_deck(lv);

                        if let Some(d) = deck {
                            entries_count -= d.len();
                            decks.insert(lv, d);
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

        pub fn gen_sentences<'a>(article: &'a str, delimiter: Option<&str>) -> Vec<&'a str> {
            let delimiter = delimiter.unwrap_or("。");
            article
                .split(delimiter)
                .filter(|res| res.len() > 0)
                .collect()
        }

        pub fn gen_entries<'a>(text: &'a str, delimiter: Option<&str>) -> Vec<&'a str> {
            // let delimiter = delimiter.unwrap_or("#");
            let re = Regex::new(r"<<[^>]*>>").unwrap();
            let mut vec = Vec::new();

            for cap in re.captures_iter(text) {
                vec.push(cap.get(0).unwrap().as_str())
            }
            vec
        }
    }

    type Deck<'a> = Vec<&'a Entry>; // A collection of pointers to entries with the same level
    type Decks<'a> = HashMap<u8, Deck<'a>>; // // {level: deck}

    pub struct Backlog<'a> {
        decks: Option<Decks<'a>>,
    }
    impl<'a> Backlog<'a> {
        pub fn decks(&self) -> &Option<Decks> {
            &self.decks
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const ARTICLE: &str = r#"ロシアへの<<経済制裁[けいざいせいさい]>>が<<強[つよ]>>まる<<中[なか]>>、日本の<<自動車[じどうしゃ]>>メーカーに<<影響[えいきょう]>>が<<広がっています[ひろがる・to spread out]>>。トヨタ自動車はあすからロシアにある<<工場[こうじょう]>>の<<稼働[かどう・operation of a machine, running>>を<<停止[ていし]>>すると<<発表[はっぴょう]>>しました。"#;

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
        let sentences = wordbook::Book::gen_sentences(ARTICLE, None);
        assert_eq!(sentences.len(), 2);
        assert_eq!(sentences[1],"トヨタ自動車はあすからロシアにある<<工場[こうじょう]>>の<<稼働[かどう・operation of a machine, running>>を<<停止[ていし]>>すると<<発表[はっぴょう]>>しました");
    }

    #[test]
    fn capture_entries() {
        let entries = wordbook::Book::gen_entries(ARTICLE, None);

        assert_eq!(entries.len(), 10);
        assert_eq!(entries[0], "<<経済制裁[けいざいせいさい]>>");
        assert_eq!(entries[1], "<<強[つよ]>>");
    }

    #[test]
    fn create_book() {
        let book = wordbook::Book::from(WORDBOOK_META);

        assert_eq!(
            book.entry("経済制裁").unwrap().hiragana(),
            "けいざいせいさい"
        );

        assert_eq!(book.entry("停止").unwrap().hiragana(), "ていし");

        if let Some(_) = book.entry("停止").unwrap().annotation() {
            panic!();
        };
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
        let book = wordbook::Book::from(WORDBOOK_META);
        let decks = book.gen_decks().unwrap();

        assert_eq!(decks.len(), 2);

        let deck_lv1 = decks.get(&1).unwrap();
        let deck_lv2 = decks.get(&2).unwrap();

        assert_eq!(deck_lv1.len(), 2);
        assert_eq!(deck_lv2.len(), 1);
    }

    #[test]
    fn gen_backlog() {
        let book = wordbook::Book::from(WORDBOOK_META);
        let backlog = book.gen_backlog();
        let decks = backlog.decks();

        if let Some(v) = decks {
            assert_eq!(v.len(), 2);

            let deck_lv1 = v.get(&1).unwrap();
            let deck_lv2 = v.get(&2).unwrap();

            assert_eq!(deck_lv1.len(), 2);
            assert_eq!(deck_lv2.len(), 1);
        } else {
            panic!();
        }
    }
}

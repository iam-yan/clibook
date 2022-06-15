use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};

#[derive(Deserialize, Serialize)]
struct WordbookEntry {
    word: String,
    level: u8,
    hiragana: String,
    sentence_id: String,
}

type WordbookMeta = HashMap<String, WordbookEntry>;
struct Wordbook<'a> {
    meta: WordbookMeta,
    decks: HashMap<u8, Vec<&'a WordbookEntry>>,
}

impl<'a> Wordbook<'a> {
    // Initialize a wordbook with json string.
    pub fn from_json(json_str: &str) -> Wordbook {
        Wordbook {
            meta: serde_json::from_str(&json_str).unwrap(),
            decks: HashMap::new(),
        }
    }

    // Get the reference of an entry by its word value.
    pub fn entry_of_word(&self, word: &str) -> Option<&WordbookEntry> {
        let entry_id = base64::encode(&word);
        self.meta.get(&entry_id)
    }

    pub fn gen_deck_of_meta(meta: &WordbookMeta, lv: u8) -> Option<Vec<&WordbookEntry>> {
        let deck: Vec<&WordbookEntry> = meta.values().filter(|entry| entry.level == lv).collect();

        match deck.len() {
            0 => None,
            _ => Some(deck),
        }
    }

    pub fn gen_decks_of_meta(meta: &WordbookMeta) -> HashMap<u8, Vec<&WordbookEntry>> {
        let mut entries = meta.len();
        let mut decks = HashMap::new();
        let mut lv: u8 = 1;

        loop {
            if entries == 0 {
                break;
            }

            let deck = Wordbook::gen_deck_of_meta(&meta, lv);

            if let Some(d) = deck {
                let catched_entries = d.len();
                decks.insert(lv, d);
                entries -= catched_entries;
            }

            lv += 1;
        }

        decks
    }

    //
    pub fn update_decks(&'a mut self) {
        let mut entries = self.meta.len();
        let mut decks = HashMap::new();
        let mut lv: u8 = 1;

        loop {
            if entries == 0 {
                break;
            }

            let deck = Wordbook::gen_deck_of_meta(&self.meta, lv);

            if let Some(d) = deck {
                let catched_entries = d.len();
                decks.insert(lv, d);
                entries -= catched_entries;
            }

            lv += 1;
        }

        self.decks = decks;
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
        let sentences: Vec<&str> = ARTICLE.split('。').filter(|res| res.len() > 0).collect();
        assert_eq!(sentences.len(), 2);
        assert_eq!(sentences[1],"トヨタ自動車はあすからロシアにある#工場(こうじょう)の#稼働(かどう・operation [of a machine], running)#を#停止(ていし)#すると#発表(はっぴょう)#しました");
    }

    #[test]
    fn create_wb_from_json() {
        let wb = Wordbook::from_json(&WORDBOOK_META);
        assert_eq!(
            wb.entry_of_word("経済制裁").unwrap().hiragana,
            "けいざいせいさい"
        );
        assert_eq!(wb.entry_of_word("停止").unwrap().hiragana, "ていし");
    }

    #[test]
    fn gen_deck() {
        let wb = Wordbook::from_json(&WORDBOOK_META);
        let deck_lv1 = Wordbook::gen_deck_of_meta(&wb.meta, 1).unwrap();
        let deck_lv2 = Wordbook::gen_deck_of_meta(&wb.meta, 2).unwrap();

        assert_eq!(deck_lv1.len(), 2);
        assert_eq!(deck_lv2.len(), 1);
    }

    #[test]
    fn gen_decks() {
        let wb = Wordbook::from_json(&WORDBOOK_META).update_decks();
    }
}

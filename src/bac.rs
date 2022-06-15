use base64::encode;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Deserialize, Serialize)]
struct Entry {
    word: String,
    level: u8,
    hiragana: String,
    sentence_id: String,
}

struct Sentence {
    entry_ids: Vec<u32>,
}

type Wordbook = HashMap<String, Entry>;
type SentenceBook = HashMap<String, Sentence>;

type EntryDeck<'a> = Vec<&'a Entry>;
type WeightedEntryDecks<'a> = HashMap<u8, EntryDeck<'a>>;

struct Wb<'a> {
    meta: HashMap<String, Entry>,
    weighted_entries: Option<WeightedEntryDecks<'a>>,
}

impl<'a> Wb<'a> {
    pub fn from_json(json_str: &str) -> Wb {
        let meta: HashMap<String, Entry> = serde_json::from_str(&json_str).unwrap();
        let mut wb = Wb {
            meta,
            weighted_entries: None,
        };
        wb.weighted_entries = wb.gen_weighted_decks();
        wb
    }

    // Get an entry by a word str.
    pub fn entry(&self, word: &str) -> &Entry {
        let entry_id = encode(&word);
        self.meta.get(&entry_id).unwrap()
    }

    fn deck_by_weight(&self, lv: u8) -> Option<EntryDeck> {
        let deck: EntryDeck = self
            .meta
            .values()
            .filter(|entry| entry.level == lv)
            .collect();

        match deck.len() {
            0 => None,
            _ => Some(deck),
        }
    }

    fn update_weighted_decks(&mut self) {
        let mut entries = self.meta.len();
        if entries == 0 {
            self.weighted_entries = None;
        }

        let mut weighted_decks = WeightedEntryDecks::new();
        let mut lv: u8 = 1;

        loop {
            let deck = self.deck_by_weight(lv);
            if let Some(d) = deck {
                let catched_entries = d.len();
                weighted_decks.insert(lv, d);
                entries -= catched_entries;
            }

            if entries == 0 {
                break;
            } else {
                lv += 1;
            }
        }

        self.weighted_entries = Some(weighted_decks);
    }
}

fn json_to_wordbook(json_str: &str) -> Wordbook {
    let wordbook: Wordbook = serde_json::from_str(&json_str).unwrap();
    wordbook
}

// fn words_of_level(lv:u8) ->Vec<Entry> {

// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ARTICLE: &str = r#"ロシアへの#経済制裁(けいざいせいさい)#が#強(つよ)#まる#中(なか)#、日本の#自動車(じどうしゃ)#メーカーに#影響(えいきょう)が#広がっています(ひろがる・to spread out)。トヨタ自動車はあすからロシアにある#工場(こうじょう)の#稼働(かどう・operation [of a machine], running)#を#停止(ていし)#すると#発表(はっぴょう)#しました。"#;

    const TEST_JSON: &str = r#"{
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
    fn can_split_article_by_maru() {
        let sentences: Vec<&str> = TEST_ARTICLE
            .split('。')
            .filter(|res| res.len() > 0)
            .collect();
        assert_eq!(sentences.len(), 2);
        assert_eq!(sentences[1],"トヨタ自動車はあすからロシアにある#工場(こうじょう)の#稼働(かどう・operation [of a machine], running)#を#停止(ていし)#すると#発表(はっぴょう)#しました");
    }

    #[test]
    fn can_create_wordbook_from_json() {
        let wb = Wb::from_json(&TEST_JSON);
        assert_eq!(wb.entry("経済制裁").hiragana, "けいざいせいさい");
        assert_eq!(wb.entry("停止").hiragana, "ていし");
    }

    #[test]
    fn can_prioritize_word() {
        // let wordbook: Wordbook = json_to_wordbook(&TEST_JSON);
        // -> 1 -> [&entry, &entry]
        // let lv: u8 = 1;
        // let t: Vec<Entry> = wordbook
        //     .into_values()
        //     .filter(|entry| entry.level == lv)
        //     .collect();

        // assert_eq!(t.len(), 2);
        // assert_eq!(t[0].level, 1);
        // assert_eq!(t[1].level, 1);

        let wb = Wb::from_json(&TEST_JSON);

        match wb.weighted_entries {
            None => panic!(),
            Some(decks) => assert_eq!(decks.len(), 2),
        }
    }
}

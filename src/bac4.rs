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

    pub type Meta = HashMap<String, Entry>;
    pub type Deck = Vec<Entry>;
    pub type Decks = HashMap<u8, Deck>; // {level: &Entry}

    pub struct Book {
        decks: Decks,
    }

    impl Book {
        // pub fn from_json(json: &str) -> Book {

        // }

        pub fn from(json: &str) {
            let meta = Book::meta(json);
            let mut entries = meta.len();
            // let decks = HashMap::new();
            let mut lv: u8 = 1;

            // meta.values().red;

            loop {
                if entries == 0 {
                    break;
                }
            }
        }

        pub fn meta(json: &str) -> Meta {
            serde_json::from_str(json).unwrap()
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
    fn gen_meta_from_json() {
        let meta = wordbook::Book::meta(WORDBOOK_META);

        assert_eq!(meta.len(), 3);
    }
}

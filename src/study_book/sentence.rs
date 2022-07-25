use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
pub struct SentenceEntry {
    pub backlog_volumn: u8,
    pub sentence: Sentence,
    #[serde(flatten)]
    pub wordentry_ids: Vec<String>,
}

pub type SentenceEntryMap = HashMap<String, SentenceEntry>; // {id - sentence entyr}

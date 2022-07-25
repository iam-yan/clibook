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
pub struct WordEntry {
    pub annotation: Option<String>,
    pub hiragana: String,
    pub level: u8,
    pub sentence_id: String,
    #[serde(flatten)]
    pub word: Word,
}

impl WordEntry {
    pub fn set_level(&mut self, level: u8) {
        self.level = level;
    }
}

pub type WordEntryMap = HashMap<String, WordEntry>; // {id - word entry}

#[cfg(test)]
mod tests {}

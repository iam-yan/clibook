use crate::wordbook::WordEntry;
use std::collections::HashMap;

pub struct Deck<'a> {
    pub level: u8,
    pub wordEntries: Vec<&'a mut WordEntry>, // Deck should have a method for update level
}

impl<'a> Deck<'a> {
    pub fn from_word_entry(word_entry: &mut WordEntry) -> Deck {
        let lv = word_entry.level;
        Deck {
            level: lv,
            wordEntries: vec![word_entry],
        }
    }

    pub fn add_word(&mut self, word_entry: &'a mut WordEntry) {
        self.wordEntries.push(word_entry);
    }
}

use crate::wordbook::{ WordEntry};
use std::collections::HashMap;

pub struct Deck<'a> {
    pub level: u8,
    pub wordEntries: Vec<&'a mut WordEntry>, // Deck should have a method for update level
}

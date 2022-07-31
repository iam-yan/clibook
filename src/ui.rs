use std::{fmt, vec};

use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::study_book::word::WordEntry;

pub fn request_raw_content() -> Result<String, &'static str> {
    match Input::with_theme(&ColorfulTheme::default()).with_prompt("Please input some content with valid markups.")
    .default("トヨタ自動車はあすからロシアにある<<工場・こうじょう>>の<<稼働・かどう・operation of a machine, running>>を<<停止・ていし>>すると<<発表・はっぴょう>>しました。".into())
    .interact_text() {
        Ok(input) => Ok(input),
        Err(_) => Err("Failed to get the input"),
    }
}

#[derive(Clone)]
pub enum NextStep {
    Study,
    AddMore,
}

impl fmt::Display for NextStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            NextStep::Study => "Yes",
            NextStep::AddMore => "No, let's add more contents",
        };
        write!(f, "{}", printable)
    }
}

pub fn study_or_add_more() -> Result<NextStep, &'static str> {
    let options = vec![NextStep::Study, NextStep::AddMore];

    match Select::with_theme(&ColorfulTheme::default())
        .items(&options)
        .default(0)
        .interact_on_opt(&Term::stderr())
    {
        Ok(res) => match res {
            Some(index) => Ok(options[index].clone()),
            None => Ok(options[0].clone()),
        },
        Err(_) => Err("Failed to get the input"),
    }
}

pub fn exam_word(word_entry: &WordEntry) -> Result<bool, &'static str> {
    let w = word_entry.word.word();
    let h = &word_entry.hiragana;

    let q = format!("What's the kanji for {}?", h);

    match Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(&q)
        .interact_text()
    {
        Ok(kanji) => Ok(kanji == w),
        Err(_) => Err("Failed to get the input"),
    }
}

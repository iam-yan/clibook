use regex::Regex;

pub struct Parser {
    cap_word: String,
    cap_entry: String,
    delimiter_field: char,
    delimiter_sentence: char,
    regex: Regex,
}

impl Parser {
    pub fn new() -> Parser {
        let cap_word = String::from("word");
        let cap_entry = String::from("entry");
        let regex = Regex::new(&format!(
            r"<<(?P<{}>(?P<{}>[^>・]*)[^>]*)>>",
            cap_entry, cap_word
        ))
        .unwrap();

        Parser {
            cap_word,
            cap_entry,
            delimiter_field: '・',
            delimiter_sentence: '。',
            regex,
        }
    }

    // "...<<word_1・rest_1>>...<<word_2>>..."
    //  -> "...`word_1`...`word_2`...。"
    pub fn clean_sentence(&self, sentence: &str) -> String {
        let mut cleaned = self
            .regex
            .replace_all(sentence, format!("`${}`", self.cap_word))
            .into_owned();

        cleaned.push(self.delimiter_sentence);
        // if &cleaned[cleaned.len() - 3..] != self.delimiter_sentence.to_string() {
        // }

        cleaned
    }

    // "...<<word_1・hiragana_1・annotation_1>>...<<word_2・hiragana_2>>..."
    //  -> iter[word_1・hiragana_1・annotation_1, word_2・hiragana_2]
    pub fn cap_entries_iter<'a>(&'a self, sentence: &'a str) -> impl Iterator<Item = &'a str> {
        self.regex
            .captures_iter(sentence)
            .map(|caps| caps.name(&self.cap_entry).unwrap().as_str())
    }

    pub fn cap_sentences_iter<'a>(&self, article: &'a str) -> impl Iterator<Item = &'a str> {
        article
            .split(self.delimiter_sentence)
            .filter(|res| res.len() > 0)
    }

    // "word_1・hiragana_1・annotation_1"
    //  -> iter[word, hiragana, annotation]
    pub fn cap_fields_iter<'a>(&self, entry: &'a str) -> impl Iterator<Item = &'a str> {
        entry
            .split(self.delimiter_field)
            .filter(|res| res.len() > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARTICLE: &str = r"ロシアへの<<経済制裁・けいざいせいさい>>が<<強・つよ>>まる<<中・なか>>、日本の<<自動車・じどうしゃ>>メーカーに<<影響・えいきょう・>>が<<広がっています・ひろがる・to spread out>>。トヨタ自動車はあすからロシアにある<<工場・こうじょう>>の<<稼働・かどう・operation of a machine, running>>を<<停止・ていし>>すると<<発表・はっぴょう>>しました。";
    const SENTENCE: &str = r"トヨタ自動車はあすからロシアにある<<工場・こうじょう>>の<<稼働・かどう・operation of a machine, running>>を";
    const ENTRY: &str = "稼働・かどう・operation of a machine, running";
    const ENTRY_NO_ANNOTATION: &str = "工場・こうじょう";

    #[test]
    fn cap_sentence() {
        let p = Parser::new();
        let mut iter = p.cap_sentences_iter(ARTICLE);

        assert_eq!(iter.next(), Some("ロシアへの<<経済制裁・けいざいせいさい>>が<<強・つよ>>まる<<中・なか>>、日本の<<自動車・じどうしゃ>>メーカーに<<影響・えいきょう・>>が<<広がっています・ひろがる・to spread out>>"));
        assert_eq!(iter.next(), Some("トヨタ自動車はあすからロシアにある<<工場・こうじょう>>の<<稼働・かどう・operation of a machine, running>>を<<停止・ていし>>すると<<発表・はっぴょう>>しました"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn clean_sentence() {
        let p = Parser::new();
        assert_eq!(
            p.clean_sentence(SENTENCE),
            "トヨタ自動車はあすからロシアにある`工場`の`稼働`を。"
        );
    }

    #[test]
    fn cap_entry_str() {
        let p = Parser::new();
        let mut iter = p.cap_entries_iter(SENTENCE);

        assert_eq!(iter.next(), Some("工場・こうじょう"));
        assert_eq!(
            iter.next(),
            Some("稼働・かどう・operation of a machine, running")
        );
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn cap_entry_field() {
        let p = Parser::new();
        let mut iter = p.cap_fields_iter(ENTRY);
        assert_eq!(iter.next(), Some("稼働"));
        assert_eq!(iter.next(), Some("かどう"));
        assert_eq!(iter.next(), Some("operation of a machine, running"));

        let mut iter = p.cap_fields_iter(ENTRY_NO_ANNOTATION);
        assert_eq!(iter.next(), Some("工場"));
        assert_eq!(iter.next(), Some("こうじょう"));
        assert_eq!(iter.next(), None);
    }
}

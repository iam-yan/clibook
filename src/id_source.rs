use serde::{Deserialize, Serialize};

// #[derive(Deserialize, Serialize)]
// #[serde(flatten)]
pub struct IdSrc {
    value: String,
    preprocessor: Option<fn(String) -> String>,
}

impl IdSrc {
    pub fn from(value: &str, preprocessor: fn(String) -> String) -> IdSrc {
        IdSrc {
            value: value.to_owned(),
            preprocessor: Some(preprocessor),
        }
    }

    pub fn from_str(value: &str) -> IdSrc {
        IdSrc {
            value: value.to_owned(),
            preprocessor: None,
        }
    }

    pub fn id(&self) -> String {
        let mut v = self.value.to_owned();

        if let Some(p) = self.preprocessor {
            v = p(v);
        }

        base64::encode(v)
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn word_to_id() {
        let w = IdSrc::from_str("自動車");
        assert_eq!(w.id(), base64::encode("自動車"));
    }

    #[test]
    fn preprocess() {
        let s = IdSrc::from(
            "トヨタ`自動車`は`あす`からロシア",
            |v: String| {
                let re = Regex::new("`").unwrap();
                re.replace_all(&v, "").into_owned()
            },
        );

        assert_eq!(s.id(), base64::encode("トヨタ自動車はあすからロシア"));
    }
}

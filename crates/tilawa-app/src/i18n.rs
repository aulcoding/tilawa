//! All page text (lang/en.jsonc, lang/id.jsonc) and the current language.
//!
//! The .jsonc files are JSON plus `//` / `/* */` comments and optional trailing
//! commas, so the notes for whoever edits the copy can stay in the file.
//! The page fetches them at load time, so editing text never needs a Rust rebuild.

use serde_json::{Map, Value};

use crate::Texts;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    En,
    Id,
}

impl Lang {
    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Id => "id",
        }
    }

    /// "id" → Indonesian, anything else → English.
    pub fn from_code(code: &str) -> Lang {
        if code == "id" { Lang::Id } else { Lang::En }
    }

    /// Indonesian or Malay browsers start in Indonesian.
    pub fn from_browser(language: &str) -> Lang {
        let l = language.to_lowercase();
        if l.starts_with("id") || l.starts_with("ms") { Lang::Id } else { Lang::En }
    }

    pub fn other(self) -> Lang {
        match self {
            Lang::En => Lang::Id,
            Lang::Id => Lang::En,
        }
    }
}

/// Both languages. Indonesian falls back to English for any key it doesn't have.
pub struct Catalog {
    en: Map<String, Value>,
    id: Map<String, Value>,
    /// Parse problems, e.g. "lang/id.jsonc: expected `,` or `}` at line 16 column 3"
    pub errors: Vec<String>,
}

impl Catalog {
    pub fn parse(en_src: &str, id_src: &str) -> Catalog {
        let mut errors = Vec::new();
        let en = parse_object("lang/en.jsonc", en_src, &mut errors);
        let mut id = en.clone();
        id.extend(parse_object("lang/id.jsonc", id_src, &mut errors));
        Catalog { en, id, errors }
    }

    pub fn get(&self, lang: Lang) -> &Map<String, Value> {
        match lang {
            Lang::En => &self.en,
            Lang::Id => &self.id,
        }
    }

    /// A plain string entry (`"title": "…"`), if there is one.
    pub fn text(&self, lang: Lang, key: &str) -> Option<&str> {
        self.get(lang).get(key)?.as_str()
    }

    /// Everything for `lang` as plain JSON (for the page's JavaScript).
    pub fn json(&self, lang: Lang) -> String {
        Value::Object(self.get(lang).clone()).to_string()
    }

    /// The part the cards and the header use.
    pub fn texts(&self, lang: Lang) -> Texts {
        Texts::from_json(&self.json(lang))
    }
}

fn parse_object(name: &str, src: &str, errors: &mut Vec<String>) -> Map<String, Value> {
    match serde_json::from_str(&strip_jsonc(src)) {
        Ok(Value::Object(map)) => map,
        Ok(_) => {
            errors.push(format!("{name}: should be one {{ … }} object"));
            Map::new()
        }
        Err(e) => {
            errors.push(format!("{name}: {e}")); // "… at line 16 column 3"
            Map::new()
        }
    }
}

/// JSON with comments → plain JSON. Comments and trailing commas become spaces,
/// newlines are kept, so error line numbers still match the .jsonc file.
pub fn strip_jsonc(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let mut out = String::with_capacity(src.len());
    let mut i = 0;
    let mut in_string = false;
    // Position in `out` of a comma that may turn out to be trailing
    let mut pending_comma: Option<usize> = None;

    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();

        if in_string {
            out.push(c);
            if c == '\\' {
                if let Some(n) = next {
                    out.push(n);
                    i += 1;
                }
            } else if c == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        match (c, next) {
            ('/', Some('/')) => {
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
                continue;
            }
            ('/', Some('*')) => {
                i += 2;
                while i < chars.len() && !(chars[i] == '*' && chars.get(i + 1) == Some(&'/')) {
                    if chars[i] == '\n' {
                        out.push('\n');
                    }
                    i += 1;
                }
                i += 2;
                continue;
            }
            _ => {}
        }

        if c.is_whitespace() {
            out.push(c);
        } else {
            if (c == '}' || c == ']') && let Some(at) = pending_comma {
                out.replace_range(at..at + 1, " ");
            }
            pending_comma = None;
            if c == ',' {
                pending_comma = Some(out.len());
            } else if c == '"' {
                in_string = true;
            }
            out.push(c);
        }
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comments_and_trailing_commas_are_removed() {
        let src = "{\n  // note\n  \"a\": \"x // not a comment\", /* gone */\n  \"b\": [1, 2,],\n}";
        let v: Value = serde_json::from_str(&strip_jsonc(src)).unwrap();
        assert_eq!(v["a"], "x // not a comment");
        assert_eq!(v["b"], serde_json::json!([1, 2]));
    }

    #[test]
    fn escaped_quotes_stay_inside_the_string() {
        let src = r#"{"a": "say \"hi\", // ok",}"#;
        let v: Value = serde_json::from_str(&strip_jsonc(src)).unwrap();
        assert_eq!(v["a"], r#"say "hi", // ok"#);
    }

    #[test]
    fn line_numbers_survive_stripping() {
        let src = "{\n// one\n/* two\nthree */\n\"a\": oops\n}";
        let err = serde_json::from_str::<Value>(&strip_jsonc(src)).unwrap_err();
        assert_eq!(err.line(), 5);
    }

    #[test]
    fn indonesian_falls_back_to_english() {
        let c = Catalog::parse(r#"{"title": "Hi", "day": "Day"}"#, r#"{"day": "Hari"}"#);
        assert_eq!(c.text(Lang::Id, "day"), Some("Hari"));
        assert_eq!(c.text(Lang::Id, "title"), Some("Hi"));
        assert!(c.errors.is_empty());
    }

    #[test]
    fn a_broken_file_is_reported_with_its_line() {
        let c = Catalog::parse("{\"a\": 1}", "{\n\"a\": 2\n\"b\": 3}");
        assert_eq!(c.errors.len(), 1);
        assert!(c.errors[0].starts_with("lang/id.jsonc:") && c.errors[0].contains("line 3"), "{}", c.errors[0]);
        assert_eq!(c.get(Lang::Id)["a"], 1); // still has English
    }

    #[test]
    fn the_real_text_files_parse() {
        let c = Catalog::parse(include_str!("../../../lang/en.jsonc"), include_str!("../../../lang/id.jsonc"));
        assert!(c.errors.is_empty(), "{:?}", c.errors);
        assert_eq!(c.texts(Lang::Id).done_days.len(), 29);
        assert_eq!(c.texts(Lang::En).done_days.len(), 29);
    }

    #[test]
    fn browser_language() {
        assert_eq!(Lang::from_browser("id-ID"), Lang::Id);
        assert_eq!(Lang::from_browser("ms"), Lang::Id);
        assert_eq!(Lang::from_browser("en-GB"), Lang::En);
    }
}

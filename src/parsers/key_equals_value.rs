use std::collections::HashMap;

use super::Line;

/// Parse a Ghostty-style `key = value` config source into an ordered list of
/// [`Line`]s and a lookup map of key → value.
///
/// Lines that are blank or start with `#` are stored as [`Line::Other`].
/// Lines containing `=` are split on the *first* `=` and stored as
/// [`Line::KeyValue`]; any remaining text is part of the value.
/// Everything else is also [`Line::Other`].
pub fn read(src: &str) -> (Vec<Line>, HashMap<String, String>) {
    let mut lines = Vec::new();
    let mut map = HashMap::new();

    for raw in src.lines() {
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            lines.push(Line::Other(raw.to_string()));
            continue;
        }
        if let Some(eq_pos) = raw.find('=') {
            let key = raw[..eq_pos].trim().to_string();
            let value = raw[eq_pos + 1..].trim().to_string();
            if !key.is_empty() {
                map.insert(key.clone(), value.clone());
                lines.push(Line::KeyValue { key, value, raw: raw.to_string() });
                continue;
            }
        }
        lines.push(Line::Other(raw.to_string()));
    }

    (lines, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_key_value_lines() {
        let src = "font-size = 12\ntheme = dark\n";
        let (lines, map) = read(src);
        assert_eq!(map["font-size"], "12");
        assert_eq!(map["theme"], "dark");
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn preserves_comments_and_blanks() {
        let src = "# comment\n\nfont-size = 12\n";
        let (lines, map) = read(src);
        assert_eq!(lines.len(), 3);
        assert_eq!(map.len(), 1);
        assert!(matches!(&lines[0], Line::Other(s) if s == "# comment"));
        assert!(matches!(&lines[1], Line::Other(s) if s.is_empty()));
    }

    #[test]
    fn value_may_contain_equals() {
        let (_, map) = read("key = a=b=c\n");
        assert_eq!(map["key"], "a=b=c");
    }

    #[test]
    fn unknown_line_stored_as_other() {
        let (lines, map) = read("not_a_kv_line\n");
        assert!(map.is_empty());
        assert!(matches!(&lines[0], Line::Other(_)));
    }
}

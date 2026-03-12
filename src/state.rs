use std::collections::HashMap;

use crate::parsers::Line;
use crate::schema::Schema;

pub struct AppState {
    pub schema: Schema,
    pub lines: Vec<Line>,
    pub parsed_values: HashMap<String, String>,
    pub pending_edits: HashMap<String, String>,
}

impl AppState {
    pub fn new(
        schema: Schema,
        lines: Vec<Line>,
        parsed_values: HashMap<String, String>,
    ) -> Self {
        AppState {
            schema,
            lines,
            parsed_values,
            pending_edits: HashMap::new(),
        }
    }

    /// Stage an edit. Does not write to disk.
    pub fn apply_edit(&mut self, key: &str, value: String) {
        self.pending_edits.insert(key.to_string(), value);
    }

    /// Resolve the current value for `key` using priority:
    /// pending edit > parsed value > schema default.
    pub fn resolve_value(&self, key: &str) -> Option<&str> {
        if let Some(v) = self.pending_edits.get(key) {
            return Some(v.as_str());
        }
        if let Some(v) = self.parsed_values.get(key) {
            return Some(v.as_str());
        }
        self.schema
            .options
            .iter()
            .find(|o| o.key == key)
            .and_then(|o| o.default.as_deref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::load_schema;

    fn make_state() -> AppState {
        let schema = load_schema(
            r#"
[tool]
name = "ghostty"
config_path = "~/.config/ghostty/config"
format = "key_equals_value"
[[options]]
key = "font-size"
type = "int"
default = "12"
[[options]]
key = "theme"
type = "string"
"#,
        )
        .unwrap();
        let mut parsed = HashMap::new();
        parsed.insert("theme".to_string(), "dark".to_string());
        AppState::new(schema, vec![], parsed)
    }

    #[test]
    fn pending_edit_takes_priority() {
        let mut state = make_state();
        state.apply_edit("theme", "light".to_string());
        assert_eq!(state.resolve_value("theme"), Some("light"));
    }

    #[test]
    fn parsed_value_used_when_no_edit() {
        let state = make_state();
        assert_eq!(state.resolve_value("theme"), Some("dark"));
    }

    #[test]
    fn schema_default_used_as_fallback() {
        let state = make_state();
        // font-size not in parsed_values, but has a schema default
        assert_eq!(state.resolve_value("font-size"), Some("12"));
    }

    #[test]
    fn unknown_key_returns_none() {
        let state = make_state();
        assert_eq!(state.resolve_value("nonexistent"), None);
    }
}

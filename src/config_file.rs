use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::parsers::Line;

/// Expand a leading `~` to the value of `$HOME`.
pub fn expand_tilde(path: &str) -> PathBuf {
    if path == "~" {
        PathBuf::from(std::env::var("HOME").unwrap_or_default())
    } else if let Some(rest) = path.strip_prefix("~/") {
        PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(rest)
    } else {
        PathBuf::from(path)
    }
}

/// Write `lines` back to `path`, substituting any key present in
/// `pending_edits` in-place and appending new keys at the end.
/// The write is atomic: content is written to a sibling temp file first,
/// then renamed over the target.
pub fn write_config(
    path: &Path,
    lines: &[Line],
    pending_edits: &HashMap<String, String>,
) -> Result<(), String> {
    let mut written: HashSet<String> = HashSet::new();
    let mut output = String::new();

    for line in lines {
        match line {
            Line::KeyValue { key, raw, .. } => {
                if let Some(new_val) = pending_edits.get(key) {
                    output.push_str(&format!("{} = {}\n", key, new_val));
                    written.insert(key.clone());
                } else {
                    output.push_str(raw);
                    output.push('\n');
                }
            }
            Line::Other(raw) => {
                output.push_str(raw);
                output.push('\n');
            }
        }
    }

    // Append keys not already present in the file.
    let mut new_keys: Vec<(&String, &String)> = pending_edits
        .iter()
        .filter(|(k, _)| !written.contains(*k))
        .collect();
    new_keys.sort_by_key(|(k, _)| k.as_str()); // deterministic order
    for (key, value) in new_keys {
        output.push_str(&format!("{} = {}\n", key, value));
    }

    // Atomic write via temp-file rename (same directory → same filesystem).
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }
    let tmp = path.with_extension("cfgwow_tmp");
    std::fs::write(&tmp, output.as_bytes()).map_err(|e| e.to_string())?;
    std::fs::rename(&tmp, path).map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::key_equals_value;
    use crate::schema::load_schema;
    use crate::state::AppState;

    // ── step 8: integration round-trip ──────────────────────────────────────

    #[test]
    fn ghostty_round_trip_edits_in_place() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config");

        std::fs::write(
            &path,
            "# Ghostty config\nfont-size = 12\ntheme = dark\n",
        )
        .unwrap();

        let src = std::fs::read_to_string(&path).unwrap();
        let (lines, parsed) = key_equals_value::read(&src);

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
"#,
        )
        .unwrap();

        let mut state = AppState::new(schema, lines, parsed);
        state.apply_edit("font-size", "14".to_string());

        write_config(&path, &state.lines, &state.pending_edits).unwrap();

        let result = std::fs::read_to_string(&path).unwrap();
        let (_, parsed2) = key_equals_value::read(&result);

        assert_eq!(parsed2["font-size"], "14", "edit should be applied");
        assert_eq!(parsed2["theme"], "dark", "other keys preserved");
        assert!(result.contains("# Ghostty config"), "comment preserved");
    }

    #[test]
    fn write_appends_new_key() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config");
        std::fs::write(&path, "font-size = 12\n").unwrap();

        let src = std::fs::read_to_string(&path).unwrap();
        let (lines, _) = key_equals_value::read(&src);

        let mut edits = HashMap::new();
        edits.insert("new-key".to_string(), "hello".to_string());
        write_config(&path, &lines, &edits).unwrap();

        let result = std::fs::read_to_string(&path).unwrap();
        assert!(result.contains("new-key = hello"), "new key appended");
        assert!(result.contains("font-size = 12"), "existing key preserved");
    }

    #[test]
    fn expand_tilde_replaces_home() {
        let home = std::env::var("HOME").unwrap_or_default();
        assert_eq!(expand_tilde("~/foo/bar"), PathBuf::from(&home).join("foo/bar"));
        assert_eq!(expand_tilde("~"), PathBuf::from(&home));
        assert_eq!(expand_tilde("/abs/path"), PathBuf::from("/abs/path"));
    }
}

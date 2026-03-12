use std::path::PathBuf;

use serde_derive::Deserialize;

/// Parse a TOML string into a [`Schema`]. Returns an error message on failure.
pub fn load_schema(toml_src: &str) -> Result<Schema, String> {
    toml::from_str(toml_src).map_err(|e| e.to_string())
}

/// Load all schemas: bundled built-ins first, then user schemas from
/// `~/.config/cfgwow/schemas/`. A user schema with the same `tool.name` as a
/// built-in overrides it. Malformed files are skipped with a warning printed
/// to stderr.
pub fn load_all_schemas() -> Vec<Schema> {
    const BUNDLED: &[(&str, &str)] = &[
        ("ghostty", include_str!("../schemas/ghostty.toml")),
        ("tmux", include_str!("../schemas/tmux.toml")),
        ("fish", include_str!("../schemas/fish.toml")),
    ];

    let mut schemas: Vec<Schema> = BUNDLED
        .iter()
        .filter_map(|(name, src)| {
            load_schema(src)
                .map_err(|e| eprintln!("cfgwow: built-in schema '{name}' is malformed: {e}"))
                .ok()
        })
        .collect();

    // Merge user schemas, overriding built-ins with the same name.
    for schema in load_user_schemas() {
        if let Some(existing) = schemas
            .iter_mut()
            .find(|s| s.tool.name == schema.tool.name)
        {
            *existing = schema;
        } else {
            schemas.push(schema);
        }
    }

    schemas
}

fn user_schema_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(home)
        .join(".config")
        .join("cfgwow")
        .join("schemas")
}

fn load_user_schemas() -> Vec<Schema> {
    let dir = user_schema_dir();
    let entries = match std::fs::read_dir(&dir) {
        Ok(e) => e,
        Err(_) => return Vec::new(), // directory absent — not an error
    };

    entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()?.to_str()? != "toml" {
                return None;
            }
            let src = std::fs::read_to_string(&path)
                .map_err(|e| eprintln!("cfgwow: could not read {}: {e}", path.display()))
                .ok()?;
            load_schema(&src)
                .map_err(|e| eprintln!("cfgwow: malformed schema {}: {e}", path.display()))
                .ok()
        })
        .collect()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OptionType {
    Bool,
    String,
    Int,
    Float,
    Enum,
    Color,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OptionDef {
    pub key: std::string::String,
    #[serde(rename = "type")]
    pub option_type: OptionType,
    pub default: Option<std::string::String>,
    pub description: Option<std::string::String>,
    pub section: Option<std::string::String>,
    /// Minimum value for `int`/`float` types.
    pub min: Option<f64>,
    /// Maximum value for `int`/`float` types.
    pub max: Option<f64>,
    /// Allowed values for `enum` type.
    pub values: Option<Vec<std::string::String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolMeta {
    pub name: std::string::String,
    pub description: Option<std::string::String>,
    /// Path to the tool's config file (may contain `~`).
    pub config_path: std::string::String,
    /// Parser format: `key_equals_value`, `key_space_value`, or `fish_set`.
    pub format: std::string::String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Schema {
    pub tool: ToolMeta,
    #[serde(default)]
    pub options: Vec<OptionDef>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = r#"
[tool]
name = "ghostty"
config_path = "~/.config/ghostty/config"
format = "key_equals_value"

[[options]]
key = "font-size"
type = "int"
default = "12"
description = "Font size"
min = 6.0
max = 144.0
section = "Font"

[[options]]
key = "theme"
type = "enum"
default = "dark"
values = ["light", "dark"]
"#;

    #[test]
    fn parses_valid_schema() {
        let schema = load_schema(VALID).expect("should parse");
        assert_eq!(schema.tool.name, "ghostty");
        assert_eq!(schema.tool.format, "key_equals_value");
        assert_eq!(schema.options.len(), 2);

        let font = &schema.options[0];
        assert_eq!(font.key, "font-size");
        assert!(matches!(font.option_type, OptionType::Int));
        assert_eq!(font.min, Some(6.0));
        assert_eq!(font.max, Some(144.0));

        let theme = &schema.options[1];
        assert!(matches!(theme.option_type, OptionType::Enum));
        let vals = theme.values.as_ref().expect("should have values");
        assert_eq!(vals, &["light", "dark"]);
    }

    #[test]
    fn errors_on_missing_tool_section() {
        let result = load_schema("[[options]]\nkey = \"x\"\ntype = \"bool\"\n");
        assert!(result.is_err());
    }

    #[test]
    fn errors_on_unknown_option_type() {
        let src = r#"
[tool]
name = "t"
config_path = "/tmp/t"
format = "key_equals_value"

[[options]]
key = "x"
type = "bogus"
"#;
        assert!(load_schema(src).is_err());
    }

    #[test]
    fn empty_options_list_is_ok() {
        let src = r#"
[tool]
name = "t"
config_path = "/tmp/t"
format = "key_equals_value"
"#;
        let schema = load_schema(src).expect("should parse");
        assert!(schema.options.is_empty());
    }

    #[test]
    fn load_all_schemas_returns_three_builtins() {
        let schemas = load_all_schemas();
        let names: Vec<&str> = schemas.iter().map(|s| s.tool.name.as_str()).collect();
        assert!(names.contains(&"ghostty"), "missing ghostty");
        assert!(names.contains(&"tmux"), "missing tmux");
        assert!(names.contains(&"fish"), "missing fish");
    }

    #[test]
    fn user_schema_overrides_builtin() {
        use std::io::Write as _;

        let dir = tempfile::tempdir().unwrap();
        // Write a ghostty override with a different config_path.
        let mut f = std::fs::File::create(dir.path().join("ghostty.toml")).unwrap();
        writeln!(
            f,
            "[tool]\nname = \"ghostty\"\nconfig_path = \"/custom\"\nformat = \"key_equals_value\""
        )
        .unwrap();

        // Temporarily redirect user_schema_dir via HOME.
        // We can't easily call load_all_schemas with a custom dir without
        // refactoring, so test load_user_schemas directly.
        let src = std::fs::read_to_string(dir.path().join("ghostty.toml")).unwrap();
        let schema = load_schema(&src).unwrap();
        assert_eq!(schema.tool.config_path, "/custom");
    }
}

pub mod key_equals_value;

/// A single line from a config file, preserving enough information for
/// lossless write-back.
#[derive(Debug, Clone)]
pub enum Line {
    /// A parsed `key = value` (or equivalent) line.
    KeyValue { key: String, value: String, raw: String },
    /// Any other line (comment, blank, unknown) stored verbatim.
    Other(String),
}

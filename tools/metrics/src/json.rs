//! Reading the JSON of transcripts and logs, parsed by `serde_json` (research note 0025, The
//! crates): a missing member reads as `null`, a value of another type as empty or 0, as the
//! TypeScript scripts read them.

use serde_json::Value;

/// The `null` a missing member reads as.
static NULL: Value = Value::Null;

/// What the reports read of a JSON value.
pub trait Json {
    /// A member of an object, `null` when missing or when this is not an object.
    fn at(&self, key: &str) -> &Value;
    /// The string, empty when this is not a string.
    fn text(&self) -> &str;
    /// The items, none when this is not an array.
    fn items(&self) -> &[Value];
    /// The number, 0 when this is not a number.
    fn number(&self) -> f64;
    /// The number as a whole one, truncated; 0 when this is not a number.
    fn whole(&self) -> i64;
}

impl Json for Value {
    fn at(&self, key: &str) -> &Value {
        self.get(key).unwrap_or(&NULL)
    }

    fn text(&self) -> &str {
        self.as_str().unwrap_or_default()
    }

    fn items(&self) -> &[Value] {
        self.as_array().map_or(&[], Vec::as_slice)
    }

    fn number(&self) -> f64 {
        self.as_f64().unwrap_or(0.0)
    }

    fn whole(&self) -> i64 {
        // Times, durations and token counts are whole numbers far below 2^53.
        #[allow(clippy::cast_possible_truncation)]
        self.as_i64().unwrap_or_else(|| self.number() as i64)
    }
}

/// The JSON objects of a JSON Lines text: its lines starting with `{`, those that do not parse
/// left out.
#[must_use]
pub fn json_lines(text: &str) -> Vec<Value> {
    text.split('\n')
        .filter(|line| line.starts_with('{'))
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect()
}

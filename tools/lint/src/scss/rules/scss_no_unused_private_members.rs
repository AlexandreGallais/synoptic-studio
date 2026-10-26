//! `no-unused-private-members` of stylelint-scss 7.3.0
//! (`src/rules/no-unused-private-members/index.js`, REF-STYLELINT-SOURCE): a private placeholder,
//! variable, function or mixin (named from `-` or `_`, the two the same to Sass) that nothing else
//! in the sheet names, reported over its declaration; a sheet with an `@import` left alone, its
//! members maybe used by the sheets it loads.

use super::{Context, Report, SheetRule};
use crate::javascript::JsText;
use crate::scss::tree::Kind;

/// The rule.
pub struct NoUnusedPrivateMembers;

/// A map keeping its first order, as JavaScript's `Map`.
#[derive(Default)]
struct Members(Vec<(String, usize)>);

impl Members {
    /// Sets a key's node, in place when the key is there.
    fn set(&mut self, key: String, node: usize) {
        match self.0.iter_mut().find(|(each, _)| *each == key) {
            Some(entry) => entry.1 = node,
            None => self.0.push((key, node)),
        }
    }

    /// Whether a key is there.
    fn has(&self, key: &str) -> bool {
        self.0.iter().any(|(each, _)| each == key)
    }

    /// Drops a key unless it belongs to a node.
    fn drop_unless(&mut self, key: &str, node: usize) {
        self.0.retain(|(each, owner)| each != key || *owner == node);
    }
}

/// Whether a byte is a word character or `-` (`[\w-]`).
const fn is_word(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-'
}

/// The private names of a text (`/([$%]*[-_][\w-]+)/g`), underscores read as dashes.
fn private_names(text: &str) -> Vec<String> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        let mut index = at;
        while index < bytes.len() && matches!(bytes[index], b'$' | b'%') {
            index += 1;
        }
        if index + 1 < bytes.len()
            && matches!(bytes[index], b'-' | b'_')
            && is_word(bytes[index + 1])
        {
            let mut end = index + 1;
            while end < bytes.len() && is_word(bytes[end]) {
                end += 1;
            }
            found.push(text[at..end].replace('_', "-"));
            at = end;
        } else {
            at += 1;
        }
    }
    found
}

/// The name before a function's or a mixin's first parenthesis
/// (`/(?:\s*([\w\-$]+)\s*)?\(/`), `None` without parenthesis or without name (where the
/// original throws reading the name it lacks).
fn call_name(params: &str) -> Option<&str> {
    let open = params.find('(')?;
    let before = params[..open].js_trim_end();
    let start = before
        .bytes()
        .rposition(|byte| !(is_word(byte) || byte == b'$'))
        .map_or(0, |at| at + 1);
    (start < before.len()).then(|| &before[start..])
}

impl SheetRule for NoUnusedPrivateMembers {
    fn name(&self) -> &'static str {
        "scss/no-unused-private-members"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "expected",
            "Expected usage of private member \"{}\" within the stylesheet",
        )]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        if sheet
            .all(Kind::AtRule)
            .iter()
            .any(|(_, node)| node.name == "import")
        {
            return;
        }
        let (mut selectors, mut variables, mut functions, mut mixins) = (
            Members::default(),
            Members::default(),
            Members::default(),
            Members::default(),
        );
        for index in sheet.sheet.walk() {
            let node = sheet.node(index);
            if node.is(Kind::Rule) && (node.selector.contains("%-") || node.selector.contains("%_"))
            {
                for selector in node.selector.split([':', '.']) {
                    if (selector.starts_with("%-") || selector.contains("%_"))
                        && !selectors.has(selector)
                    {
                        selectors.set(selector.replace('_', "-"), index);
                    }
                }
            }
            let in_mixin = sheet
                .ancestors(index)
                .iter()
                .any(|each| each.is(Kind::AtRule) && each.name == "mixin");
            if node.is(Kind::Declaration)
                && (node.prop.starts_with("$-") || node.prop.starts_with("$_"))
                && !in_mixin
            {
                variables.set(node.prop.replace('_', "-"), index);
            }
            let private = node.params.starts_with(['-', '_']);
            if node.is(Kind::AtRule) && node.name == "function" && private {
                if let Some(name) = call_name(&node.params) {
                    functions.set(name.replace('_', "-"), index);
                }
            } else if node.is(Kind::AtRule) && node.name == "mixin" && private {
                let name = call_name(&node.params).unwrap_or(&node.params);
                mixins.set(name.replace('_', "-"), index);
            }
        }
        for index in sheet.sheet.walk() {
            let node = sheet.node(index);
            let text = if node.is(Kind::Rule) {
                &node.selector
            } else if node.is(Kind::AtRule) {
                &node.params
            } else {
                continue;
            };
            for name in private_names(text) {
                mixins.drop_unless(&name, index);
                if node.is(Kind::AtRule) {
                    selectors.drop_unless(&name, index);
                }
                variables.drop_unless(&name, index);
                functions.drop_unless(&name, index);
            }
        }
        for (index, node) in sheet.all(Kind::Declaration) {
            for name in private_names(&node.value) {
                variables.drop_unless(&name, index);
                functions.drop_unless(&name, index);
            }
        }
        for (key, index) in selectors
            .0
            .iter()
            .chain(&variables.0)
            .chain(&functions.0)
            .chain(&mixins.0)
        {
            let node = sheet.node(*index);
            let shown = if node.is(Kind::Declaration) {
                &node.prop
            } else {
                key
            };
            reports.push(
                Report::new(node.start, node.end.unwrap_or(sheet.text.len()), "expected")
                    .with(&[shown]),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NoUnusedPrivateMembers, call_name, private_names};
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/no-unused-private-members.txt",
            &|_| Ok(Box::new(NoUnusedPrivateMembers)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (14, 10, 0),
            "{:#?}",
            tally.apart
        );
        assert_eq!(private_names("a-b $_c %-d_e"), ["-b", "$-c", "%-d-e"]);
        assert_eq!(
            (call_name("-f($a)"), call_name("(a)"), call_name("-m")),
            (Some("-f"), None, None)
        );
    }
}

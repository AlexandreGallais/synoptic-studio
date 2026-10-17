//! `unicorn/consistent-compound-words`: a compound word of the plugin's list is cased as one
//! word in a declared name, `password` and not `passWord` (`eslint-plugin-unicorn` 78.0.0, its
//! default options: names declared checked, properties not, imports of packages and shorthand
//! destructurings left alone). Its rename is a suggestion in `ESLint`: none here.

use oxc_semantic::SymbolId;

use super::identifiers::{
    GeneratedNames, available_name, checked_symbols, declared_span, is_identifier_name,
    is_renamable, is_upper_case, is_upper_first, lower_first, scopes_of, upper_first,
};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct ConsistentCompoundWords;

/// The compound words split by a capital, each with its one-word spelling.
const REPLACEMENTS: [(&str, &str); 52] = [
    ("backGround", "background"),
    ("callBack", "callback"),
    ("checkBox", "checkbox"),
    ("clipBoard", "clipboard"),
    ("codeBase", "codebase"),
    ("dataBase", "database"),
    ("downLoad", "download"),
    ("feedBack", "feedback"),
    ("foreGround", "foreground"),
    ("frameWork", "framework"),
    ("headLine", "headline"),
    ("keyBoard", "keyboard"),
    ("keyFrame", "keyframe"),
    ("lifeCycle", "lifecycle"),
    ("metaData", "metadata"),
    ("midPoint", "midpoint"),
    ("nameSpace", "namespace"),
    ("overRide", "override"),
    ("passWord", "password"),
    ("payLoad", "payload"),
    ("placeHolder", "placeholder"),
    ("preView", "preview"),
    ("screenShot", "screenshot"),
    ("sideBar", "sidebar"),
    ("subClass", "subclass"),
    ("subDirectory", "subdirectory"),
    ("subDomain", "subdomain"),
    ("subMenu", "submenu"),
    ("subProcess", "subprocess"),
    ("subString", "substring"),
    ("subTree", "subtree"),
    ("subType", "subtype"),
    ("subTitle", "subtitle"),
    ("timeOut", "timeout"),
    ("timeStamp", "timestamp"),
    ("toolBar", "toolbar"),
    ("toolKit", "toolkit"),
    ("toolTip", "tooltip"),
    ("touchScreen", "touchscreen"),
    ("unSubscribe", "unsubscribe"),
    ("underScore", "underscore"),
    ("upLoad", "upload"),
    ("userName", "username"),
    ("viewPort", "viewport"),
    ("webCam", "webcam"),
    ("webHook", "webhook"),
    ("webSite", "website"),
    ("weekEnd", "weekend"),
    ("whiteSpace", "whitespace"),
    ("wildCard", "wildcard"),
    ("workFlow", "workflow"),
    ("workSpace", "workspace"),
];

/// Whether a compound may end at an offset of a name: at its end, before a digit, `_`, `$` or a
/// capital.
fn is_boundary(name: &str, at: usize) -> bool {
    name[at..].chars().next().is_none_or(|next| {
        next.is_ascii_digit() || next == '_' || next == '$' || next.is_uppercase()
    })
}

/// The compound spelled at an offset: a word of the list in lower case at the name's start, or
/// with a capital anywhere, followed by a boundary; with its length and its one-word spelling.
fn compound_at(name: &str, at: usize) -> Option<(usize, String)> {
    let rest = &name[at..];
    let lower = at == 0;
    REPLACEMENTS
        .iter()
        .find_map(|(split, joined)| {
            let lower_match =
                lower && rest.starts_with(split) && is_boundary(name, at + split.len());
            lower_match.then(|| (split.len(), lower_first(joined)))
        })
        .or_else(|| {
            if !rest.starts_with(|first: char| first.is_ascii_uppercase()) {
                return None;
            }
            REPLACEMENTS.iter().find_map(|(split, joined)| {
                let tail = &split[1..];
                let capital = rest
                    .get(..1)
                    .is_some_and(|first| first.eq_ignore_ascii_case(&split[..1]))
                    && rest[1..].starts_with(tail);
                (capital && is_boundary(name, at + split.len()))
                    .then(|| (split.len(), upper_first(joined)))
            })
        })
        .map(|(length, joined)| {
            let part = &rest[..length];
            (
                length,
                if is_upper_first(part) {
                    upper_first(&joined)
                } else {
                    lower_first(&joined)
                },
            )
        })
}

/// A name with every compound of the list joined, when it has one; a constant is left alone.
fn joined(name: &str) -> Option<String> {
    if is_upper_case(name) {
        return None;
    }
    let mut out = String::with_capacity(name.len());
    let mut at = 0;
    while at < name.len() {
        if let Some((length, replacement)) = compound_at(name, at) {
            out.push_str(&replacement);
            at += length;
            continue;
        }
        let character = name[at..].chars().next()?;
        out.push(character);
        at += character.len_utf8();
    }
    (out != name).then_some(out)
}

impl ConsistentCompoundWords {
    /// Reports a declared name with a split compound.
    fn report_symbol(
        &self,
        source: &Source,
        symbol: SymbolId,
        generated: &mut GeneratedNames,
        findings: &mut Vec<Finding>,
    ) {
        let name = source.semantic.scoping().symbol_name(symbol);
        let Some(replacement) = joined(name) else {
            return;
        };
        let scopes = scopes_of(source, symbol);
        let safe = is_identifier_name(&replacement)
            .then(|| available_name(source, &replacement, &scopes, generated))
            .flatten();
        if let Some(safe) = &safe
            && is_renamable(source, symbol)
        {
            generated.add(safe, &scopes);
        }
        findings.push(source.finding(
            self.name(),
            declared_span(source, symbol).start,
            format!("Prefer `{}` over `{name}`.", safe.unwrap_or(replacement)),
        ));
    }
}

impl Rule for ConsistentCompoundWords {
    fn name(&self) -> &'static str {
        "unicorn/consistent-compound-words"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let mut generated = GeneratedNames::default();
        for symbol in checked_symbols(source) {
            self.report_symbol(source, symbol, &mut generated, findings);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConsistentCompoundWords;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&ConsistentCompoundWords, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_split_compounds() {
        assert_eq!(
            check("const passWord = 'secret';"),
            ["Prefer `password` over `passWord`."]
        );
        assert_eq!(
            check("const isInViewPort = true;"),
            ["Prefer `isInViewport` over `isInViewPort`."]
        );
        assert_eq!(
            check("function unSubscribe() {}"),
            ["Prefer `unsubscribe` over `unSubscribe`."]
        );
        assert_eq!(
            check("type UserName = string;"),
            ["Prefer `Username` over `UserName`."]
        );
        assert_eq!(
            check("const onTimeOut2 = 1;"),
            ["Prefer `onTimeout2` over `onTimeOut2`."]
        );
        assert_eq!(
            check("const password = 1;\nconst passWord = 2;"),
            ["Prefer `password_` over `passWord`."]
        );
    }

    #[test]
    fn accepts_joined_ambiguous_and_constant_names() {
        for code in [
            "const password = 'secret';",
            "const isInViewport = true;",
            "function unsubscribe() {}",
            "const fileName = 'a';",
            "const setUp = 1;",
            "const PASS_WORD = 1;",
            "const passWords = 1;",
            "const object = {passWord: 1};",
            "import {passWord} from 'package';",
            "const {passWord} = object;",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}

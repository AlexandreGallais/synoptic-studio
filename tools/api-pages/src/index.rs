//! The index page and the sidebar file of the API, grouped by kind as `TypeDoc` grouped them:
//! type aliases, variables, functions, each sorted by name.

use std::fmt::Write;

use crate::model::{Export, Kind};
use crate::page::escape;

/// The exports of one kind, sorted by name.
fn group(exports: &[Export], kind: Kind) -> Vec<&Export> {
    let mut of_kind: Vec<&Export> = exports.iter().filter(|each| each.kind == kind).collect();
    of_kind.sort_by(|left, right| crate::order::compare(&left.name, &right.name));
    of_kind
}

/// The index page: the title, then one list of links per kind.
#[must_use]
pub fn index(exports: &[Export], title: &str) -> String {
    let mut out = vec![format!("# {title}")];

    for kind in Kind::ALL {
        let of_kind = group(exports, kind);
        if of_kind.is_empty() {
            continue;
        }
        out.push(format!("## {}", kind.group()));
        let links: Vec<String> = of_kind
            .iter()
            .map(|each| {
                format!(
                    "- [{}]({}/{}.md)",
                    escape(&each.name),
                    kind.folder(),
                    each.name
                )
            })
            .collect();
        out.push(links.join("\n"));
    }

    format!("{}\n", out.join("\n\n"))
}

/// A string as JSON.
fn json_string(text: &str) -> String {
    let mut out = String::from("\"");
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c if u32::from(c) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", u32::from(c));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// The sidebar file `VitePress` reads (`docs/.vitepress/config.ts`): one collapsed group per
/// kind, each item linking to its page under the site path.
#[must_use]
pub fn sidebar(exports: &[Export], site_path: &str) -> String {
    let groups: Vec<String> = Kind::ALL
        .iter()
        .filter_map(|kind| {
            let of_kind = group(exports, *kind);
            (!of_kind.is_empty()).then(|| {
                let items: Vec<String> = of_kind
                    .iter()
                    .map(|each| {
                        format!(
                            "{{\"text\":{},\"link\":{}}}",
                            json_string(&each.name),
                            json_string(&format!("{site_path}{}/{}.md", kind.folder(), each.name))
                        )
                    })
                    .collect();
                format!(
                    "{{\"text\":{},\"collapsed\":true,\"items\":[{}]}}",
                    json_string(kind.group()),
                    items.join(",")
                )
            })
        })
        .collect();

    format!("[{}]", groups.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract::module;

    fn exports() -> Vec<Export> {
        module(
            "/** f. */\nexport function bytesToHex(): void {}\n/** F. */\nexport type ByteRgb = number;\n/** g. */\nexport function add(): void {}\n/** C. */\nexport const DEGREES_PER_TURN = 360;\n",
            "a.ts",
        )
        .exports
    }

    #[test]
    fn writes_the_index_by_kind_sorted_by_name() {
        assert_eq!(
            index(&exports(), "editor"),
            "# editor\n\n## Type Aliases\n\n- [ByteRgb](type-aliases/ByteRgb.md)\n\n## Variables\n\n- [DEGREES\\_PER\\_TURN](variables/DEGREES_PER_TURN.md)\n\n## Functions\n\n- [add](functions/add.md)\n- [bytesToHex](functions/bytesToHex.md)\n"
        );
    }

    #[test]
    fn writes_the_sidebar_and_leaves_an_empty_kind_out() {
        let functions: Vec<Export> = exports()
            .into_iter()
            .filter(|each| each.kind == Kind::Function)
            .collect();

        assert_eq!(
            sidebar(&functions, "/api/"),
            "[{\"text\":\"Functions\",\"collapsed\":true,\"items\":[{\"text\":\"add\",\"link\":\"/api/functions/add.md\"},{\"text\":\"bytesToHex\",\"link\":\"/api/functions/bytesToHex.md\"}]}]"
        );
        assert_eq!(
            index(&functions, "t"),
            "# t\n\n## Functions\n\n- [add](functions/add.md)\n- [bytesToHex](functions/bytesToHex.md)\n"
        );
    }

    #[test]
    fn escapes_json_strings() {
        assert_eq!(json_string("a\"b\\c\nd\u{1}"), "\"a\\\"b\\\\c\\nd\\u0001\"");
    }
}

//! `markdown/require-alt-text`: an image has a text alternative, in Markdown `![alt](…)` or in an
//! HTML `<img alt="…">`; an empty `alt=""` or `aria-hidden="true"` marks a decorative image.

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::javascript::is_js_space;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, nodes, start};
use crate::markdown::text::{attributes_end, find_without_case, strip_html_comments};

/// The rule.
pub struct RequireAltText;

/// The `<img …>` tags of a text, each with its offset: `<img(?:\s(…)*)?\/?>`, without case.
#[must_use]
pub fn img_tags(text: &str) -> Vec<(usize, &str)> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut at = 0;

    while let Some(open) = find_without_case(text, "<img", at) {
        at = open + 1;
        let mut end = attributes_end(text, open + 4);
        if bytes.get(end) == Some(&b'/') {
            end += 1;
        }
        if bytes.get(end) == Some(&b'>') {
            found.push((open, &text[open..=end]));
            at = end + 1;
        }
    }
    found
}

/// The first attribute of a tag of a name, `\s<name>(?:\s*=\s*['"]([^'"]*)['"])?` without
/// case: `None` when absent, its quoted value, `""` when it has none.
#[must_use]
pub fn attribute<'t>(tag: &'t str, name: &str) -> Option<&'t str> {
    let mut at = 0;
    let found = loop {
        let candidate = find_without_case(tag, name, at)?;
        let spaced = tag[..candidate]
            .chars()
            .next_back()
            .is_some_and(is_js_space);
        if spaced {
            break candidate;
        }
        at = candidate + 1;
    };
    let rest = tag[found + name.len()..].trim_start_matches(is_js_space);
    let value = rest
        .strip_prefix('=')
        .map(|rest| rest.trim_start_matches(is_js_space))
        .and_then(|rest| rest.strip_prefix(['"', '\'']))
        .and_then(|rest| {
            let length = rest.find(['"', '\''])?;
            Some(&rest[..length])
        });
    Some(value.unwrap_or_default())
}

impl MarkdownRule for RequireAltText {
    fn name(&self) -> &'static str {
        "markdown/require-alt-text"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        let message = || "Alternative text for image is required.".to_owned();

        for node in nodes(&page.root) {
            match node {
                Node::Image(image) if image.alt.trim_matches(is_js_space).is_empty() => {
                    findings.push(page.finding(self.name(), start(node), message()));
                }
                Node::ImageReference(image) if image.alt.trim_matches(is_js_space).is_empty() => {
                    findings.push(page.finding(self.name(), start(node), message()));
                }
                Node::Html(_) => {
                    for (offset, tag) in img_tags(&strip_html_comments(page.source(node))) {
                        let hidden = attribute(tag, "aria-hidden")
                            .is_some_and(|value| value.eq_ignore_ascii_case("true"));
                        let missing = attribute(tag, "alt").is_none_or(|value| {
                            !value.is_empty() && value.trim_matches(is_js_space).is_empty()
                        });
                        if !hidden && missing {
                            findings.push(page.finding(
                                self.name(),
                                start(node) + offset,
                                message(),
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RequireAltText;
    use crate::markdown::testing::{messages, places};
    use crate::testing::NONE;

    const MESSAGE: &str = "Alternative text for image is required.";

    #[test]
    fn reports_the_documented_images_without_alternative() {
        let page = "![](sunset.png)\n\n![ ](sunset.png)\n\n![][ref]\n\n[ref]: sunset.png\n\n<img src=\"sunset.png\">\n\n<IMG src=\"sunset.png\" alt=\" \"/>\n\n<img src=\"a.png\" aria-hidden=\"false\">\n";

        assert_eq!(
            places(&RequireAltText, page),
            [
                (1, 1, MESSAGE.to_owned()),
                (3, 1, MESSAGE.to_owned()),
                (5, 1, MESSAGE.to_owned()),
                (9, 1, MESSAGE.to_owned()),
                (11, 1, MESSAGE.to_owned()),
                (13, 1, MESSAGE.to_owned()),
            ]
        );
    }

    #[test]
    fn accepts_the_documented_alternatives() {
        let page = "![A beautiful sunset](sunset.png)\n\n![Company logo][logo]\n\n[logo]: logo.png\n\n<img src=\"sunset.png\" alt=\"\">\n\n<img src=\"sunset.png\" alt=\"A beautiful sunset\">\n\n<img src=\"decorative.png\" aria-hidden=\"true\">\n\n<!-- <img src=\"a.png\"> -->\n";

        assert_eq!(messages(&RequireAltText, page), NONE);
    }
}

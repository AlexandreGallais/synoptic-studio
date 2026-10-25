//! `local/image-alt-text`, a rule of the project (EN-077): an image's text alternative is short
//! and says what the image shows, as the records' conventions, area writing asks after Google's guide
//! (`REF-GOOGLE-IMAGES`): under 155 characters, never opening with "Image of". Nothing checked it
//! before; `markdown/require-alt-text` only asks that there be one.

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, nodes, start};
use crate::markdown::rules::require_alt_text::{attribute, img_tags};
use crate::markdown::text::strip_html_comments;

/// The rule.
pub struct ImageAltText;

/// The length an alternative stays under, in characters.
pub const MAX_LENGTH: usize = 155;

/// The opening an alternative never has, compared without case.
pub const NEEDLESS_OPENING: &str = "image of";

/// What is wrong with an alternative, if anything.
fn problem(alt: &str) -> Option<String> {
    let length = alt.chars().count();
    let opening = alt.trim_start().get(..NEEDLESS_OPENING.len());

    if length >= MAX_LENGTH {
        Some(format!(
            "Alternative text of {length} characters: keep it under {MAX_LENGTH}."
        ))
    } else if opening.is_some_and(|opening| opening.eq_ignore_ascii_case(NEEDLESS_OPENING)) {
        Some("Alternative text opens with \"Image of\": say what the image shows.".into())
    } else {
        None
    }
}

impl MarkdownRule for ImageAltText {
    fn name(&self) -> &'static str {
        "local/image-alt-text"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        for node in nodes(&page.root) {
            let alts: Vec<(usize, String)> = match node {
                Node::Image(image) => vec![(start(node), image.alt.clone())],
                Node::ImageReference(image) => vec![(start(node), image.alt.clone())],
                Node::Html(_) => img_tags(&strip_html_comments(page.source(node)))
                    .into_iter()
                    .filter_map(|(offset, tag)| {
                        attribute(tag, "alt").map(|alt| (start(node) + offset, alt.to_owned()))
                    })
                    .collect(),
                _ => continue,
            };
            for (offset, alt) in alts {
                if let Some(message) = problem(&alt) {
                    findings.push(page.finding(self.name(), offset, message));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ImageAltText, MAX_LENGTH};
    use crate::markdown::testing::{messages, places};
    use crate::testing::NONE;

    #[test]
    fn reports_a_long_alternative_and_an_image_of() {
        let long = "a".repeat(MAX_LENGTH);
        let page = format!(
            "![{long}](a.svg)\n\n![Image of a rectangle](a.svg)\n\n<img src=\"a.svg\" alt=\"image of a polygon\">\n"
        );

        assert_eq!(
            places(&ImageAltText, &page),
            [
                (
                    1,
                    1,
                    "Alternative text of 155 characters: keep it under 155.".to_owned()
                ),
                (
                    3,
                    1,
                    "Alternative text opens with \"Image of\": say what the image shows."
                        .to_owned()
                ),
                (
                    5,
                    1,
                    "Alternative text opens with \"Image of\": say what the image shows."
                        .to_owned()
                ),
            ]
        );
    }

    #[test]
    fn accepts_a_short_description() {
        // 154 characters, one under the limit; "Imagery" is not "Image of".
        let page = format!(
            "![{}](a.svg)\n\n![A rectangle of 100 × 20](a.svg)\n\n![Imagery of a plant](b.svg)\n\n![][ref]\n\n[ref]: c.svg\n",
            "é".repeat(MAX_LENGTH - 1)
        );

        assert_eq!(messages(&ImageAltText, &page), NONE);
    }
}

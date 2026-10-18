//! `markdown/no-empty-images`: an image has a source other than `#`.

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, nodes, start};
use crate::markdown::rules::no_empty_definitions::is_empty_url;

/// The rule.
pub struct NoEmptyImages;

impl MarkdownRule for NoEmptyImages {
    fn name(&self) -> &'static str {
        "markdown/no-empty-images"
    }

    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        for node in nodes(&page.root) {
            if matches!(node, Node::Image(image) if is_empty_url(&image.url)) {
                findings.push(page.finding(
                    self.name(),
                    start(node),
                    "Unexpected empty image found.".into(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoEmptyImages;
    use crate::markdown::testing::{messages, places};
    use crate::testing::NONE;

    #[test]
    fn reports_the_documented_empty_images() {
        assert_eq!(
            places(&NoEmptyImages, "![]()\n\n![Image](#)\n"),
            [
                (1, 1, "Unexpected empty image found.".to_owned()),
                (3, 1, "Unexpected empty image found.".to_owned()),
            ]
        );
    }

    #[test]
    fn accepts_an_image_with_a_source() {
        assert_eq!(messages(&NoEmptyImages, "![Image](./a.svg)\n"), NONE);
    }
}

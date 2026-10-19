//! `local/anchor-slugs`, a rule of the project (EN-140, F33.AC2): a link to a heading's anchor,
//! on its own page or on another Markdown page of the repository, reaches it on GitHub and on the
//! site alike. GitHub writes a heading's anchor with `github-slugger` (`REF-GITHUB-SLUGGER`), the
//! site with `VitePress`'s `slugify` (`REF-VITEPRESS-SLUGIFY`); they differ on a heading holding a
//! `/`, an `_`, a `—` or a leading digit, among others, so a link that works on one breaks on the
//! other. A heading linked whose two anchors differ is reported, naming its page and its text; a
//! fragment that names no heading of another page is reported too, which
//! `markdown/no-missing-link-fragments` only checks on the link's own page.

use std::fs;
use std::path::Path;

use markdown::mdast::Node;

use crate::finding::Finding;
use crate::markdown::MarkdownRule;
use crate::markdown::page::{Page, nodes, start};
use crate::markdown::rules::no_missing_link_fragments::heading_text;
use crate::markdown::slugger::{SiteSlugger, Slugger};

/// The rule.
pub struct AnchorSlugs;

/// The rule's name.
const NAME: &str = "local/anchor-slugs";

/// A heading's anchors: its text, GitHub's anchor and the site's, `None` when not computed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Anchor {
    /// The heading's text, as both anchors read it.
    pub text: String,
    /// Its anchor on GitHub.
    pub github: String,
    /// Its anchor on the site.
    pub site: Option<String>,
}

/// The anchors of a page's headings, in the page's order, each made unique as its slugger does.
#[must_use]
pub fn anchors(root: &Node) -> Vec<Anchor> {
    let mut github = Slugger::default();
    let mut site = SiteSlugger::default();

    nodes(root)
        .into_iter()
        .filter(|node| matches!(node, Node::Heading(_)))
        .map(|heading| {
            let text = heading_text(heading);

            Anchor {
                github: github.slug(&text),
                site: site.slug(&text),
                text,
            }
        })
        .collect()
}

/// The page and the fragment a link's URL names: `None` for this page, the path as written for
/// another Markdown page; `None` for a URL without a fragment, with a scheme, from the site's root
/// or to a file other than Markdown.
fn target(url: &str) -> Option<(Option<&str>, &str)> {
    let (path, fragment) = url.split_once('#')?;

    if fragment.is_empty() {
        return None;
    }
    if path.is_empty() {
        return Some((None, fragment));
    }

    let local = !path.contains(':') && !path.starts_with('/');

    let markdown = Path::new(path)
        .extension()
        .is_some_and(|extension| extension == "md");

    (local && markdown).then_some((Some(path), fragment))
}

/// A page's path joined to a link's relative path, `.` and `..` resolved, `/`-separated; `None`
/// when it climbs above the root.
#[must_use]
pub fn joined(page: &str, link: &str) -> Option<String> {
    let mut parts: Vec<&str> = page.split('/').collect();

    parts.pop();
    for part in link.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            _ => parts.push(part),
        }
    }

    Some(parts.join("/"))
}

/// The link and definition nodes of a page with their URL.
fn links(root: &Node) -> Vec<(&Node, &str)> {
    nodes(root)
        .into_iter()
        .filter_map(|node| match node {
            Node::Link(link) => Some((node, link.url.as_str())),
            Node::Definition(definition) => Some((node, definition.url.as_str())),
            _ => None,
        })
        .collect()
}

/// The findings of the rule on a page, another page's text given by `read` from its path (`None`
/// when it cannot be read, the link then left to the site's build and the reader).
pub fn check_with(page: &Page, read: &dyn Fn(&str) -> Option<String>, findings: &mut Vec<Finding>) {
    // Placeholder (EN-140.T1): the links read, no finding made.
    let read_links = links(&page.root)
        .iter()
        .filter_map(|(_, url)| target(url))
        .count();
    findings.reserve(read_links);
    let _ = (read, NAME, start(&page.root));
}

impl MarkdownRule for AnchorSlugs {
    fn name(&self) -> &'static str {
        NAME
    }

    /// Reads another page from the linter's folder, the repository's root.
    fn check(&self, page: &Page, findings: &mut Vec<Finding>) {
        check_with(
            page,
            &|path| fs::read_to_string(Path::new(path)).ok(),
            findings,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{Anchor, anchors, check_with, joined, target};
    use crate::markdown::page::Page;
    use crate::testing::{NONE, messages};

    /// The messages of the rule on a page at `docs/a.md`, `docs/b.md` reading `other`.
    fn found(text: &str, other: &str) -> Vec<String> {
        let page = Page::parse("docs/a.md", text).unwrap();
        let mut findings = Vec::new();

        check_with(
            &page,
            &|path| (path == "docs/b.md").then(|| other.to_owned()),
            &mut findings,
        );
        messages(&findings)
    }

    #[test]
    fn f33_ac2_gives_each_heading_its_two_anchors() {
        let page = Page::parse("a.md", "# A `b/c`\n\n## A b/c\n\n## Step 2 — D\n").unwrap();

        // GitHub drops the slash and the dash, the site turns them into hyphens; the second
        // "A b/c" is numbered by both.
        assert_eq!(
            anchors(&page.root),
            [
                ("A b/c", "a-bc", "a-b-c"),
                ("A b/c", "a-bc-1", "a-b-c-1"),
                ("Step 2 — D", "step-2--d", "step-2-—-d"),
            ]
            .map(|(text, github, site)| Anchor {
                text: text.to_owned(),
                github: github.to_owned(),
                site: Some(site.to_owned()),
            })
        );
    }

    #[test]
    fn f33_ac2_accepts_a_link_whose_heading_has_one_anchor() {
        let page = "# The tools\n\n[Same](#the-tools) [Other](./b.md#rust) [Line](#L3) [Top](#top)\n\n[Web](https://a.example/b.md#x) [Root](/b.md#x) [Code](./c.rs#x) [None](./b.md)\n";

        assert_eq!(found(page, "# Rust\n"), NONE);
    }

    #[test]
    fn f33_ac2_refuses_a_heading_whose_anchors_differ_naming_its_page() {
        let page = "## 2. Logged items\n\n[Here](#2-logged-items)\n\n[There](b.md#a-bc) [Site](../docs/b.md#step-2-—-d)\n";
        let other = "# A b/c\n\n## Step 2 — D\n";

        assert_eq!(
            found(page, other),
            [
                "Heading '2. Logged items' of docs/a.md has the anchor '#2-logged-items' on GitHub and '#_2-logged-items' on the site: write it so that both are the same.",
                "Heading 'A b/c' of docs/b.md has the anchor '#a-bc' on GitHub and '#a-b-c' on the site: write it so that both are the same.",
                "Heading 'Step 2 — D' of docs/b.md has the anchor '#step-2--d' on GitHub and '#step-2-—-d' on the site: write it so that both are the same.",
            ]
        );
    }

    #[test]
    fn f33_ac2_refuses_a_fragment_another_page_lacks_and_a_heading_not_computed() {
        let page =
            "# I ♥ it\n\n[Mine](#i--it) [Gone](./b.md#gone) [Missing](#nowhere) [Away](./x.md#a)\n";

        // A fragment missing on its own page is `markdown/no-missing-link-fragments`'s; a page that
        // cannot be read is left to the site's build.
        assert_eq!(
            found(page, "# Rust\n"),
            [
                "Heading 'I ♥ it' of docs/a.md holds a character whose anchor on the site is not computed: write it in ASCII, its anchor on GitHub being '#i--it'.",
                "Link fragment '#gone' does not reference a heading of docs/b.md.",
            ]
        );
    }

    #[test]
    fn reads_a_link_s_target_and_joins_its_path() {
        assert_eq!(target("#a"), Some((None, "a")));
        assert_eq!(target("b.md#a"), Some((Some("b.md"), "a")));
        assert_eq!(target("b.md#"), None);
        assert_eq!(target("b.md"), None);
        assert_eq!(target("mailto:a.md#b"), None);
        assert_eq!(
            joined("docs/a/b.md", "../c.md").as_deref(),
            Some("docs/c.md")
        );
        assert_eq!(
            joined("docs/a.md", "./b/./c.md").as_deref(),
            Some("docs/b/c.md")
        );
        assert_eq!(joined("a.md", "../b.md"), None);
    }
}

//! The order of the stories in the index: epic by epic, in the order of the tables of each
//! epic's file, each feature's stories in the order of its own file's table.

use std::collections::{HashMap, HashSet};

use crate::story::Story;

/// A parent item of the backlog, epic or feature: its id, its own epic and its file's text.
pub struct Parent {
    pub id: String,
    pub epic: String,
    pub text: String,
}

/// The ids linked from the rows of a file's tables, in their order: `| [F11](…) — … |`.
pub fn table_links(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| line.starts_with('|'))
        .flat_map(links_in)
        .collect()
}

/// The texts of the links `[ID](` of one line, in their order.
fn links_in(line: &str) -> Vec<String> {
    line.split('[')
        .skip(1)
        .filter_map(|part| part.split_once("]("))
        .map(|(label, _)| label.to_owned())
        .collect()
}

/// The stories in the index's order.
///
/// Epics come by id. Within an epic, the ids its tables link come in their order: a feature
/// brings its stories in the order of its own tables, then those it does not list, by file
/// name; a story attached to the epic comes alone. The epic's features it does not link
/// follow, by id, then its stories still left, by file name. Stories of no epic file end the
/// list, by file name.
pub fn ordered<'a>(stories: &'a [Story], epics: &[Parent], features: &[Parent]) -> Vec<&'a Story> {
    let mut walk = Walk {
        stories: by_file(stories),
        features: features
            .iter()
            .map(|feature| (feature.id.as_str(), feature))
            .collect(),
        seen: HashSet::new(),
        out: Vec::new(),
    };
    let mut epics_by_id: Vec<&Parent> = epics.iter().collect();

    epics_by_id.sort_by(|left, right| left.id.cmp(&right.id));

    for epic in epics_by_id {
        walk.epic(epic, features);
    }

    walk.rest(|_| true);

    walk.out
}

/// The stories sorted by file name, which starts with their epic, feature and id.
fn by_file(stories: &[Story]) -> Vec<&Story> {
    let mut sorted: Vec<&Story> = stories.iter().collect();

    sorted.sort_by(|left, right| left.file.cmp(&right.file));

    sorted
}

/// The walk through the parents' tables, the stories already placed kept.
struct Walk<'a, 'p> {
    stories: Vec<&'a Story>,
    features: HashMap<&'p str, &'p Parent>,
    seen: HashSet<&'a str>,
    out: Vec<&'a Story>,
}

impl<'a> Walk<'a, '_> {
    /// Places an epic's features and stories.
    fn epic(&mut self, epic: &Parent, features: &[Parent]) {
        for id in table_links(&epic.text) {
            match self.features.get(id.as_str()) {
                Some(feature) if feature.epic == epic.id => self.feature(&feature.id.clone()),
                _ => self.story(&id, |story| story.epic == epic.id),
            }
        }

        let mut unlinked: Vec<&Parent> = features
            .iter()
            .filter(|feature| feature.epic == epic.id)
            .collect();

        unlinked.sort_by(|left, right| left.id.cmp(&right.id));

        for feature in unlinked {
            self.feature(&feature.id);
        }

        self.rest(|story| story.epic == epic.id);
    }

    /// Places a feature's stories: those its tables link, then the others by file name.
    fn feature(&mut self, id: &str) {
        let text = self
            .features
            .get(id)
            .map(|feature| feature.text.clone())
            .unwrap_or_default();

        for link in table_links(&text) {
            self.story(&link, |story| story.feature == id);
        }

        self.rest(|story| story.feature == id);
    }

    /// Places the story of an id, when it belongs where the walk is and is not placed yet.
    fn story(&mut self, id: &str, belongs: impl Fn(&Story) -> bool) {
        let found = self
            .stories
            .iter()
            .copied()
            .find(|story| story.id == id && belongs(story));

        if let Some(story) = found {
            self.place(story);
        }
    }

    /// Places, by file name, every story not placed yet that matches.
    fn rest(&mut self, belongs: impl Fn(&Story) -> bool) {
        let left: Vec<&'a Story> = self
            .stories
            .iter()
            .copied()
            .filter(|story| belongs(story))
            .collect();

        for story in left {
            self.place(story);
        }
    }

    /// Places a story once.
    fn place(&mut self, story: &'a Story) {
        if self.seen.insert(story.id.as_str()) {
            self.out.push(story);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Parent, ordered, table_links};
    use crate::story::Story;

    fn story(id: &str, epic: &str, feature: &str) -> Story {
        let parent = if feature.is_empty() { epic } else { feature };

        Story {
            id: id.to_owned(),
            epic: epic.to_owned(),
            feature: feature.to_owned(),
            title: String::new(),
            status: "done".to_owned(),
            file: format!("{epic}-{parent}-{id}.md"),
        }
    }

    fn parent(id: &str, epic: &str, text: &str) -> Parent {
        Parent {
            id: id.to_owned(),
            epic: epic.to_owned(),
            text: text.to_owned(),
        }
    }

    fn ids(stories: &[&Story]) -> Vec<String> {
        stories.iter().map(|story| story.id.clone()).collect()
    }

    #[test]
    fn reads_the_links_of_table_rows_only() {
        let text = "See [F01](./a.md).\n| [F02](./b.md) — x | [US-001](./c.md) |\n| x |\n";

        assert_eq!(table_links(text), vec!["F02", "US-001"]);
    }

    #[test]
    fn follows_the_epic_then_the_feature_tables() {
        // E01 lists F02 before F01, then its review; F01 lists US-002 before US-001.
        let stories = [
            story("US-001", "E01", "F01"),
            story("US-002", "E01", "F01"),
            story("SP-001", "E01", "F02"),
            story("REV-001", "E01", ""),
        ];
        let epics = [parent(
            "E01",
            "",
            "| [F02](x) |\n| [F01](x) |\n| [REV-001](x) |\n",
        )];
        let features = [
            parent("F01", "E01", "| [US-002](x) |\n| [US-001](x) |\n"),
            parent("F02", "E01", "| [SP-001](x) |\n"),
        ];

        assert_eq!(
            ids(&ordered(&stories, &epics, &features)),
            vec!["SP-001", "US-002", "US-001", "REV-001"]
        );
    }

    #[test]
    fn places_what_no_table_lists_by_file_name_after_its_parent() {
        // F01's table lists only US-002; F03 is in no epic table; E02 has no file.
        let stories = [
            story("US-002", "E01", "F01"),
            story("EN-009", "E01", "F01"),
            story("US-001", "E01", "F01"),
            story("SP-003", "E01", "F03"),
            story("RET-001", "E01", ""),
            story("SP-004", "E02", "F04"),
        ];
        let epics = [parent("E01", "", "| [F01](x) |\n")];
        let features = [
            parent("F01", "E01", "| [US-002](x) |\n"),
            parent("F03", "E01", ""),
        ];

        assert_eq!(
            ids(&ordered(&stories, &epics, &features)),
            vec!["US-002", "EN-009", "US-001", "SP-003", "RET-001", "SP-004"]
        );
    }

    #[test]
    fn places_each_story_once_and_only_under_its_own_parent() {
        // F01's table also links F02's SP-001 and repeats US-001: each comes once, in F02 for SP-001.
        let stories = [story("US-001", "E01", "F01"), story("SP-001", "E01", "F02")];
        let epics = [parent("E01", "", "| [F01](x) |\n| [F02](x) |\n")];
        let features = [
            parent(
                "F01",
                "E01",
                "| [SP-001](x) |\n| [US-001](x) |\n| [US-001](x) |\n",
            ),
            parent("F02", "E01", "| [SP-001](x) |\n"),
        ];

        assert_eq!(
            ids(&ordered(&stories, &epics, &features)),
            vec!["US-001", "SP-001"]
        );
    }

    #[test]
    fn gives_nothing_without_stories() {
        assert_eq!(
            ids(&ordered(&[], &[parent("E01", "", "| [F01](x) |\n")], &[])),
            Vec::<&str>::new()
        );
    }
}

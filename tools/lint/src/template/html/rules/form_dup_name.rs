//! `form-dup-name` of `html-validate` 11.17.0 (`src/rules/form-dup-name.ts`,
//! REF-HTML-VALIDATE-SOURCE): two enabled controls of one form (or one `<template>`, or the
//! template's root) sharing a static `name`, reported at the second's value; radio buttons and
//! buttons share a name among their own type, a `name[]` never mixes with `name`, and a checkbox
//! may follow the hidden field of its default value.

use super::super::a11y::{html_hidden, inert};
use super::super::view::Doc;
use super::super::{MESSAGES, report};
use crate::template::rules::{Report, Template, TemplateRule};

/// The rule and its options.
pub struct FormDupName {
    /// Whether `name[]` may repeat.
    pub allow_array_brackets: bool,
    /// Whether a checkbox may follow a hidden field of its name.
    pub allow_checkbox_default: bool,
    /// The control types that may share a name, the rule's default when `None`.
    pub shared: Option<Vec<String>>,
}

/// The control types that may share a name by default.
const SHARED: [&str; 4] = ["radio", "button", "reset", "submit"];

impl FormDupName {
    /// The project's configuration: the rule's defaults.
    pub const PROJECT: Self = Self {
        allow_array_brackets: true,
        allow_checkbox_default: true,
        shared: None,
    };

    /// Whether a control type may share its name.
    fn shares(&self, kind: &str) -> bool {
        self.shared.as_ref().map_or_else(
            || SHARED.contains(&kind),
            |shared| shared.iter().any(|each| each == kind),
        )
    }
}

/// A name seen: whether an array, and whether a hidden field a checkbox may follow.
struct Seen {
    /// The group it was seen in.
    group: Option<usize>,
    /// Its name, `[]` dropped for an array.
    name: String,
    /// Whether written `name[]`.
    array: bool,
    /// Whether a hidden field.
    hidden_default: bool,
}

/// Whether a control is disabled (`isInputDisabled`): its own `disabled`, or a disabled
/// `<fieldset>` around it, written.
fn disabled(doc: &Doc, index: usize) -> bool {
    let written = |each: usize| {
        doc.attr(each, "disabled")
            .is_some_and(|attr| !attr.is_dynamic())
    };
    written(index)
        || doc
            .closest(index, "fieldset[disabled]")
            .is_some_and(written)
}

/// A control's type, `submit` for a `<button>` without one (`getControlType`).
fn control_type(doc: &Doc, index: usize) -> String {
    let kind = doc.value(index, "type").unwrap_or_default();
    if doc.el(index).tag == "button" && kind.is_empty() {
        return "submit".to_owned();
    }
    kind.to_owned()
}

impl TemplateRule for FormDupName {
    fn name(&self) -> &'static str {
        "html-validate/form-dup-name"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        MESSAGES
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let doc = Doc::new(template.text, template.nodes);
        let controls: Vec<usize> = (0..doc.elements.len())
            .filter(|index| {
                doc.el(*index)
                    .meta
                    .and_then(|meta| meta.form_associated)
                    .is_some_and(|associated| associated.listed)
            })
            .filter(|index| {
                !html_hidden(&doc, *index).any()
                    && !inert(&doc, *index).any()
                    && !disabled(&doc, *index)
            })
            .collect();
        let (shared, unique): (Vec<usize>, Vec<usize>) = controls
            .into_iter()
            .partition(|index| self.shares(&control_type(&doc, *index)));
        let mut seen: Vec<Seen> = Vec::new();
        let named = |index: usize| {
            doc.attr(index, "name").and_then(|attr| {
                attr.text()
                    .filter(|name| !name.is_empty())
                    .map(|name| (attr, name))
            })
        };
        let group_of = |index: usize| doc.closest(index, "form, template");
        for index in unique {
            let Some((attr, name)) = named(index) else {
                continue;
            };
            let group = group_of(index);
            let duplicate = |name: &str| format!("Duplicate form control name \"{name}\"");
            if self.allow_array_brackets {
                let array = name.ends_with("[]");
                let base = name.strip_suffix("[]").unwrap_or(name);
                let found = seen
                    .iter()
                    .find(|each| each.group == group && each.name == base);
                if found.is_some_and(|each| each.array != array) {
                    reports.push(report(
                        attr.value_span(),
                        format!("Cannot mix \"{base}[]\" and \"{base}\""),
                    ));
                    continue;
                }
                if found.is_none() && array {
                    seen.push(Seen {
                        group,
                        name: base.to_owned(),
                        array: true,
                        hidden_default: false,
                    });
                }
                if array {
                    continue;
                }
            }
            let checkbox =
                doc.el(index).tag == "input" && doc.value(index, "type") == Some("checkbox");
            match seen
                .iter_mut()
                .find(|each| each.group == group && each.name == name)
            {
                Some(previous)
                    if self.allow_checkbox_default && previous.hidden_default && checkbox =>
                {
                    previous.hidden_default = false;
                }
                Some(_) => reports.push(report(attr.value_span(), duplicate(name))),
                None => seen.push(Seen {
                    group,
                    name: name.to_owned(),
                    array: false,
                    hidden_default: doc.el(index).tag == "input"
                        && doc.value(index, "type") == Some("hidden"),
                }),
            }
        }
        check_shared(&doc, &shared, &seen, reports);
    }
}

/// Checks the controls that may share a name (`validateSharedName`): one a unique control has,
/// or a control of another type of the same group, is reported.
fn check_shared(doc: &Doc, shared: &[usize], seen: &[Seen], reports: &mut Vec<Report>) {
    let mut shared_seen: Vec<(Option<usize>, &str, String)> = Vec::new();
    for index in shared {
        let Some(attr) = doc.attr(*index, "name") else {
            continue;
        };
        let Some(name) = attr.text().filter(|name| !name.is_empty()) else {
            continue;
        };
        let group = doc.closest(*index, "form, template");
        let kind = control_type(doc, *index);
        let unique = seen
            .iter()
            .any(|each| each.group == group && each.name == name);
        let other_type = shared_seen
            .iter()
            .rev()
            .find(|(each_group, each, _)| *each_group == group && *each == name)
            .is_some_and(|(_, _, each_kind)| *each_kind != kind);
        if unique || other_type {
            reports.push(report(
                attr.value_span(),
                format!("Duplicate form control name \"{name}\""),
            ));
        }
        shared_seen.push((group, name, kind));
    }
}

#[cfg(test)]
mod tests {
    use super::FormDupName;
    use crate::template::html::cases::run;

    #[test]
    fn f53_ac1_passes_the_cases_of_html_validate() {
        let tally = run("form-dup-name.txt", "form-dup-name", &|options| {
            let mut rule = FormDupName::PROJECT;
            if let Some(options) = options {
                if let Some(flag) = options.flag("allowArrayBrackets") {
                    rule.allow_array_brackets = flag;
                }
                if let Some(flag) = options.flag("allowCheckboxDefault") {
                    rule.allow_checkbox_default = flag;
                }
                if let Some(shared) = options.texts("shared") {
                    rule.shared = Some(shared);
                }
            }
            Box::new(rule)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.valid, tally.invalid, tally.apart_count()),
            (20, 12, 8),
            "{:#?}",
            tally.apart_cases
        );
    }
}

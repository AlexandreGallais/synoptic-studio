//! `@angular-eslint/template/no-duplicate-attributes`: no input, output or plain attribute
//! twice on an element, by the name written (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/no-duplicate-attributes.ts` and `src/utils/get-original-attribute-name.ts`): the
//! inputs and the plain attributes are compared together, the outputs apart, the output of a
//! two-way binding left out unless `allowTwoWayDataBinding` is off; `class` and `style` may be
//! both bound and plain with `allowStylePrecedenceDuplicates`; the names of `ignore` never
//! count. Each duplicate is reported over its attribute; the plugin's removal is a suggestion,
//! not carried. The project runs the plugin's defaults.

use super::{Bound3, Report, Role, Template, TemplateRule, bound3, is_plain_element};
use crate::template::tree::Node;

/// The rule and its options.
pub struct NoDuplicateAttributes {
    /// Whether the output of `[(x)]` is left out.
    pub allow_two_way_data_binding: bool,
    /// Whether `class` and `style` may be both bound and plain.
    pub allow_style_precedence_duplicates: bool,
    /// The names that never count.
    pub ignore: &'static [&'static str],
}

impl NoDuplicateAttributes {
    /// The plugin's defaults, which the project runs.
    pub const PROJECT: Self = Self {
        allow_two_way_data_binding: true,
        allow_style_precedence_duplicates: false,
        ignore: &[],
    };
}

/// The names Angular merges when both bound and plain (`angularStylePrecedenceDuplicatesAllowed`).
const STYLE_PRECEDENCE: [&str; 2] = ["class", "style"];

/// The attributes of a list whose name another of the list has (`findDuplicates`).
fn duplicates<'b, 't>(attributes: &[&'b Bound3<'t>]) -> Vec<&'b Bound3<'t>> {
    attributes
        .iter()
        .enumerate()
        .filter(|(index, each)| {
            attributes
                .iter()
                .enumerate()
                .any(|(other, them)| other != *index && them.original == each.original)
        })
        .map(|(_, each)| *each)
        .collect()
}

impl NoDuplicateAttributes {
    /// The duplicates of an element's attributes, as the plugin finds them.
    fn found<'b, 't>(&self, attributes: &'b [Bound3<'t>]) -> Vec<&'b Bound3<'t>> {
        let of = |role: Role| -> Vec<&'b Bound3<'t>> {
            attributes.iter().filter(|each| each.role == role).collect()
        };
        let (inputs, outputs, texts) = (of(Role::Input), of(Role::Output), of(Role::Text));
        let mut found = duplicates(&[inputs.clone(), texts.clone()].concat());
        let styled = |each: &&Bound3| STYLE_PRECEDENCE.contains(&each.original.as_str());
        if self.allow_style_precedence_duplicates && inputs.iter().any(styled) {
            let (inputs_ignored, inputs_kept): (Vec<_>, Vec<_>) =
                inputs.iter().partition(|each| styled(each));
            let (texts_ignored, texts_kept): (Vec<_>, Vec<_>) =
                texts.iter().partition(|each| styled(each));
            found = [
                duplicates(&inputs_ignored),
                duplicates(&texts_ignored),
                duplicates(&inputs_kept),
                duplicates(&texts_kept),
            ]
            .concat();
        }
        let outputs: Vec<&Bound3> = outputs
            .into_iter()
            .filter(|output| {
                !self.allow_two_way_data_binding
                    || !inputs
                        .iter()
                        .any(|input| input.attribute.span == output.attribute.span)
            })
            .collect();
        found.extend(duplicates(&outputs));
        found.retain(|each| !self.ignore.contains(&each.original.as_str()));
        found
    }
}

impl TemplateRule for NoDuplicateAttributes {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/no-duplicate-attributes"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            (
                "noDuplicateAttributes",
                "Duplicate attribute `{{attributeName}}`",
            ),
            (
                "suggestRemoveAttribute",
                "Remove attribute `{{attributeName}}`",
            ),
        ]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        template.walk(&mut |node, _| {
            let Node::Element(element) = node else {
                return;
            };
            if !is_plain_element(element) {
                return;
            }
            let attributes = bound3(element);
            for duplicate in self.found(&attributes) {
                reports.push(Report {
                    data: vec![("attributeName", duplicate.original.clone())],
                    ..Report::new(duplicate.attribute.span, "noDuplicateAttributes")
                });
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::NoDuplicateAttributes;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("no-duplicate-attributes.txt", &[], &|options| {
            let defaults = NoDuplicateAttributes::PROJECT;
            let ignore: Vec<String> = options
                .and_then(|each| each.texts("ignore"))
                .unwrap_or_default();
            Box::new(NoDuplicateAttributes {
                allow_two_way_data_binding: options
                    .and_then(|each| each.flag("allowTwoWayDataBinding"))
                    .unwrap_or(defaults.allow_two_way_data_binding),
                allow_style_precedence_duplicates: options
                    .and_then(|each| each.flag("allowStylePrecedenceDuplicates"))
                    .unwrap_or(defaults.allow_style_precedence_duplicates),
                ignore: Vec::leak(
                    ignore
                        .into_iter()
                        .map(|each| &*String::leak(each))
                        .collect(),
                ),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 13 valid and 14 invalid cases, all run; their 31 suggestions, not carried, unchecked.
        assert_eq!(
            (tally.valid, tally.invalid, tally.suggestions),
            (13, 14, 31)
        );
    }
}

//! `@angular-eslint/template/mouse-events-have-key-events`: a `(mouseover)` comes with a
//! `(focus)`, a `(mouseout)` with a `(blur)` (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/mouse-events-have-key-events.ts`): an element of the DOM, its name in lower case,
//! that has the mouse event and not the key event is reported over itself. The plugin's
//! selector (`:has`) finds both events among the element's outputs and those of every element
//! under it, as the parser's keys walk them: the children of elements, templates, blocks and
//! content projections, the outputs of elements only (a `<ng-template>`'s and a
//! `<ng-content>`'s are not walked).

use super::dom::is_dom_element;
use super::{Report, Role, Template, TemplateRule, bound3, is_plain_element};
use crate::template::tree::{Element, Node};

/// The rule.
pub struct MouseEventsHaveKeyEvents;

/// Each key event and the mouse event it must come with.
const PAIRS: [(&str, &str); 2] = [("blur", "mouseout"), ("focus", "mouseover")];

/// The outputs of the nodes under a list of siblings, as `:has` walks them.
fn outputs_under(nodes: &[Node], found: &mut Vec<String>) {
    for node in nodes {
        match node {
            Node::Element(element) => {
                if is_plain_element(element) {
                    outputs_of(element, found);
                }
                outputs_under(&element.children, found);
            }
            Node::Block(block) => outputs_under(&block.children, found),
            Node::Text(_) | Node::Comment(_) | Node::LetDeclaration(_) | Node::Expansion(_) => {}
        }
    }
}

/// The names of an element's outputs.
fn outputs_of(element: &Element, found: &mut Vec<String>) {
    found.extend(
        bound3(element)
            .into_iter()
            .filter(|each| each.role == Role::Output)
            .map(|each| each.name),
    );
}

impl TemplateRule for MouseEventsHaveKeyEvents {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/mouse-events-have-key-events"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "mouseEventsHaveKeyEvents",
            "`{{mouseEvent}}` must be accompanied by `{{keyEvent}}` for accessibility (https://www.w3.org/WAI/WCAG21/Understanding/keyboard)",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, _) in template.elements() {
            if !is_plain_element(element) || !is_dom_element(&element.name) {
                continue;
            }
            let mut outputs = Vec::new();
            outputs_of(element, &mut outputs);
            outputs_under(&element.children, &mut outputs);
            let has = |name: &str| outputs.iter().any(|each| each == name);
            for (key_event, mouse_event) in PAIRS {
                if has(mouse_event) && !has(key_event) {
                    reports.push(Report {
                        data: vec![
                            ("keyEvent", key_event.to_owned()),
                            ("mouseEvent", mouse_event.to_owned()),
                        ],
                        ..Report::new(element.span, "mouseEventsHaveKeyEvents")
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MouseEventsHaveKeyEvents;
    use crate::template::lint_with;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac3_passes_the_plugin_s_cases() {
        let tally = run("mouse-events-have-key-events.txt", &[], &|_| {
            Box::new(MouseEventsHaveKeyEvents)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 4 valid and 2 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (4, 2, 0));
    }

    #[test]
    fn finds_the_events_of_the_elements_under_it() {
        // Our own cases, after `:has`: the `<div>` and the `<p>` hold a `(mouseover)` without
        // `(focus)`, so they (1:1, 1:6) and the `<span>` (1:9) are reported; a `(focus)` under
        // it, through a block, satisfies it.
        let places = |text: &str| {
            lint_with("a.component.html", text, &[&MouseEventsHaveKeyEvents])
                .iter()
                .map(|finding| (finding.line, finding.column))
                .collect::<Vec<_>>()
        };

        assert_eq!(
            places("<div><p><span (mouseover)=\"a()\"></span></p></div>"),
            [(1, 1), (1, 6), (1, 9)]
        );
        assert_eq!(
            places("<div (mouseover)=\"a()\">@if (b) {<i (focus)=\"c()\"></i>}</div>"),
            []
        );
        // A `<ng-template>`'s own outputs are not walked.
        assert_eq!(
            places("<div (mouseover)=\"a()\"><ng-template (focus)=\"c()\"></ng-template></div>"),
            [(1, 1)]
        );
    }
}

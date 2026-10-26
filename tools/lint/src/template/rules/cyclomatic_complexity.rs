//! `@angular-eslint/template/cyclomatic-complexity`: no template whose paths exceed
//! `maxComplexity` (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/cyclomatic-complexity.ts`): each `@if`, `@for`, `ngIf`, `ngForOf` and `@case`,
//! `@default`, `ngSwitchCase` and `ngSwitchDefault` (the `classic` variant), or each `@switch`
//! and `ngSwitch` (the `modified` one), counts 1, in the order the plugin's walk meets them:
//! a structural directive's bindings before its element, an element's children before its own
//! attributes; each one past the limit is reported over itself. The project runs the plugin's
//! defaults, a limit of 5 and the `classic` variant.

use super::r3::{R3, siblings};
use super::{Report, Role, Template, TemplateRule, bound3, switch_groups, template_attrs};
use crate::template::lexer::Span;
use crate::template::tags::split_ns_name;
use crate::template::tree::{Block, Element, Node};

/// The rule and its options.
pub struct CyclomaticComplexity {
    /// The highest complexity allowed.
    pub max_complexity: u32,
    /// Whether the `modified` variant, a switch counting 1 whatever its cases.
    pub modified: bool,
}

impl CyclomaticComplexity {
    /// The plugin's defaults, which the project runs.
    pub const PROJECT: Self = Self {
        max_complexity: 5,
        modified: false,
    };
}

/// The walk's count and its reports.
struct Counter<'r> {
    /// The rule.
    rule: &'r CyclomaticComplexity,
    /// The count so far.
    total: u32,
    /// The reports.
    reports: &'r mut Vec<Report>,
}

impl Counter<'_> {
    /// Counts a node, reported past the limit (`increment`).
    fn increment(&mut self, span: Span) {
        self.total += 1;
        if self.total > self.rule.max_complexity {
            self.reports.push(Report {
                data: vec![
                    ("maxComplexity", self.rule.max_complexity.to_string()),
                    ("totalComplexity", self.total.to_string()),
                ],
                ..Report::new(span, "cyclomaticComplexity")
            });
        }
    }

    /// Whether a bound attribute's name counts (`isLegacyStructuralDirective`).
    fn counts_bound(&self, name: &str) -> bool {
        let switch = if self.rule.modified {
            "ngSwitch"
        } else {
            "ngSwitchCase"
        };
        matches!(name, "ngForOf" | "ngIf") || name == switch
    }

    /// Whether a plain attribute's name counts (`isLegacySwitchDefault`).
    fn counts_text(&self, name: &str) -> bool {
        !self.rule.modified && name == "ngSwitchDefault"
    }

    /// Walks a list of siblings.
    fn walk(&mut self, template: &Template, nodes: &[Node]) {
        for node in siblings(nodes) {
            match node {
                R3::Element(element) => self.element(template, element),
                R3::If(branches) => {
                    let first = branches[0];
                    let last = branches[branches.len() - 1];
                    self.increment(Span::new(first.span.start as usize, last.span.end as usize));
                    for branch in branches {
                        self.walk(template, &branch.children);
                    }
                }
                R3::For(main, empty) => {
                    let end = empty.map_or(main.span.end, |block| block.span.end);
                    self.increment(Span::new(main.span.start as usize, end as usize));
                    self.walk(template, &main.children);
                    if let Some(empty) = empty {
                        self.walk(template, &empty.children);
                    }
                }
                R3::Other(Node::Block(block)) => self.block(template, block),
                R3::Text(_) | R3::Other(_) => {}
            }
        }
    }

    /// Walks a block other than `@if` and `@for`.
    fn block(&mut self, template: &Template, block: &Block) {
        if block.name != "switch" {
            self.walk(template, &block.children);
            return;
        }
        if self.rule.modified {
            self.increment(block.span);
        }
        for group in switch_groups(block).groups {
            for case in &group.cases {
                if !self.rule.modified {
                    self.increment(case.span);
                }
            }
            self.walk(template, &group.body.children);
        }
    }

    /// Walks an element: its structural directive's bindings, its children, its inputs, then
    /// its plain attributes; a `<ng-template>`'s children, then its inputs.
    fn element(&mut self, template: &Template, element: &Element) {
        for each in template_attrs(template.allocator, element) {
            let counted = if each.value.is_some() {
                self.counts_bound(&each.name)
            } else {
                self.counts_text(&each.name)
            };
            if counted {
                self.increment(each.span);
            }
        }
        self.walk(template, &element.children);
        let name = split_ns_name(&element.name).1;
        if name == "ng-content" {
            return;
        }
        let attributes = bound3(element);
        for each in attributes.iter().filter(|each| each.role == Role::Input) {
            if self.counts_bound(&each.name) {
                self.increment(each.attribute.span);
            }
        }
        if name == "ng-template" {
            return;
        }
        for each in attributes.iter().filter(|each| each.role == Role::Text) {
            if self.counts_text(&each.name) {
                self.increment(each.attribute.span);
            }
        }
    }
}

impl TemplateRule for CyclomaticComplexity {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/cyclomatic-complexity"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "cyclomaticComplexity",
            "The cyclomatic complexity {{totalComplexity}} exceeds the defined limit {{maxComplexity}}",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let mut counter = Counter {
            rule: self,
            total: 0,
            reports,
        };
        counter.walk(template, template.nodes);
    }
}

#[cfg(test)]
mod tests {
    use super::CyclomaticComplexity;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("cyclomatic-complexity.txt", &[], &|options| {
            Box::new(CyclomaticComplexity {
                max_complexity: options
                    .and_then(|each| each.number("maxComplexity"))
                    .map_or(5, |each| format!("{each}").parse().unwrap_or(5)),
                modified: options.and_then(|each| each.text("variant")) == Some("modified"),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 8 valid and 5 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (8, 5, 0));
    }
}

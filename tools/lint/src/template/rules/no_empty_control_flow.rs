//! `@angular-eslint/template/no-empty-control-flow`: no empty `@if`, `@else if`, `@else`,
//! `@for`, `@empty`, `@switch`, nor group of `@case`s (`@angular-eslint/eslint-plugin-template`
//! 22.6.0, `src/rules/no-empty-control-flow.ts`): a block is empty when it has no child but
//! comments, which Angular's tree leaves out, or only whitespace between its braces; a `@switch`
//! when it has no case, a group of cases when its body is empty, reported at its first case.
//! Reported over the block's name.

use super::{Report, Template, TemplateRule, switch_groups};
use crate::template::tree::{Block, Node};

/// The rule.
pub struct NoEmptyControlFlow;

/// Whether a block's body is empty (`children.length === 0 || isEmpty(…)`).
fn is_empty(text: &str, block: &Block) -> bool {
    let no_child = block
        .children
        .iter()
        .all(|child| matches!(child, Node::Comment(_)));
    let blank = block.end_span.is_some_and(|end| {
        text.get(block.start_span.end as usize..end.start as usize)
            .is_some_and(|inner| inner.trim().is_empty())
    });
    no_child || blank
}

impl TemplateRule for NoEmptyControlFlow {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/no-empty-control-flow"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("noEmptyControlFlow", "Unexpected empty control flow block.")]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        template.walk(&mut |node, _| {
            let Node::Block(block) = node else {
                return;
            };
            match block.name.as_str() {
                "if" | "else if" | "else" | "for" | "empty" => {
                    if is_empty(template.text, block) {
                        reports.push(Report::new(block.name_span, "noEmptyControlFlow"));
                    }
                }
                "switch" => {
                    let switch = switch_groups(block);
                    if switch.groups.is_empty() {
                        reports.push(Report::new(block.name_span, "noEmptyControlFlow"));
                    }
                    for group in &switch.groups {
                        if is_empty(template.text, group.body)
                            && let Some(first) = group.cases.first()
                        {
                            reports.push(Report::new(first.name_span, "noEmptyControlFlow"));
                        }
                    }
                }
                _ => {}
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::NoEmptyControlFlow;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("no-empty-control-flow.txt", &[], &|_| {
            Box::new(NoEmptyControlFlow)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 9 valid and 16 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (9, 16, 0));
    }
}

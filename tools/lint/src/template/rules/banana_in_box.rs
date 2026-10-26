//! `@angular-eslint/template/banana-in-box`: a two-way binding written `([x])` is an event
//! named `[x]`, fixed into `[(x)]` (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/banana-in-box.ts`); reported over the whole attribute, the event's source span.
//! Angular's compiler also holds it, NG8101 `invalidBananaInBox` (REF-ANGULAR-EXTENDED-DIAGNOSTICS).

use super::super::bindings::{AttributeKind, attribute_kind};
use super::super::tree::Node;
use super::{Report, Template, TemplateRule};
use crate::finding::Fix;

/// The rule.
pub struct BananaInBox;

impl TemplateRule for BananaInBox {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/banana-in-box"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "bananaInBox",
            "Invalid binding syntax. Use [(expr)] instead",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        template.walk(&mut |node, _| {
            let Node::Element(element) = node else {
                return;
            };
            for attribute in &element.attrs {
                let (kind, name) = attribute_kind(&attribute.name);
                // `INVALID_PATTERN`, `/\[(.*)\]/`: from the first `[` to the last `]`.
                let (Some(open), Some(close)) = (name.find('['), name.rfind(']')) else {
                    continue;
                };
                if kind != AttributeKind::Event || close < open {
                    continue;
                }
                let start = attribute.span.start;
                let end = start + super::super::lexer::offset(name.len() + 2);
                reports.push(Report {
                    fix: Some(Fix {
                        start,
                        end,
                        text: format!("[({})]", &name[open + 1..close]),
                    }),
                    ..Report::new(attribute.span, "bananaInBox")
                });
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::BananaInBox;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("banana-in-box.txt", &[], &|_| Box::new(BananaInBox));

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 3 valid and 2 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (3, 2, 0));
    }
}

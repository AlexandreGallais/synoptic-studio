//! `@angular-eslint/template/require-switch-default`: every `@switch` has a `@default`, or the
//! exhaustive `@default never;` (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/require-switch-default.ts`), reported over the switch's name.

use super::{Report, Template, TemplateRule, switch_groups};
use crate::template::tree::Node;

/// The rule.
pub struct RequireSwitchDefault;

impl TemplateRule for RequireSwitchDefault {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/require-switch-default"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "requireSwitchDefault",
            "Switch should have a default block. Add `@default { ... }`, or `@default never` to enforce a compile-time exhaustive switch.",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        template.walk(&mut |node, _| {
            let Node::Block(block) = node else {
                return;
            };
            if block.name != "switch" {
                return;
            }
            let switch = switch_groups(block);
            let has_default = switch
                .groups
                .iter()
                .any(|group| group.cases.iter().any(|case| case.name == "default"));
            if !switch.exhaustive && !has_default {
                reports.push(Report::new(block.name_span, "requireSwitchDefault"));
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::RequireSwitchDefault;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("require-switch-default.txt", &[], &|_| {
            Box::new(RequireSwitchDefault)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 6 valid and 3 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (6, 3, 0));
    }
}

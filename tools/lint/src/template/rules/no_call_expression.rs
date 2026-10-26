//! `@angular-eslint/template/no-call-expression`: no call in a template but in an event's
//! statements, a call running at every change detection (`@angular-eslint/eslint-plugin-template`
//! 22.6.0, `src/rules/no-call-expression.ts`): each call, `$any(…)` and optional calls
//! (`f?.()`) aside, is reported over itself, unless its callee's name is in `allowList`, starts
//! with `allowPrefix` or ends with `allowSuffix`. The project runs the plugin's defaults, no
//! call allowed.

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;

use super::{Report, Site, Template, TemplateRule, each_node, placed};

/// The rule and its options.
pub struct NoCallExpression {
    /// The callees allowed by name.
    pub allow_list: &'static [&'static str],
    /// The prefix of the callees allowed.
    pub allow_prefix: Option<&'static str>,
    /// The suffix of the callees allowed.
    pub allow_suffix: Option<&'static str>,
}

impl NoCallExpression {
    /// The plugin's defaults, which the project runs.
    pub const PROJECT: Self = Self {
        allow_list: &[],
        allow_prefix: None,
        allow_suffix: None,
    };
}

/// The name of a callee, when it reads one.
fn callee_name<'r>(callee: &'r Expression) -> Option<&'r str> {
    match callee {
        Expression::Identifier(identifier) => Some(identifier.name.as_str()),
        Expression::StaticMemberExpression(member) => Some(member.property.name.as_str()),
        _ => None,
    }
}

impl TemplateRule for NoCallExpression {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/no-call-expression"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("noCallExpression", "Avoid calling expressions in templates")]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for written in template.expressions() {
            if written.site == Site::Event {
                continue;
            }
            each_node(&written, &mut |node, bound| {
                let AstKind::CallExpression(call) = node else {
                    return;
                };
                if call.optional {
                    return;
                }
                let name = callee_name(&call.callee);
                let allowed = name.is_some_and(|name| {
                    !name.is_empty()
                        && (name == "$any"
                            || self.allow_list.contains(&name)
                            || self
                                .allow_prefix
                                .is_some_and(|prefix| name.starts_with(prefix))
                            || self
                                .allow_suffix
                                .is_some_and(|suffix| name.ends_with(suffix)))
                });
                if !allowed {
                    reports.push(Report::new(placed(bound, call.span), "noCallExpression"));
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoCallExpression;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("no-call-expression.txt", &[], &|options| {
            let list: Vec<&'static str> = options
                .and_then(|each| each.texts("allowList"))
                .unwrap_or_default()
                .into_iter()
                .map(|each| &*String::leak(each))
                .collect();
            let text = |key: &str| {
                options
                    .and_then(|each| each.text(key))
                    .map(|each| &*String::leak(each.to_owned()))
            };
            Box::new(NoCallExpression {
                allow_list: Vec::leak(list),
                allow_prefix: text("allowPrefix"),
                allow_suffix: text("allowSuffix"),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 12 valid and 15 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (12, 15, 0));
    }
}

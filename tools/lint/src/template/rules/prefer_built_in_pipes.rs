//! `@angular-eslint/template/prefer-built-in-pipes`: the `lowercase`, `uppercase` and
//! `titlecase` pipes rather than calling `toLowerCase`, `toUpperCase`, `toLocaleLowerCase` or
//! `toLocaleUpperCase` in a template (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/prefer-built-in-pipes.ts`): a call of a method of `disallowList` is reported
//! over the method read, unless in an event's statements while `allowInOutputHandlers`. The
//! project runs the plugin's defaults.

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;

use super::{Report, Template, TemplateRule, each_node, placed};
use crate::template::bindings::{AttributeKind, Value, read_attribute};
use crate::template::rules::{Site, Written};

/// The rule and its options.
pub struct PreferBuiltInPipes {
    /// The methods reported.
    pub disallow_list: &'static [&'static str],
    /// Whether an event's statements may call them.
    pub allow_in_output_handlers: bool,
}

impl PreferBuiltInPipes {
    /// The plugin's defaults, which the project runs.
    pub const PROJECT: Self = Self {
        disallow_list: &DEFAULT_DISALLOW_LIST,
        allow_in_output_handlers: true,
    };
}

/// The methods the plugin reports by default (`DEFAULT_DISALLOW_LIST`).
pub const DEFAULT_DISALLOW_LIST: [&str; 4] = [
    "toLowerCase",
    "toUpperCase",
    "toLocaleLowerCase",
    "toLocaleUpperCase",
];

impl PreferBuiltInPipes {
    /// Reports the calls of a written expression.
    fn check_written(&self, written: &Written, reports: &mut Vec<Report>) {
        each_node(written, &mut |node, bound| {
            let AstKind::CallExpression(call) = node else {
                return;
            };
            if call.optional {
                return;
            }
            let (name, span) = match &call.callee {
                Expression::StaticMemberExpression(member) if !member.optional => {
                    (member.property.name.as_str(), member.span)
                }
                Expression::Identifier(identifier) => (identifier.name.as_str(), identifier.span),
                _ => return,
            };
            if self.disallow_list.contains(&name) {
                reports.push(Report {
                    data: vec![("methodName", name.to_owned())],
                    ..Report::new(placed(bound, span), "preferBuiltInPipes")
                });
            }
        });
    }
}

impl TemplateRule for PreferBuiltInPipes {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/prefer-built-in-pipes"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "preferBuiltInPipes",
            "Avoid using method \"{{ methodName }}\" in templates. Prefer the Angular built-in pipes instead (e.g. lowercase, uppercase, titlecase).",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for written in template.expressions() {
            // An event's statements; a two-way binding is read here as its input, below as its
            // output.
            let in_handler = written.site == Site::Event;
            if !(in_handler && self.allow_in_output_handlers) {
                self.check_written(&written, reports);
            }
        }
        if self.allow_in_output_handlers {
            return;
        }
        for (element, _) in template.elements() {
            for attribute in &element.attrs {
                let Ok(read) = read_attribute(template.allocator, &element.name, attribute) else {
                    continue;
                };
                if let (AttributeKind::TwoWay, Value::Binding(bound)) = (read.kind, read.value) {
                    self.check_written(
                        &Written {
                            bound,
                            site: Site::Event,
                        },
                        reports,
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_DISALLOW_LIST, PreferBuiltInPipes};
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("prefer-built-in-pipes.txt", &[], &|options| {
            let disallow: Option<Vec<&'static str>> = options
                .and_then(|each| each.texts("disallowList"))
                .map(|list| list.into_iter().map(|each| &*String::leak(each)).collect());
            Box::new(PreferBuiltInPipes {
                disallow_list: disallow
                    .map_or(&DEFAULT_DISALLOW_LIST[..], |list| &*Vec::leak(list)),
                allow_in_output_handlers: options
                    .and_then(|each| each.flag("allowInOutputHandlers"))
                    .unwrap_or(true),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 12 valid and 9 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (12, 9, 0));
    }
}

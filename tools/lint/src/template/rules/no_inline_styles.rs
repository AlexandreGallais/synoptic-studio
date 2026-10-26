//! `@angular-eslint/template/no-inline-styles`: no inline style on an element, a `style`
//! attribute, a `[style]`, `[style.x]` or `[attr.style]` binding, nor `[ngStyle]`
//! (`@angular-eslint/eslint-plugin-template` 22.6.0, `src/rules/no-inline-styles.ts` and
//! `src/utils/is-constant-expression.ts`), the element reported; `allowNgStyle` and
//! `allowBindToStyle` allow their bindings always (`true`) or when not constant (`dynamic`).
//! The project runs the plugin's defaults, every inline style reported.

use super::r3::{is_constant, of_bound};
use super::{Report, Role, Template, TemplateRule, bound3, is_plain_element};
use crate::template::bindings::{AttributeKind, Value, attribute_kind, read_attribute};
use crate::template::tree::Element;

/// Whether a binding is allowed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Allow {
    /// Never.
    Never,
    /// Always.
    Always,
    /// When its value is not a constant.
    Dynamic,
}

/// The rule and its options.
pub struct NoInlineStyles {
    /// Whether `[ngStyle]` is allowed.
    pub allow_ng_style: Allow,
    /// Whether `[style]`, `[style.x]` and an interpolated `style` are allowed.
    pub allow_bind_to_style: Allow,
}

impl NoInlineStyles {
    /// The plugin's defaults, which the project runs.
    pub const PROJECT: Self = Self {
        allow_ng_style: Allow::Never,
        allow_bind_to_style: Allow::Never,
    };
}

/// Whether a binding's value is allowed under an option (`isAllowed`).
fn is_allowed(
    template: &Template,
    element: &Element,
    attribute: &crate::template::tree::Attribute,
    option: Allow,
) -> bool {
    match option {
        Allow::Always => true,
        Allow::Never => false,
        Allow::Dynamic => {
            let Ok(read) = read_attribute(template.allocator, &element.name, attribute) else {
                return true;
            };
            let constant = match read.value {
                Value::Binding(bound) => is_constant(&of_bound(&bound)),
                Value::Interpolations(bounds) => {
                    bounds.iter().all(|bound| is_constant(&of_bound(bound)))
                }
                _ => false,
            };
            !constant
        }
    }
}

impl TemplateRule for NoInlineStyles {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/no-inline-styles"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "noInlineStyles",
            "<{{element}}/> element should not have inline styles via style attribute. Please use classes instead.",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, _) in template.elements() {
            if !is_plain_element(element) {
                continue;
            }
            let invalid = bound3(element).iter().any(|each| match each.role {
                Role::Text => each.name == "style",
                Role::Output => false,
                Role::Input => {
                    let (kind, inner) = attribute_kind(&each.attribute.name);
                    if kind == AttributeKind::Animation {
                        return false;
                    }
                    if inner.starts_with("attr.") {
                        return each.name == "style";
                    }
                    if inner.starts_with("style.") {
                        return !is_allowed(
                            template,
                            element,
                            each.attribute,
                            self.allow_bind_to_style,
                        );
                    }
                    if inner.starts_with("class.") || inner.starts_with('@') {
                        return false;
                    }
                    match each.name.as_str() {
                        "style" => {
                            !is_allowed(template, element, each.attribute, self.allow_bind_to_style)
                        }
                        "ngStyle" => {
                            !is_allowed(template, element, each.attribute, self.allow_ng_style)
                        }
                        _ => false,
                    }
                }
            });
            if invalid {
                reports.push(Report {
                    data: vec![("element", element.name.clone())],
                    ..Report::new(element.span, "noInlineStyles")
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Allow, NoInlineStyles};
    use crate::template::corpus::Value;
    use crate::template::rules::tester::run;

    /// An option as the case writes it.
    fn allow(value: Option<&Value>) -> Allow {
        match value {
            Some(Value::Bool(true)) => Allow::Always,
            Some(Value::Text(text)) if text == "dynamic" => Allow::Dynamic,
            _ => Allow::Never,
        }
    }

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("no-inline-styles.txt", &[], &|options| {
            Box::new(NoInlineStyles {
                allow_ng_style: allow(options.and_then(|each| each.get("allowNgStyle"))),
                allow_bind_to_style: allow(options.and_then(|each| each.get("allowBindToStyle"))),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 45 valid and 24 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (45, 24, 0));
    }
}

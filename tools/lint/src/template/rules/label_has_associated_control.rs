//! `@angular-eslint/template/label-has-associated-control`: a label names its control
//! (`@angular-eslint/eslint-plugin-template` 22.6.0, `src/rules/label-has-associated-control.ts`):
//! a label component (`<label>` by default, its name in lower case) without one of its inputs
//! (`for`, `htmlFor`), as a plain attribute or a binding, and holding no control through its
//! child elements is reported over itself. The controls are `input`, `meter`, `output`,
//! `progress`, `select` and `textarea`, and `controlComponents`; `labelComponents` adds label
//! components or replaces one of the same selector; with `checkIds`, the label's first input
//! must be a text some control has as its plain `id` too. The project runs the plugin's
//! defaults.

use super::a11y::is_child_node_of;
use super::{Report, Role, Template, TemplateRule, bound3, is_plain_element};
use crate::template::tree::Element;

/// A label component: its selector and the inputs that name its control.
#[derive(Clone, Copy, Debug)]
pub struct LabelComponent {
    /// The element's name.
    pub selector: &'static str,
    /// The inputs naming the control, in the order looked up.
    pub inputs: &'static [&'static str],
}

/// The rule and its options.
pub struct LabelHasAssociatedControl {
    /// The controls, beside the plugin's.
    pub control_components: &'static [&'static str],
    /// The label components, beside or instead of the plugin's.
    pub label_components: &'static [LabelComponent],
    /// Whether the control a label names must be in the template.
    pub check_ids: bool,
}

impl LabelHasAssociatedControl {
    /// The plugin's defaults, which the project runs.
    pub const PROJECT: Self = Self {
        control_components: &[],
        label_components: &[],
        check_ids: false,
    };
}

/// The controls (`DEFAULT_CONTROL_COMPONENTS`).
const CONTROLS: [&str; 6] = ["input", "meter", "output", "progress", "select", "textarea"];

/// The label component (`DEFAULT_LABEL_COMPONENTS`).
const LABEL: LabelComponent = LabelComponent {
    selector: "label",
    inputs: &["for", "htmlFor"],
};

/// What an attribute holds: a plain attribute's text, or a binding's expression, never equal to
/// another value.
#[derive(Clone, Copy, Debug)]
enum Held<'e> {
    /// A text.
    Text(&'e str),
    /// An expression.
    Expression,
}

/// Whether a control has a plain `id` of a text.
fn has_id(control: &Element, id: &str) -> bool {
    bound3(control)
        .iter()
        .any(|each| each.role == Role::Text && each.name == "id" && each.attribute.value == id)
}

impl LabelHasAssociatedControl {
    /// The label components: the plugin's, each replaced by an option of its selector, then the
    /// options' others.
    fn labels(&self) -> Vec<LabelComponent> {
        let mut labels = vec![LABEL];
        for each in self.label_components {
            match labels
                .iter_mut()
                .find(|label| label.selector == each.selector)
            {
                Some(label) => *label = *each,
                None => labels.push(*each),
            }
        }
        labels
    }

    /// Whether a label names a control by its inputs.
    fn has_for(&self, label: &Element, component: &LabelComponent, controls: &[&Element]) -> bool {
        // `new Map([...attributes, ...inputs])`: the last of a name wins.
        let mut held: Vec<(String, Held)> = Vec::new();
        let attributes = bound3(label);
        let texts = attributes.iter().filter(|each| each.role == Role::Text);
        let inputs = attributes.iter().filter(|each| each.role == Role::Input);
        for each in texts.chain(inputs) {
            let value = if each.role == Role::Text {
                Held::Text(&each.attribute.value)
            } else {
                Held::Expression
            };
            held.retain(|(name, _)| *name != each.name);
            held.push((each.name.clone(), value));
        }
        let first = component.inputs.iter().find_map(|input| {
            held.iter()
                .find(|(name, _)| name == input)
                .map(|(_, value)| *value)
        });
        match first {
            Some(_) if !self.check_ids => true,
            Some(Held::Text(id)) => controls.iter().any(|control| has_id(control, id)),
            // No input, or an expression, never equal to a control's text.
            None | Some(Held::Expression) => false,
        }
    }
}

impl TemplateRule for LabelHasAssociatedControl {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/label-has-associated-control"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "labelHasAssociatedControl",
            "A label component must be associated with a form element",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let names: Vec<&str> = CONTROLS
            .iter()
            .chain(self.control_components)
            .copied()
            .collect();
        let labels = self.labels();
        let elements: Vec<&Element> = template
            .elements()
            .into_iter()
            .map(|(element, _)| element)
            .filter(|element| is_plain_element(element))
            .collect();
        let controls: Vec<&Element> = elements
            .iter()
            .filter(|element| names.contains(&element.name.to_lowercase().as_str()))
            .copied()
            .collect();
        for label in elements {
            let tag = label.name.to_lowercase();
            let Some(component) = labels.iter().find(|each| each.selector == tag) else {
                continue;
            };
            if self.has_for(label, component, &controls)
                || names.iter().any(|name| is_child_node_of(label, name))
            {
                continue;
            }
            reports.push(Report::new(label.span, "labelHasAssociatedControl"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LabelComponent, LabelHasAssociatedControl};
    use crate::template::corpus::Value;
    use crate::template::rules::tester::run;

    /// Strings kept for the test's life.
    fn leaked(texts: Vec<String>) -> &'static [&'static str] {
        Vec::leak(texts.into_iter().map(|each| &*String::leak(each)).collect())
    }

    #[test]
    fn f52_ac3_passes_the_plugin_s_cases() {
        let tally = run("label-has-associated-control.txt", &[], &|options| {
            let labels: Vec<LabelComponent> =
                match options.and_then(|each| each.get("labelComponents")) {
                    Some(Value::List(list)) => list
                        .iter()
                        .map(|each| LabelComponent {
                            selector: String::leak(
                                each.text("selector").unwrap_or_default().to_owned(),
                            ),
                            inputs: leaked(each.texts("inputs").unwrap_or_default()),
                        })
                        .collect(),
                    _ => Vec::new(),
                };
            Box::new(LabelHasAssociatedControl {
                control_components: leaked(
                    options
                        .and_then(|each| each.texts("controlComponents"))
                        .unwrap_or_default(),
                ),
                label_components: Vec::leak(labels),
                check_ids: options
                    .and_then(|each| each.flag("checkIds"))
                    .unwrap_or(false),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 6 valid and 5 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (6, 5, 0));
    }
}

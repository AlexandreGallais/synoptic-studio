//! `@angular-eslint/template/elements-content`: an anchor, a button and a heading have content
//! (`@angular-eslint/eslint-plugin-template` 22.6.0, `src/rules/elements-content.ts`): an `<a>`,
//! `<button>` or `<h1>` to `<h6>`, any case, without children in Angular's tree is reported over
//! itself, unless hidden from a screen reader or carrying a plain attribute or an input of the
//! plugin's list (`aria-label`, `innerHtml`, `innerHTML`, `innerText`, `outerHTML`,
//! `textContent`, `title`) or of `allowList`. The project runs the plugin's default, its list
//! alone.

use super::a11y::{attributes_and_inputs, is_hidden_from_screen_reader};
use super::r3::siblings;
use super::{Report, Template, TemplateRule, is_plain_element};

/// The rule and its option.
pub struct ElementsContent {
    /// The attributes and inputs that give an element its content, beside the plugin's.
    pub allow_list: &'static [&'static str],
}

impl ElementsContent {
    /// The plugin's default, which the project runs.
    pub const PROJECT: Self = Self { allow_list: &[] };
}

/// The attributes and inputs that give an element its content (`DEFAULT_SAFELIST_ATTRIBUTES`).
const SAFELIST: [&str; 7] = [
    "aria-label",
    "innerHtml",
    "innerHTML",
    "innerText",
    "outerHTML",
    "textContent",
    "title",
];

impl TemplateRule for ElementsContent {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/elements-content"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("elementsContent", "<{{element}}> should have content")]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, ancestors) in template.elements() {
            let name = element.name.to_lowercase();
            if !matches!(
                name.as_str(),
                "a" | "button" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
            ) || !is_plain_element(element)
                || !siblings(&element.children).is_empty()
                || is_hidden_from_screen_reader(template.allocator, element, &ancestors)
            {
                continue;
            }
            let allowed = attributes_and_inputs(element).iter().any(|each| {
                SAFELIST.contains(&each.name.as_str())
                    || self.allow_list.contains(&each.name.as_str())
            });
            if !allowed {
                reports.push(Report {
                    data: vec![("element", element.name.clone())],
                    ..Report::new(element.span, "elementsContent")
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ElementsContent;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac3_passes_the_plugin_s_cases() {
        let tally = run("elements-content.txt", &[], &|options| {
            let list: Vec<&'static str> = options
                .and_then(|each| each.texts("allowList"))
                .unwrap_or_default()
                .into_iter()
                .map(|each| &*String::leak(each))
                .collect();
            Box::new(ElementsContent {
                allow_list: Vec::leak(list),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 3 valid and 5 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (3, 5, 0));
    }
}

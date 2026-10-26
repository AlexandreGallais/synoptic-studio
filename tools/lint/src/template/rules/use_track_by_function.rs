//! `@angular-eslint/template/use-track-by-function`: an `ngFor` tracks its items by a
//! `trackBy` function (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/use-track-by-function.ts`): a bound `ngForOf` without `ngForTrackBy`, nor a name
//! of `alias`, among the inputs beside it is reported over itself; one of a microsyntax
//! (`*ngFor`) without a `trackBy` among its bindings, from the `*` to the character after its
//! last binding. The project runs
//! the plugin's default, no alias.

use super::{Report, Role, Template, TemplateRule, bound3, template_attrs};

/// The rule and its option.
pub struct UseTrackByFunction {
    /// The names that stand for `ngForTrackBy`.
    pub alias: &'static [&'static str],
}

impl UseTrackByFunction {
    /// The plugin's default, which the project runs.
    pub const PROJECT: Self = Self { alias: &[] };

    /// Whether a name tracks the items (`isNgForTrackByFactory`).
    fn tracks(&self, name: &str) -> bool {
        name == "ngForTrackBy" || self.alias.contains(&name)
    }
}

impl TemplateRule for UseTrackByFunction {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/use-track-by-function"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "useTrackByFunction",
            "Missing trackBy function in ngFor directive",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for (element, _) in template.elements() {
            let inputs: Vec<_> = bound3(element)
                .into_iter()
                .filter(|each| each.role == Role::Input)
                .collect();
            if !inputs.iter().any(|each| self.tracks(&each.name)) {
                for each in inputs.iter().filter(|each| each.name == "ngForOf") {
                    reports.push(Report::new(each.attribute.span, "useTrackByFunction"));
                }
            }
            let attrs = template_attrs(template.allocator, element);
            let bound_for = attrs
                .iter()
                .find(|each| each.name == "ngForOf" && each.value.is_some());
            if let Some(each) = bound_for
                && !attrs.iter().any(|other| self.tracks(&other.name))
            {
                // From the `*` before the first binding to the character after the last.
                let end = attrs.last().map_or(each.span.end, |last| last.span.end + 1);
                let span = crate::template::lexer::Span::new(
                    each.attribute.key_span.start as usize,
                    end as usize,
                );
                reports.push(Report::new(span, "useTrackByFunction"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UseTrackByFunction;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("use-track-by-function.txt", &[], &|options| {
            let alias: Vec<&'static str> = options
                .and_then(|each| each.texts("alias"))
                .unwrap_or_default()
                .into_iter()
                .map(|each| &*String::leak(each))
                .collect();
            Box::new(UseTrackByFunction {
                alias: Vec::leak(alias),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 11 valid and 7 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (11, 7, 0));
    }
}

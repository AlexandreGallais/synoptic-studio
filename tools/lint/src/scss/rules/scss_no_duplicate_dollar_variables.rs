//! `no-duplicate-dollar-variables` of stylelint-scss 7.3.0
//! (`src/rules/no-duplicate-dollar-variables/index.js`, REF-STYLELINT-SOURCE): a `$variable`
//! declared again in its block or a block around it, reported over its name; a `!default`
//! declaration counted as the options say (by default, one after another declaration is allowed).
//! Blocks are told apart by their text, as the original keys them.

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;

/// How `!default` declarations count (`ignoreDefaults`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Defaults {
    /// A number: a `!default` declaration is a duplicate once that many came before.
    Count(u32),
    /// `true`: `!default` declarations are never duplicates; `false`: they are as any other.
    Ignored(bool),
}

/// The rule and its options.
pub struct NoDuplicateDollarVariables {
    /// `ignoreInside`: `"at-rule"` or `"nested-at-rule"`.
    pub ignore_inside: Option<String>,
    /// `ignoreInsideAtRules`.
    pub ignore_inside_at_rules: Vec<String>,
    /// `ignoreDefaults`.
    pub defaults: Defaults,
}

impl NoDuplicateDollarVariables {
    /// The project's configuration: the original's defaults.
    pub const PROJECT: Self = Self {
        ignore_inside: None,
        ignore_inside_at_rules: Vec::new(),
        defaults: Defaults::Count(1),
    };
}

/// What is known of a variable in a scope.
#[derive(Clone, Copy, Debug, Default)]
struct Known {
    /// Its `!default` declarations.
    defaults: u32,
    /// Whether it was declared without `!default` (or counted as such).
    declared: bool,
}

/// A scope, keyed by its block's text, with its variables and its inner scopes.
#[derive(Debug, Default)]
struct Scope {
    /// The variables.
    variables: Vec<(String, Known)>,
    /// The inner scopes.
    inner: Vec<(String, Scope)>,
}

impl Scope {
    /// The scope of a path of keys, made when missing (`getScope`).
    fn at(&mut self, path: &[String]) -> &mut Self {
        let Some((first, rest)) = path.split_first() else {
            return self;
        };
        let position = self
            .inner
            .iter()
            .position(|(key, _)| key == first)
            .unwrap_or_else(|| {
                self.inner.push((first.clone(), Self::default()));
                self.inner.len() - 1
            });
        self.inner[position].1.at(rest)
    }

    /// What the outermost scope of a path knows of a variable (`getVariableData`).
    fn known(&self, path: &[String], name: &str) -> Known {
        let mut scope = self;
        for key in path {
            let Some((_, inner)) = scope.inner.iter().find(|(each, _)| each == key) else {
                break;
            };
            scope = inner;
            if let Some((_, known)) = scope.variables.iter().find(|(each, _)| each == name) {
                return *known;
            }
        }
        Known::default()
    }
}

impl SheetRule for NoDuplicateDollarVariables {
    fn name(&self) -> &'static str {
        "scss/no-duplicate-dollar-variables"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[("rejected", "Unexpected duplicate dollar variable {}")]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        let mut root = Scope::default();
        for (index, node) in sheet.all(Kind::Declaration) {
            let parent = sheet.node(node.parent.unwrap_or(0));
            let in_at_rule = parent.is(Kind::AtRule);
            let nested_at_rule = in_at_rule && parent.parent != Some(0);
            if !node.prop.starts_with('$')
                || (in_at_rule && self.ignore_inside.as_deref() == Some("at-rule"))
                || (nested_at_rule && self.ignore_inside.as_deref() == Some("nested-at-rule"))
                || (in_at_rule && self.ignore_inside_at_rules.contains(&parent.name))
            {
                continue;
            }
            let mut path: Vec<String> = sheet
                .ancestors(index)
                .into_iter()
                .map(|at| sheet.source(at).to_owned())
                .collect();
            path.reverse();
            let is_default = node.value.contains("!default");
            let mut known = root.known(&path, &node.prop);
            let declared = if is_default {
                match self.defaults {
                    Defaults::Count(count) => known.defaults >= count,
                    Defaults::Ignored(true) => false,
                    Defaults::Ignored(false) => known.declared,
                }
            } else {
                known.declared
            };
            if declared {
                let (start, end) = sheet.word_range(node, &node.prop);
                reports.push(Report::new(start, end, "rejected").with(&[&node.prop]));
            }
            if is_default {
                known.defaults += 1;
            }
            if !is_default || self.defaults == Defaults::Ignored(false) {
                known.declared = true;
            }
            let scope = root.at(&path);
            scope.variables.retain(|(name, _)| *name != node.prop);
            scope.variables.push((node.prop.clone(), known));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Defaults, NoDuplicateDollarVariables};
    use crate::scss::rules::tester::{Js, run, secondary};

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/no-duplicate-dollar-variables.txt",
            &|config| {
                let options = secondary(config);
                let defaults = match options.and_then(|each| each.get("ignoreDefaults")) {
                    Some(Js::Bool(flag)) => Defaults::Ignored(*flag),
                    Some(Js::Number(count)) => {
                        Defaults::Count(count.to_string().parse().map_err(|_| "a count")?)
                    }
                    _ => Defaults::Count(1),
                };
                Ok(Box::new(NoDuplicateDollarVariables {
                    ignore_inside: options.and_then(|each| match each.get("ignoreInside") {
                        Some(Js::Text(text)) => Some(text.clone()),
                        _ => None,
                    }),
                    ignore_inside_at_rules: options
                        .and_then(|each| each.texts("ignoreInsideAtRules"))
                        .unwrap_or_default(),
                    defaults,
                }))
            },
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (57, 52, 0),
            "{:#?}",
            tally.apart
        );
    }
}

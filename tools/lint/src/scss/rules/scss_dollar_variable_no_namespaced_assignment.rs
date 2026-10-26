//! `dollar-variable-no-namespaced-assignment` of stylelint-scss 7.3.0
//! (`src/rules/dollar-variable-no-namespaced-assignment/index.js`, REF-STYLELINT-SOURCE): a
//! declaration assigning another module's variable (`module.$a: 1`), reported over it.

use super::{Context, Report, SheetRule};
use crate::scss::tree::Kind;

/// The rule.
pub struct DollarVariableNoNamespacedAssignment;

/// Whether a property is `name.$variable` (`/^[^$.]+\.\$./`).
fn is_namespaced(prop: &str) -> bool {
    prop.find('.').is_some_and(|dot| {
        dot > 0
            && !prop[..dot].contains('$')
            && prop[dot + 1..].starts_with('$')
            && prop.len() > dot + 2
    })
}

impl SheetRule for DollarVariableNoNamespacedAssignment {
    fn name(&self) -> &'static str {
        "scss/dollar-variable-no-namespaced-assignment"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "rejected",
            "Unexpected assignment to a namespaced $ variable",
        )]
    }

    fn check(&self, sheet: &Context, reports: &mut Vec<Report>) {
        for (_, node) in sheet.all(Kind::Declaration) {
            if is_namespaced(&node.prop) {
                reports.push(Report::new(
                    node.start,
                    node.end.unwrap_or(sheet.text.len()),
                    "rejected",
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DollarVariableNoNamespacedAssignment;
    use crate::scss::rules::tester::run;

    #[test]
    fn f53_ac2_passes_the_cases_of_stylelint_scss() {
        let tally = run(
            "stylelint-scss/dollar-variable-no-namespaced-assignment.txt",
            &|_| Ok(Box::new(DollarVariableNoNamespacedAssignment)),
            &[],
        );

        assert_eq!(tally.failures, Vec::<String>::new());
        assert_eq!(
            (tally.accepted, tally.rejected, tally.apart_count()),
            (2, 1, 0),
            "{:#?}",
            tally.apart
        );
    }
}

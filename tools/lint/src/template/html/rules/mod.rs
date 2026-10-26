//! The rules of html-validate the catalogue keeps, one module each.

pub mod attribute_allowed_values;
pub mod attribute_misuse;
pub mod deprecated;
pub mod element_permitted_content;
pub mod element_permitted_occurrences;
pub mod element_permitted_order;
pub mod element_permitted_parent;
pub mod element_required_ancestor;
pub mod element_required_attributes;
pub mod element_required_content;
pub mod form_dup_name;
pub mod input_attributes;
pub mod map_dup_name;
pub mod map_id_name;
pub mod no_deprecated_attr;
pub mod no_dup_class;
pub mod no_dup_id;
pub mod no_multiple_main;
pub mod no_raw_characters;
pub mod no_redundant_for;
pub mod no_utf8_bom;
pub mod valid_autocomplete;
pub mod valid_for;
pub mod valid_id;

use crate::template::rules::TemplateRule;

/// The rules, in the order of their names, as the project runs them.
pub const RULES: &[&dyn TemplateRule] = &[
    &attribute_allowed_values::AttributeAllowedValues,
    &attribute_misuse::AttributeMisuse,
    &deprecated::Deprecated::PROJECT,
    &element_permitted_content::ElementPermittedContent,
    &element_permitted_occurrences::ElementPermittedOccurrences,
    &element_permitted_order::ElementPermittedOrder,
    &element_permitted_parent::ElementPermittedParent,
    &element_required_ancestor::ElementRequiredAncestor,
    &element_required_attributes::ElementRequiredAttributes,
    &element_required_content::ElementRequiredContent,
    &form_dup_name::FormDupName::PROJECT,
    &input_attributes::InputAttributes,
    &map_dup_name::MapDupName,
    &map_id_name::MapIdName,
    &no_deprecated_attr::NoDeprecatedAttr,
    &no_dup_class::NoDupClass,
    &no_dup_id::NoDupId::PROJECT,
    &no_multiple_main::NoMultipleMain,
    &no_raw_characters::NoRawCharacters::PROJECT,
    &no_redundant_for::NoRedundantFor,
    &no_utf8_bom::NoUtf8Bom,
    &valid_autocomplete::ValidAutocomplete,
    &valid_for::ValidFor,
    &valid_id::ValidId::PROJECT,
];

#[cfg(test)]
mod tests {
    use super::RULES;

    #[test]
    fn names_each_rule_once_in_order() {
        let names: Vec<&str> = RULES.iter().map(|rule| rule.name()).collect();
        let mut sorted = names.clone();
        sorted.sort_unstable();
        sorted.dedup();

        assert_eq!(names, sorted);
        assert!(names.iter().all(|name| name.starts_with("html-validate/")));
    }
}

#[cfg(test)]
mod elements {
    use super::{
        attribute_allowed_values::AttributeAllowedValues, attribute_misuse::AttributeMisuse,
        deprecated::Deprecated, element_permitted_content::ElementPermittedContent,
        element_permitted_occurrences::ElementPermittedOccurrences,
        element_permitted_order::ElementPermittedOrder,
        element_permitted_parent::ElementPermittedParent,
        element_required_ancestor::ElementRequiredAncestor,
        element_required_attributes::ElementRequiredAttributes,
        element_required_content::ElementRequiredContent, form_dup_name::FormDupName,
        input_attributes::InputAttributes, map_dup_name::MapDupName, map_id_name::MapIdName,
        no_deprecated_attr::NoDeprecatedAttr, no_dup_class::NoDupClass, no_dup_id::NoDupId,
        no_multiple_main::NoMultipleMain, no_raw_characters::NoRawCharacters,
        no_redundant_for::NoRedundantFor, no_utf8_bom::NoUtf8Bom,
        valid_autocomplete::ValidAutocomplete, valid_for::ValidFor, valid_id::ValidId,
    };
    use crate::template::html::cases::run_elements;
    use crate::template::rules::TemplateRule;

    /// The rules the element tests run (`html-validate:recommended` but those
    /// `src/elements/html5.spec.ts` turns off) that the catalogue keeps, under html-validate's
    /// names, with the preset's options.
    const RUN: &[(&str, &dyn TemplateRule)] = &[
        ("attribute-allowed-values", &AttributeAllowedValues),
        ("attribute-misuse", &AttributeMisuse),
        ("deprecated", &Deprecated::PROJECT),
        ("element-permitted-content", &ElementPermittedContent),
        (
            "element-permitted-occurrences",
            &ElementPermittedOccurrences,
        ),
        ("element-permitted-order", &ElementPermittedOrder),
        ("element-permitted-parent", &ElementPermittedParent),
        ("element-required-ancestor", &ElementRequiredAncestor),
        ("element-required-attributes", &ElementRequiredAttributes),
        ("element-required-content", &ElementRequiredContent),
        ("form-dup-name", &FormDupName::PROJECT),
        ("input-attributes", &InputAttributes),
        ("map-dup-name", &MapDupName),
        ("map-id-name", &MapIdName),
        ("no-deprecated-attr", &NoDeprecatedAttr),
        ("no-dup-class", &NoDupClass),
        ("no-dup-id", &NoDupId::PROJECT),
        ("no-multiple-main", &NoMultipleMain),
        ("no-raw-characters", &NoRawCharacters::PROJECT),
        ("no-redundant-for", &NoRedundantFor),
        ("no-utf8-bom", &NoUtf8Bom),
        ("valid-autocomplete", &ValidAutocomplete),
        ("valid-for", &ValidFor),
        ("valid-id", &ValidId::PROJECT),
    ];

    /// The element files whose errors follow from a parser other than Angular's, set apart.
    const KNOWN: &[(&str, &str)] = &[
        // `<figure>`, `<details>`, `<li>`, `<dt>` and `<dd>` close an open `<p>` in HTML's parser
        // and not in Angular's (`closedByChildren` of `ml_parser/html_tags.ts`), so the
        // template nests them in the `<p>`, which the rule rightly reports.
        (
            "p-valid",
            "Angular's parser closes `<p>` on fewer elements than HTML's",
        ),
        // `<custom-form>` is given `form`'s metadata by the tests' configuration.
        ("form-invalid", "element metadata of its own"),
    ];

    #[test]
    fn f53_ac1_passes_the_element_files_of_html_validate() {
        let tally = run_elements(RUN, KNOWN);

        assert!(
            tally.failures.is_empty(),
            "{}",
            tally.failures.join("\n=====\n")
        );
        assert_eq!(
            (tally.files, tally.errors, tally.refused.len()),
            (256, 384, 36),
            "{:?}",
            tally.refused
        );
    }
}

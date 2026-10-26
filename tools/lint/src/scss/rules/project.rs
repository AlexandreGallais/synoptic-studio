//! The rules the project runs on its style sheets, each with its configuration (EN-167): the 62
//! of Stylelint and the 40 of stylelint-scss the catalogue keeps, in the catalogue's order, the two
//! of stylelint-order with the Recess order (EN-168), and
//! the files where a rule is off. The colours come from the tokens (the Product Owner's
//! preference): a hexadecimal colour, a named one or a colour function is refused everywhere but
//! in the tokens' files.

use std::sync::LazyLock;

use super::SheetRule;
use super::scss_ampersand::{Ampersand, AmpersandRules};
use super::scss_color_functions::{ColorFunctions, Colour};
use super::scss_load_rules::LoadRules;
use super::scss_patterns::{NamePattern, Named};
use super::scss_quote_functions::{QuoteFunctions, Quoting};
use super::{
    at_rule_disallowed_list, at_rule_empty_line_before, at_rule_no_deprecated,
    at_rule_no_vendor_prefix, block_no_empty, block_no_redundant_nested_style_rules, color_named,
    color_no_hex, color_no_invalid_hex, comment_empty_line_before, comment_whitespace_inside,
    container_name_pattern, custom_property_empty_line_before,
    custom_property_no_missing_var_function, custom_property_pattern,
    declaration_block_no_duplicate_custom_properties, declaration_block_no_duplicate_properties,
    declaration_block_no_redundant_longhand_properties,
    declaration_block_no_shorthand_property_overrides, declaration_no_important,
    declaration_property_value_keyword_no_deprecated, font_family_name_quotes,
    font_family_no_duplicate_names, font_family_no_missing_generic_family_keyword,
    function_disallowed_list, function_linear_gradient_no_nonstandard_direction,
    function_name_case, function_url_no_scheme_relative, function_url_quotes,
    keyframe_block_no_duplicate_selectors, keyframe_selector_notation, keyframes_name_pattern,
    layer_name_pattern, length_zero_no_unit, max_nesting_depth, media_feature_name_no_unknown,
    media_feature_name_no_vendor_prefix, media_type_no_deprecated, named_grid_areas_no_invalid,
    nesting_selector_no_missing_scoping_root, no_duplicate_selectors, no_empty_source,
    no_irregular_whitespace, number_max_precision, order, properties_order, property_no_deprecated,
    property_no_vendor_prefix, rule_empty_line_before, scss_at_extend_no_missing_placeholder,
    scss_at_if_no_null, scss_at_mixin_argumentless_call_parentheses, scss_at_root_no_redundant,
    scss_at_rule_conditional_no_parentheses, scss_at_rule_no_unknown,
    scss_at_use_no_redundant_alias, scss_at_use_no_unnamespaced, scss_block_no_redundant_nesting,
    scss_comment_no_empty, scss_declaration_nested_properties,
    scss_dimension_no_non_numeric_values, scss_dollar_variable_empty_line_before,
    scss_dollar_variable_no_missing_interpolation, scss_dollar_variable_no_namespaced_assignment,
    scss_double_slash_comment_empty_line_before, scss_double_slash_comment_whitespace_inside,
    scss_function_calculation_no_interpolation, scss_function_disallowed_list,
    scss_map_keys_quotes, scss_no_duplicate_dollar_variables, scss_no_duplicate_load_rules,
    scss_no_duplicate_mixins, scss_no_global_function_names, scss_no_unused_private_members,
    scss_property_no_unknown, scss_selector_class_pattern, selector_anb_no_unmatchable,
    selector_id_pattern, selector_max_id, selector_no_deprecated, selector_no_vendor_prefix,
    selector_not_notation, selector_pseudo_class_no_unknown,
    selector_pseudo_element_colon_notation, selector_pseudo_element_no_unknown, selector_type_case,
    selector_type_no_unknown, shorthand_property_no_redundant_values, unit_no_unknown,
    value_keyword_case, value_no_vendor_prefix,
};

/// The tokens' files, where colours are written (the design tokens of EN-169).
pub const TOKEN_FILES: &[&str] = &["**/tokens.scss", "**/_tokens.scss", "**/tokens/**"];

/// The sheets written before the tokens, whose colours move into them when the zones are restyled
/// after Figma (F56): the page's global style, plain CSS (EN-160, EN-162).
pub const BEFORE_TOKENS: &[&str] = &["playground/src/app/shared/styles/page.scss"];

/// The rules refusing a colour written outside the tokens.
pub const COLOUR_RULES: &[&str] = &[
    "stylelint/color-named",
    "stylelint/color-no-hex",
    "stylelint/function-disallowed-list",
    "scss/function-disallowed-list",
];

/// The rules the sheets written before the tokens are spared until their restyling (US-092): the
/// colour rules, and `selector-max-id`, as the page's global style selects its zones by id.
pub const BEFORE_TOKENS_RULES: &[&str] = &[
    "stylelint/color-named",
    "stylelint/color-no-hex",
    "stylelint/function-disallowed-list",
    "scss/function-disallowed-list",
    "stylelint/selector-max-id",
];

/// Whether a rule runs on a sheet.
#[must_use]
pub fn applies(rule: &str, path: &str) -> bool {
    !((COLOUR_RULES.contains(&rule) && crate::glob::matches_any(TOKEN_FILES, path))
        || (BEFORE_TOKENS_RULES.contains(&rule) && BEFORE_TOKENS.contains(&path)))
}

/// The rules, built once.
pub static RULES: LazyLock<Vec<Box<dyn SheetRule>>> = LazyLock::new(|| {
    let mut rules = core_rules();
    rules.extend(scss_rules());
    rules.extend(order_rules());
    rules
});

/// Stylelint's rules.
fn core_rules() -> Vec<Box<dyn SheetRule>> {
    vec![
        Box::new(at_rule_disallowed_list::AtRuleDisallowedList::project()),
        Box::new(at_rule_empty_line_before::AtRuleEmptyLineBefore::project()),
        Box::new(at_rule_no_deprecated::AtRuleNoDeprecated::PROJECT),
        Box::new(at_rule_no_vendor_prefix::AtRuleNoVendorPrefix::PROJECT),
        Box::new(block_no_empty::BlockNoEmpty::PROJECT),
        Box::new(block_no_redundant_nested_style_rules::BlockNoRedundantNestedStyleRules),
        Box::new(color_named::ColorNamed::PROJECT),
        Box::new(color_no_hex::ColorNoHex::PROJECT),
        Box::new(color_no_invalid_hex::ColorNoInvalidHex),
        Box::new(comment_empty_line_before::CommentEmptyLineBefore::project()),
        Box::new(comment_whitespace_inside::CommentWhitespaceInside::PROJECT),
        Box::new(container_name_pattern::ContainerNamePattern::project()),
        Box::new(custom_property_empty_line_before::CustomPropertyEmptyLineBefore::project()),
        Box::new(custom_property_no_missing_var_function::CustomPropertyNoMissingVarFunction),
        Box::new(custom_property_pattern::CustomPropertyPattern::project()),
        Box::new(declaration_block_no_duplicate_custom_properties::DeclarationBlockNoDuplicateCustomProperties::PROJECT),
        Box::new(declaration_block_no_duplicate_properties::DeclarationBlockNoDuplicateProperties::PROJECT),
        Box::new(declaration_block_no_redundant_longhand_properties::DeclarationBlockNoRedundantLonghandProperties::PROJECT),
        Box::new(declaration_block_no_shorthand_property_overrides::DeclarationBlockNoShorthandPropertyOverrides),
        Box::new(declaration_no_important::DeclarationNoImportant),
        Box::new(declaration_property_value_keyword_no_deprecated::DeclarationPropertyValueKeywordNoDeprecated::PROJECT),
        Box::new(font_family_name_quotes::FontFamilyNameQuotes::PROJECT),
        Box::new(font_family_no_duplicate_names::FontFamilyNoDuplicateNames::PROJECT),
        Box::new(font_family_no_missing_generic_family_keyword::FontFamilyNoMissingGenericFamilyKeyword::PROJECT),
        Box::new(function_disallowed_list::FunctionDisallowedList::project()),
        Box::new(function_linear_gradient_no_nonstandard_direction::FunctionLinearGradientNoNonstandardDirection),
        Box::new(function_name_case::FunctionNameCase::PROJECT),
        Box::new(function_url_no_scheme_relative::FunctionUrlNoSchemeRelative),
        Box::new(function_url_quotes::FunctionUrlQuotes::PROJECT),
        Box::new(keyframe_block_no_duplicate_selectors::KeyframeBlockNoDuplicateSelectors),
        Box::new(keyframe_selector_notation::KeyframeSelectorNotation::PROJECT),
        Box::new(keyframes_name_pattern::KeyframesNamePattern::project()),
        Box::new(layer_name_pattern::LayerNamePattern::project()),
        Box::new(length_zero_no_unit::LengthZeroNoUnit::project()),
        Box::new(max_nesting_depth::MaxNestingDepth::project()),
        Box::new(media_feature_name_no_unknown::MediaFeatureNameNoUnknown::PROJECT),
        Box::new(media_feature_name_no_vendor_prefix::MediaFeatureNameNoVendorPrefix::PROJECT),
        Box::new(media_type_no_deprecated::MediaTypeNoDeprecated::PROJECT),
        Box::new(named_grid_areas_no_invalid::NamedGridAreasNoInvalid),
        Box::new(nesting_selector_no_missing_scoping_root::NestingSelectorNoMissingScopingRoot::project()),
        Box::new(no_duplicate_selectors::NoDuplicateSelectors::PROJECT),
        Box::new(no_empty_source::NoEmptySource),
        Box::new(no_irregular_whitespace::NoIrregularWhitespace),
        Box::new(number_max_precision::NumberMaxPrecision::PROJECT),
        Box::new(property_no_deprecated::PropertyNoDeprecated::PROJECT),
        Box::new(property_no_vendor_prefix::PropertyNoVendorPrefix::PROJECT),
        Box::new(rule_empty_line_before::RuleEmptyLineBefore::project()),
        Box::new(selector_anb_no_unmatchable::SelectorAnbNoUnmatchable),
        Box::new(selector_id_pattern::SelectorIdPattern::project()),
        Box::new(selector_max_id::SelectorMaxId::project()),
        Box::new(selector_no_deprecated::SelectorNoDeprecated::PROJECT),
        Box::new(selector_no_vendor_prefix::SelectorNoVendorPrefix::PROJECT),
        Box::new(selector_not_notation::SelectorNotNotation::PROJECT),
        Box::new(selector_pseudo_class_no_unknown::SelectorPseudoClassNoUnknown::PROJECT),
        Box::new(selector_pseudo_element_colon_notation::SelectorPseudoElementColonNotation::PROJECT),
        Box::new(selector_pseudo_element_no_unknown::SelectorPseudoElementNoUnknown::PROJECT),
        Box::new(selector_type_case::SelectorTypeCase::PROJECT),
        Box::new(selector_type_no_unknown::SelectorTypeNoUnknown::PROJECT),
        Box::new(shorthand_property_no_redundant_values::ShorthandPropertyNoRedundantValues::PROJECT),
        Box::new(unit_no_unknown::UnitNoUnknown::PROJECT),
        Box::new(value_keyword_case::ValueKeywordCase::PROJECT),
        Box::new(value_no_vendor_prefix::ValueNoVendorPrefix::project()),
    ]
}

/// stylelint-order's rules: the content of a block in its order, its properties in the Recess order.
fn order_rules() -> Vec<Box<dyn SheetRule>> {
    vec![
        Box::new(order::Order::project()),
        Box::new(properties_order::PropertiesOrder::project()),
    ]
}

/// stylelint-scss's rules.
fn scss_rules() -> Vec<Box<dyn SheetRule>> {
    vec![
        Box::new(scss_at_extend_no_missing_placeholder::AtExtendNoMissingPlaceholder),
        Box::new(NamePattern::project(Named::Function)),
        Box::new(scss_at_if_no_null::AtIfNoNull),
        Box::new(scss_at_mixin_argumentless_call_parentheses::AtMixinArgumentlessCallParentheses::PROJECT),
        Box::new(AmpersandRules { which: Ampersand::RiskyInMixin }),
        Box::new(NamePattern::project(Named::Mixin)),
        Box::new(scss_at_root_no_redundant::AtRootNoRedundant),
        Box::new(scss_at_rule_conditional_no_parentheses::AtRuleConditionalNoParentheses),
        Box::new(scss_at_rule_no_unknown::AtRuleNoUnknown::PROJECT),
        Box::new(scss_at_use_no_redundant_alias::AtUseNoRedundantAlias),
        Box::new(scss_at_use_no_unnamespaced::AtUseNoUnnamespaced),
        Box::new(scss_block_no_redundant_nesting::BlockNoRedundantNesting),
        Box::new(scss_comment_no_empty::CommentNoEmpty),
        Box::new(scss_declaration_nested_properties::DeclarationNestedProperties::PROJECT),
        Box::new(scss_dimension_no_non_numeric_values::DimensionNoNonNumericValues),
        Box::new(scss_dollar_variable_empty_line_before::DollarVariableEmptyLineBefore::project()),
        Box::new(scss_dollar_variable_no_missing_interpolation::DollarVariableNoMissingInterpolation),
        Box::new(scss_dollar_variable_no_namespaced_assignment::DollarVariableNoNamespacedAssignment),
        Box::new(NamePattern::project(Named::Variable { ignore_global: false, ignore_local: false })),
        Box::new(scss_double_slash_comment_empty_line_before::DoubleSlashCommentEmptyLineBefore::project()),
        Box::new(scss_double_slash_comment_whitespace_inside::DoubleSlashCommentWhitespaceInside::PROJECT),
        Box::new(scss_function_calculation_no_interpolation::FunctionCalculationNoInterpolation),
        Box::new(scss_function_disallowed_list::FunctionDisallowedList::project()),
        Box::new(ColorFunctions { colour: Colour::Channel }),
        Box::new(ColorFunctions { colour: Colour::Relative }),
        Box::new(QuoteFunctions { quoting: Quoting::Quote }),
        Box::new(QuoteFunctions { quoting: Quoting::Unquote }),
        Box::new(LoadRules::NO_LEADING_UNDERSCORE),
        Box::new(LoadRules::EXTENSION),
        Box::new(scss_map_keys_quotes::MapKeysQuotes),
        Box::new(scss_no_duplicate_dollar_variables::NoDuplicateDollarVariables::PROJECT),
        Box::new(scss_no_duplicate_load_rules::NoDuplicateLoadRules),
        Box::new(scss_no_duplicate_mixins::NoDuplicateMixins),
        Box::new(scss_no_global_function_names::NoGlobalFunctionNames),
        Box::new(scss_no_unused_private_members::NoUnusedPrivateMembers),
        Box::new(NamePattern::project(Named::Placeholder)),
        Box::new(scss_property_no_unknown::PropertyNoUnknown::PROJECT),
        Box::new(scss_selector_class_pattern::SelectorClassPattern::project()),
        Box::new(AmpersandRules { which: Ampersand::Redundant(Vec::new()) }),
        Box::new(AmpersandRules { which: Ampersand::UnionClassName }),
    ]
}

#[cfg(test)]
mod tests {
    use super::{RULES, applies};
    use crate::scss::rules::tester::reports;

    #[test]
    fn f53_ac2_runs_each_kept_rule_once() {
        let mut names: Vec<&str> = RULES.iter().map(|rule| rule.name()).collect();
        let count = names.len();
        names.sort_unstable();
        names.dedup();

        // 62 of Stylelint, 40 of stylelint-scss, 2 of stylelint-order.
        assert_eq!((count, names.len()), (104, 104));
        assert_eq!(
            names
                .iter()
                .filter(|name| name.starts_with("scss/"))
                .count(),
            40
        );
    }

    #[test]
    fn f53_ac2_lets_the_tokens_files_alone_write_colours() {
        assert!(!applies(
            "stylelint/color-no-hex",
            "playground/src/styles/_tokens.scss"
        ));
        assert!(!applies(
            "stylelint/color-named",
            "playground/src/styles/tokens/colours.scss"
        ));
        assert!(applies(
            "stylelint/color-no-hex",
            "playground/src/app/app.component.scss"
        ));
        assert!(applies(
            "stylelint/declaration-no-important",
            "playground/src/styles/tokens.scss"
        ));
        assert!(!applies(
            "stylelint/color-no-hex",
            "playground/src/app/shared/styles/page.scss"
        ));
    }

    /// Own cases, one a rule whose message takes two or more arguments, under the project's
    /// configuration: (rule, code, message as the original's `ruleMessages` function writes it).
    /// The test files' `messages.<id>(…)` go through the same template as the reports, so they
    /// cannot tell an order. `color-named`'s and
    /// `declaration-block-no-redundant-longhand-properties`' two-argument messages are not
    /// reported under the project's options; `order`, `properties-order` and `selector-max-id`
    /// assert theirs in their own tests.
    const MESSAGES: &[(&str, &str, &str)] = &[
        (
            "stylelint/container-name-pattern",
            "@container Foo (width > 1px) {}",
            "Expected \"Foo\" to match pattern \"^(--)?([a-z][a-z0-9]*)(-[a-z0-9]+)*$\"",
        ),
        (
            "stylelint/custom-property-pattern",
            ":root { --Foo: 1px; }",
            "Expected \"--Foo\" to match pattern \"^([a-z][a-z0-9]*)(-[a-z0-9]+)*$\"",
        ),
        (
            "stylelint/declaration-block-no-shorthand-property-overrides",
            "a { padding-left: 1px; padding: 0; }",
            "Overridden property \"padding-left\" by shorthand \"padding\"",
        ),
        (
            "stylelint/declaration-property-value-keyword-no-deprecated",
            "a { word-break: break-word; }",
            "Deprecated keyword \"break-word\" for property \"word-break\"",
        ),
        (
            "stylelint/declaration-property-value-keyword-no-deprecated",
            "a { overflow: overlay; }",
            "Expected \"overlay\" to be \"auto\"",
        ),
        (
            "stylelint/function-name-case",
            "a { width: CALC(1px); }",
            "Expected \"CALC\" to be \"calc\"",
        ),
        (
            "stylelint/keyframe-selector-notation",
            "@keyframes a { from {} 100% {} }",
            "Expected \"from\" to be \"0%\"",
        ),
        (
            "stylelint/keyframes-name-pattern",
            "@keyframes Foo {}",
            "Expected \"Foo\" to match pattern \"^([a-z][a-z0-9]*)(-[a-z0-9]+)*$\"",
        ),
        (
            "stylelint/layer-name-pattern",
            "@layer Foo;",
            "Expected \"Foo\" to match pattern \"^([a-z][a-z0-9]*)([.-][a-z0-9]+)*$\"",
        ),
        (
            "stylelint/no-duplicate-selectors",
            "a {}\nb {}\na {}",
            "Duplicate selector \"a\", first used at line 1",
        ),
        (
            "stylelint/number-max-precision",
            "a { width: 0.123456px; }",
            "Expected \"0.123456\" to be \"0.1235\"",
        ),
        (
            "stylelint/property-no-deprecated",
            "a { -khtml-box-flex: 1; }",
            "Expected \"-khtml-box-flex\" to be \"flex-grow\"",
        ),
        (
            "stylelint/selector-id-pattern",
            "#Foo {}",
            "Expected \"#Foo\" to match pattern \"^([a-z][a-z0-9]*)(-[a-z0-9]+)*$\"",
        ),
        (
            "stylelint/selector-no-deprecated",
            "a:focus-ring {}",
            "Expected \":focus-ring\" to be \":focus-visible\"",
        ),
        (
            "stylelint/selector-type-case",
            "A {}",
            "Expected \"A\" to be \"a\"",
        ),
        (
            "stylelint/shorthand-property-no-redundant-values",
            "a { margin: 1px 1px; }",
            "Expected \"1px 1px\" to be \"1px\"",
        ),
        (
            "stylelint/value-keyword-case",
            "a { display: BLOCK; }",
            "Expected \"BLOCK\" to be \"block\"",
        ),
        (
            "scss/at-function-pattern",
            "@function Foo() { @return 1; }",
            "Expected \"Foo\" to match pattern \"^(-?[a-z][a-z0-9]*)(-[a-z0-9]+)*$\"",
        ),
        (
            "scss/at-mixin-pattern",
            "@mixin Foo {}",
            "Expected \"Foo\" to match pattern \"^(-?[a-z][a-z0-9]*)(-[a-z0-9]+)*$\"",
        ),
        (
            "scss/dimension-no-non-numeric-values",
            "a { width: #{$value}px; }",
            "Expected \"$value * 1px\" instead of \"#{$value}px\". Consider writing \"value\" in terms of px originally.",
        ),
        (
            "scss/dollar-variable-no-missing-interpolation",
            "$var: \"my-anim\";\n@keyframes $var {}",
            "Expected variable $var to be interpolated when using it with @keyframes",
        ),
        (
            "scss/dollar-variable-pattern",
            "$Foo: 1;",
            "Expected \"Foo\" to match pattern \"^(-?[a-z][a-z0-9]*)(-[a-z0-9]+)*$\"",
        ),
        (
            "scss/load-partial-extension",
            "@use \"a.scss\";",
            "Unexpected extension \".scss\" in @use",
        ),
        (
            "scss/percent-placeholder-pattern",
            "%Foo {}",
            "Expected \"Foo\" to match pattern \"^(-?[a-z][a-z0-9]*)(-[a-z0-9]+)*$\"",
        ),
        (
            "scss/selector-class-pattern",
            ".Foo {}",
            "Expected \".Foo\" to match pattern \"^([a-z][a-z0-9]*)(-[a-z0-9]+)*$\"",
        ),
    ];

    #[test]
    fn prints_the_arguments_of_each_message_where_the_original_does() {
        let wrong: Vec<(&str, Vec<String>)> = MESSAGES
            .iter()
            .filter_map(|&(name, code, expected)| {
                let rule = RULES.iter().find(|rule| rule.name() == name);
                let found: Vec<String> = rule
                    .map(|rule| reports(rule.as_ref(), code))
                    .unwrap_or_default()
                    .into_iter()
                    .map(|(_, _, message)| message)
                    .collect();
                (!found.iter().any(|each| each == expected)).then_some((name, found))
            })
            .collect();
        assert_eq!(wrong, []);
    }

    #[test]
    fn refuses_ids_everywhere_but_in_the_page_s_global_style_until_us_092() {
        assert!(applies(
            "stylelint/selector-max-id",
            "playground/src/app/app.component.scss"
        ));
        assert!(applies(
            "stylelint/selector-max-id",
            "playground/src/styles/_tokens.scss"
        ));
        assert!(!applies(
            "stylelint/selector-max-id",
            "playground/src/app/shared/styles/page.scss"
        ));
        assert!(applies(
            "stylelint/declaration-no-important",
            "playground/src/app/shared/styles/page.scss"
        ));
    }
}

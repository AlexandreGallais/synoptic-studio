//! The rules of `stylelint` 17.16.0 and `stylelint-scss` 7.3.0 the catalogue keeps (EN-167,
//! REF-STYLELINT-SOURCE), each a port of its rule reading the sheet's tree as the original walks
//! `PostCSS`'s, its messages the original's, so that findings compare one for one; their options
//! are the project's, after stylelint-config-standard-scss where it sets them
//! (REF-STYLELINT-CONFIGS). A rule runs only on a sheet that reads cleanly, as Stylelint reports a
//! `CssSyntaxError` and nothing else. Each rule is tested on every case of its test file the reader
//! can run (`tester`), counted.

pub mod at_rule_disallowed_list;
pub mod at_rule_empty_line_before;
pub mod at_rule_no_deprecated;
pub mod at_rule_no_vendor_prefix;
pub mod block_no_empty;
pub mod block_no_redundant_nested_style_rules;
pub mod color_named;
pub mod color_no_hex;
pub mod color_no_invalid_hex;
pub mod comment_empty_line_before;
pub mod comment_whitespace_inside;
pub mod container_name_pattern;
pub mod custom_property_empty_line_before;
pub mod custom_property_no_missing_var_function;
pub mod custom_property_pattern;
pub mod declaration_block_no_duplicate_custom_properties;
pub mod declaration_block_no_duplicate_properties;
pub mod declaration_block_no_redundant_longhand_properties;
pub mod declaration_block_no_shorthand_property_overrides;
pub mod declaration_no_important;
pub mod declaration_property_value_keyword_no_deprecated;
pub mod empty_lines;
pub mod font_families;
pub mod font_family_name_quotes;
pub mod font_family_no_duplicate_names;
pub mod font_family_no_missing_generic_family_keyword;
pub mod function_disallowed_list;
pub mod function_linear_gradient_no_nonstandard_direction;
pub mod function_name_case;
pub mod function_url_no_scheme_relative;
pub mod function_url_quotes;
pub mod keyframe_block_no_duplicate_selectors;
pub mod keyframe_selector_notation;
pub mod keyframes_name_pattern;
pub mod layer_name_pattern;
pub mod length_zero_no_unit;
pub mod max_nesting_depth;
pub mod media;
pub mod media_feature_name_no_unknown;
pub mod media_feature_name_no_vendor_prefix;
pub mod media_type_no_deprecated;
pub mod named_grid_areas_no_invalid;
pub mod nested;
pub mod nesting_selector_no_missing_scoping_root;
pub mod no_duplicate_selectors;
pub mod no_empty_source;
pub mod no_irregular_whitespace;
pub mod number_max_precision;
pub mod numbers;
pub mod order;
pub mod ordering;
pub mod prefixed;
pub mod project;
pub mod properties_order;
pub mod property_no_deprecated;
pub mod property_no_vendor_prefix;
pub mod recess_order;
pub mod reference;
pub mod rule_empty_line_before;
pub mod scss_ampersand;
pub mod scss_at_extend_no_missing_placeholder;
pub mod scss_at_if_no_null;
pub mod scss_at_mixin_argumentless_call_parentheses;
pub mod scss_at_root_no_redundant;
pub mod scss_at_rule_conditional_no_parentheses;
pub mod scss_at_rule_no_unknown;
pub mod scss_at_use_no_redundant_alias;
pub mod scss_at_use_no_unnamespaced;
pub mod scss_block_no_redundant_nesting;
pub mod scss_color_functions;
pub mod scss_comment_no_empty;
pub mod scss_declaration_nested_properties;
pub mod scss_dimension_no_non_numeric_values;
pub mod scss_dollar_variable_empty_line_before;
pub mod scss_dollar_variable_no_missing_interpolation;
pub mod scss_dollar_variable_no_namespaced_assignment;
pub mod scss_double_slash_comment_empty_line_before;
pub mod scss_double_slash_comment_whitespace_inside;
pub mod scss_function_calculation_no_interpolation;
pub mod scss_load_rules;
pub mod scss_map_keys_quotes;
pub mod scss_no_duplicate_dollar_variables;
pub mod scss_no_duplicate_load_rules;
pub mod scss_no_duplicate_mixins;
pub mod scss_no_global_function_names;
pub mod scss_no_unused_private_members;
pub mod scss_patterns;
pub mod scss_property_no_unknown;
pub mod scss_quote_functions;
pub mod scss_selector_class_pattern;
pub mod scss_selectors;
pub mod selector_anb_no_unmatchable;
pub mod selector_id_pattern;
pub mod selector_no_deprecated;
pub mod selector_no_vendor_prefix;
pub mod selector_not_notation;
pub mod selector_pseudo_class_no_unknown;
pub mod selector_pseudo_element_colon_notation;
pub mod selector_pseudo_element_no_unknown;
pub mod selector_type_case;
pub mod selector_type_no_unknown;
pub mod selectors;
pub mod shorthand_property_no_redundant_values;
#[cfg(test)]
pub mod tester;
pub mod unit_no_unknown;
pub mod value_keyword_case;
pub mod value_no_vendor_prefix;

use regex::Regex;

use super::tree::{Kind, Node, Sheet};
use crate::finding::Fix;
use crate::lines::Lines;

/// A string an option matches exactly, or a regular expression it tests, as
/// `matchesStringOrRegExp` reads a string `/…/` or `/…/i` as one.
#[derive(Clone, Debug)]
pub enum Matcher {
    /// A string compared exactly.
    Exact(String),
    /// A regular expression.
    Pattern(Regex),
}

impl Matcher {
    /// A matcher from an option's string.
    #[must_use]
    pub fn new(text: &str) -> Self {
        let pattern = if text.len() > 2 && text.starts_with('/') && text.ends_with("/i") {
            Some(format!("(?i){}", &text[1..text.len() - 2]))
        } else if text.len() > 1 && text.starts_with('/') && text.ends_with('/') {
            Some(text[1..text.len() - 1].to_owned())
        } else {
            None
        };
        pattern
            .and_then(|pattern| Regex::new(&pattern).ok())
            .map_or_else(|| Self::Exact(text.to_owned()), Self::Pattern)
    }

    /// A matcher from a regular expression's pattern and flags, `None` when Rust's engine
    /// cannot read it.
    #[must_use]
    pub fn regex(pattern: &str, flags: &str) -> Option<Self> {
        let prefix = if flags.contains('i') { "(?i)" } else { "" };
        Regex::new(&format!("{prefix}{pattern}"))
            .ok()
            .map(Self::Pattern)
    }

    /// Whether a value matches.
    #[must_use]
    pub fn matches(&self, value: &str) -> bool {
        match self {
            Self::Exact(text) => text == value,
            Self::Pattern(regex) => regex.is_match(value),
        }
    }
}

/// A pattern a name must match: a regular expression, from a string as `new RegExp(primary)`
/// reads it, and the text a message shows of it.
#[derive(Clone, Debug)]
pub struct Pattern {
    /// The regular expression, none when Rust's engine cannot read it: it matches nothing.
    pub regex: Option<Regex>,
    /// How a message shows it: the string, or `/…/flags` for a regular expression.
    pub text: String,
}

impl Pattern {
    /// The pattern of a string.
    #[must_use]
    pub fn new(source: &str) -> Self {
        Self {
            regex: Regex::new(source).ok(),
            text: source.to_owned(),
        }
    }

    /// stylelint-config-standard's kebab-case, `^([a-z][a-z0-9]*)(-[a-z0-9]+)*$`.
    #[must_use]
    pub fn kebab() -> Self {
        Self::new("^([a-z][a-z0-9]*)(-[a-z0-9]+)*$")
    }

    /// Whether a name matches.
    #[must_use]
    pub fn matches(&self, name: &str) -> bool {
        self.regex
            .as_ref()
            .is_some_and(|regex| regex.is_match(name))
    }
}

/// Whether a value matches one of the matchers (`optionsMatches`).
#[must_use]
pub fn matches_any(matchers: &[Matcher], value: &str) -> bool {
    matchers.iter().any(|matcher| matcher.matches(value))
}

/// A rule of the style sheets.
pub trait SheetRule: Sync + Send {
    /// Its name: Stylelint's prefixed by `stylelint/`, stylelint-scss's own, `scss/…`.
    fn name(&self) -> &'static str;

    /// Its messages by id, as the original writes them, each `{}` filled in order from a
    /// report's arguments.
    fn messages(&self) -> &'static [(&'static str, &'static str)];

    /// A message the original computes rather than fills, for a rule whose message is not a
    /// template; `None` for the others.
    fn computed(&self, _key: &str, _args: &[String]) -> Option<String> {
        None
    }

    /// Checks a sheet.
    fn check(&self, sheet: &Context, reports: &mut Vec<Report>);
}

/// What a rule reports: where, which message with which arguments, and its fix.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    /// The first byte reported.
    pub start: usize,
    /// The byte after the last.
    pub end: usize,
    /// The message's id.
    pub key: &'static str,
    /// The values of the message's `{}`s, in order.
    pub args: Vec<String>,
    /// The fix, for a rule the original fixes.
    pub fix: Option<Fix>,
    /// The node the original's fix rewrites, when it is not the fix's own range: its edit
    /// information is recorded once a node.
    pub scope: Option<(usize, usize)>,
}

impl Report {
    /// A report over a range, without arguments nor fix.
    #[must_use]
    /// An end at or before the start is one past it, as `rangeBy` gives it.
    pub const fn new(start: usize, end: usize, key: &'static str) -> Self {
        Self {
            start,
            end: if end > start { end } else { start + 1 },
            key,
            args: Vec::new(),
            fix: None,
            scope: None,
        }
    }

    /// The report with the node its fix rewrites.
    #[must_use]
    pub const fn scoped(mut self, start: usize, end: usize) -> Self {
        self.scope = Some((start, end));
        self
    }

    /// The report with its message's arguments.
    #[must_use]
    pub fn with(mut self, args: &[&str]) -> Self {
        self.args = args.iter().map(|&each| each.to_owned()).collect();
        self
    }

    /// The report with a fix replacing a range of the sheet.
    #[must_use]
    pub fn fixed(mut self, start: usize, end: usize, text: impl Into<String>) -> Self {
        self.fix = Some(Fix {
            start: u32::try_from(start).unwrap_or(u32::MAX),
            end: u32::try_from(end).unwrap_or(u32::MAX),
            text: text.into(),
        });
        self
    }
}

/// A report's message: the rule's template for its id, each `{}` filled in order.
#[must_use]
pub fn message(rule: &dyn SheetRule, key: &str, args: &[String]) -> String {
    if let Some(text) = rule.computed(key, args) {
        return text;
    }
    let template = rule
        .messages()
        .iter()
        .find(|(id, _)| *id == key)
        .map_or(key, |(_, text)| text);
    let mut result = String::with_capacity(template.len());
    let mut rest = template;
    let mut values = args.iter();
    // `{}` takes the next value; `{=word}` too, `word` when none is left (a default parameter).
    while let Some(at) = rest.find("{}").into_iter().chain(rest.find("{=")).min() {
        result.push_str(&rest[..at]);
        let (fallback, length) = if rest[at..].starts_with("{}") {
            ("", 2)
        } else {
            let close = rest[at..].find('}').unwrap_or(rest.len() - at);
            (&rest[at + 2..at + close], close + 1)
        };
        result.push_str(values.next().map_or(fallback, String::as_str));
        rest = &rest[at + length..];
    }
    result.push_str(rest);
    result
}

/// A sheet read cleanly: its file, its text, its lines and its tree.
pub struct Context<'s> {
    /// The file, from the root.
    pub path: &'s str,
    /// The text.
    pub text: &'s str,
    /// Where its lines start.
    pub lines: Lines,
    /// The tree.
    pub sheet: &'s Sheet,
}

impl Context<'_> {
    /// A node by its index.
    #[must_use]
    pub fn node(&self, index: usize) -> &Node {
        self.sheet.node(index)
    }

    /// Every node of a kind, with its index, in the order written.
    #[must_use]
    pub fn all(&self, kind: Kind) -> Vec<(usize, &Node)> {
        self.sheet
            .walk()
            .into_iter()
            .map(|index| (index, self.sheet.node(index)))
            .filter(|(_, node)| node.is(kind))
            .collect()
    }

    /// The text of a node, from its start to its end.
    #[must_use]
    pub fn source(&self, node: &Node) -> &str {
        self.text
            .get(node.start..node.end.unwrap_or(self.text.len()))
            .unwrap_or_default()
    }

    /// A declaration's value as written, comments included (`getDeclarationValue`).
    #[must_use]
    pub fn value<'n>(&self, node: &'n Node) -> &'n str {
        node.raw.as_deref().unwrap_or(&node.value)
    }

    /// The byte where a declaration's value starts (`declarationValueIndex`): the property and
    /// the text after it counted from the declaration's start, a hack's `_` or `*` left out as
    /// Stylelint counts it.
    #[must_use]
    pub fn value_start(&self, node: &Node) -> usize {
        node.start + node.prop.len() + node.between.len()
    }

    /// An at-rule's parameters as written, comments included (`getAtRuleParams`).
    #[must_use]
    pub fn params<'n>(&self, node: &'n Node) -> &'n str {
        node.raw.as_deref().unwrap_or(&node.params)
    }

    /// The byte where an at-rule's parameters start (`atRuleParamIndex`).
    #[must_use]
    pub fn params_start(&self, node: &Node) -> usize {
        node.start + 1 + node.name.len() + node.after_name.len()
    }

    /// A rule's selector as written, comments included (`getRuleSelector`).
    #[must_use]
    pub fn selector<'n>(&self, node: &'n Node) -> &'n str {
        node.raw.as_deref().unwrap_or(&node.selector)
    }

    /// The length of a rule's or an at-rule's text before its `{` (`beforeBlockString` without
    /// the spaces before the node).
    #[must_use]
    pub fn prelude_length(&self, node: &Node) -> usize {
        if node.is(Kind::Rule) {
            self.selector(node).len() + node.between.len()
        } else {
            1 + node.name.len()
                + node.after_name.len()
                + self.params(node).len()
                + node.between.len()
        }
    }

    /// Where a word first appears in a node's text, else the whole node (`rangeBy({ word })`).
    #[must_use]
    pub fn word_range(&self, node: &Node, word: &str) -> (usize, usize) {
        let end = node.end.unwrap_or(self.text.len());
        self.source(node)
            .find(word)
            .map_or((node.start, end), |at| {
                (node.start + at, node.start + at + word.len())
            })
    }

    /// The declarations of each block, a block's after those of the blocks nested in it, as
    /// `eachDeclarationBlock` gives them: the root's, the rules' and the at-rules'.
    #[must_use]
    pub fn declaration_blocks(&self) -> Vec<Vec<usize>> {
        let mut blocks = Vec::new();
        let mut stack: Vec<(usize, bool)> = vec![(0, false)];
        while let Some((index, visited)) = stack.pop() {
            let node = self.node(index);
            let container = node.is(Kind::Root) || node.is(Kind::Rule) || node.is(Kind::AtRule);
            if !container || node.nodes().is_empty() {
                continue;
            }
            if visited {
                let declarations: Vec<usize> = node
                    .nodes()
                    .iter()
                    .copied()
                    .filter(|&child| self.node(child).is(Kind::Declaration))
                    .collect();
                if !declarations.is_empty() {
                    blocks.push(declarations);
                }
            } else {
                stack.push((index, true));
                stack.extend(node.nodes().iter().rev().map(|&child| (child, false)));
            }
        }
        blocks
    }

    /// The range a node's removal takes out of the text: its spaces before and itself, as
    /// `node.remove()` leaves the text printed.
    #[must_use]
    pub fn removal(&self, node: &Node) -> (usize, usize) {
        (
            node.start.saturating_sub(node.before.len()),
            node.end.unwrap_or(self.text.len()),
        )
    }

    /// The ancestors of a node, its parent first.
    #[must_use]
    pub fn ancestors(&self, index: usize) -> Vec<&Node> {
        self.sheet
            .ancestors(index)
            .into_iter()
            .map(|at| self.sheet.node(at))
            .collect()
    }
}

/// The arguments of each function of a text whose name passes a test, as
/// `functionArgumentsSearch` gives them: the text between its parentheses, where it starts in
/// the text, and the function's name; for `url()`, the text read again as a block so that an
/// interpolation inside it is kept whole.
#[must_use]
pub fn function_arguments(
    source: &str,
    accepts: &dyn Fn(&str) -> bool,
) -> Vec<(String, usize, String)> {
    let parsed = super::value::parse(source);
    let mut found = Vec::new();
    parsed.walk(&mut |_, part| {
        if part.kind != super::value::Kind::Function || !accepts(&part.value) {
            return true;
        }
        let start = part.start + part.value.len() + 1;
        let end = if part.value.eq_ignore_ascii_case("url") {
            let rest = source
                .get(part.start + part.value.len()..)
                .unwrap_or_default();
            let block = super::value::parse(rest);
            let block_end = block
                .nodes
                .first()
                .map_or(0, |&first| block.part(first).end);
            let closing = source.as_bytes().get((start + block_end).wrapping_sub(2)) == Some(&b')');
            (start + block_end).saturating_sub(1 + usize::from(closing))
        } else {
            part.end - usize::from(source.as_bytes().get(part.end.wrapping_sub(1)) == Some(&b')'))
        };
        let text = source
            .get(start..end.max(start))
            .unwrap_or_default()
            .to_owned();
        found.push((text, start, part.value.clone()));
        true
    });
    found
}

/// Whether a text holds a Sass interpolation, `#{…}` (`hasScssInterpolation`).
#[must_use]
pub fn has_interpolation(text: &str) -> bool {
    text.find("#{")
        .is_some_and(|at| text[at + 2..].find('}').is_some_and(|close| close > 0))
}

/// Whether a text holds an interpolation of any syntax stylelint-scss knows (its
/// `hasInterpolation`): Sass's `#{…}`, Less's `@{…}`, a template's `{…}` on one line or a
/// `$(…)`.
#[must_use]
pub fn has_any_interpolation(text: &str) -> bool {
    let after = |opener: &str, closer: char| {
        text.find(opener)
            .is_some_and(|at| text[at + opener.len()..].contains(closer))
    };
    let template = text.match_indices('{').any(|(at, _)| {
        let line = text[at + 1..]
            .split(['\n', '\r', '\u{2028}', '\u{2029}'])
            .next()
            .unwrap_or_default();
        line.char_indices()
            .any(|(index, character)| index > 0 && character == '}')
    });
    after("#{", '}') || after("@{", '}') || after("$(", ')') || template
}

/// Whether a property is plain CSS: not a Sass variable, not interpolated
/// (`isStandardSyntaxProperty`).
#[must_use]
pub fn is_standard_property(property: &str) -> bool {
    !property.starts_with('$')
        && !property.contains(".$")
        && !property.starts_with('@')
        && !property.ends_with('+')
        && !property.ends_with("+_")
        && !has_interpolation(property)
}

/// Whether a declaration is plain CSS: not a Sass variable, not inside nested properties
/// (`isStandardSyntaxDeclaration`).
#[must_use]
pub fn is_standard_declaration(context: &Context, index: usize) -> bool {
    let node = context.node(index);
    if node.prop.starts_with('$') || node.prop.contains(".$") {
        return false;
    }
    if node.prop.starts_with('@') && !node.prop.starts_with("@{") {
        return false;
    }
    let parent = node.parent.map(|parent| context.node(parent));
    !parent.is_some_and(|parent| {
        parent.is(Kind::Rule)
            && parent.selector.ends_with(':')
            && !parent.selector.starts_with("--")
    })
}

/// Whether a selector is plain CSS: no interpolation, placeholder, nested properties nor `//`
/// comment (`isStandardSyntaxSelector`, its Less and template cases kept).
#[must_use]
pub fn is_standard_selector(selector: &str) -> bool {
    !(has_interpolation(selector)
        || selector.starts_with('%')
        || selector.ends_with(':')
        || selector.contains(":extend")
        || is_less_mixin_call(selector)
        || (selector.ends_with(')') && !selector.contains(':'))
        || is_less_parametric(selector)
        || selector.contains("<%")
        || selector.contains("%>")
        || selector.contains("//"))
}

/// `\.[\w-]+\(.*\).+`: a Less mixin call with selectors after it.
#[must_use]
pub fn is_standard_selector_less_call(selector: &str) -> bool {
    is_less_mixin_call(selector)
}

/// `\.[\w-]+\(.*\).+`: a Less mixin call with selectors after it.
fn is_less_mixin_call(selector: &str) -> bool {
    selector.match_indices('.').any(|(at, _)| {
        let rest = &selector[at + 1..];
        let name = rest
            .bytes()
            .take_while(|byte| byte.is_ascii_alphanumeric() || *byte == b'_' || *byte == b'-')
            .count();
        name > 0
            && rest[name..].starts_with('(')
            && rest[name..]
                .rfind(')')
                .is_some_and(|close| close + 1 < rest[name..].len())
    })
}

/// `\(@.*\)$`: a Less parametric mixin.
fn is_less_parametric(selector: &str) -> bool {
    selector.ends_with(')') && selector.contains("(@")
}

/// Whether a rule's selector is plain CSS (`isStandardSyntaxRule`).
#[must_use]
pub fn is_standard_rule(node: &Node) -> bool {
    node.is(Kind::Rule) && is_standard_selector(&node.selector)
}

/// Whether a value is plain CSS: no Sass variable, namespace nor interpolation at its start
/// (`isStandardSyntaxValue`).
#[must_use]
pub fn is_standard_value(value: &str) -> bool {
    let normalized = if value.starts_with(['-', '+', '*', '/']) {
        &value[1..]
    } else {
        value
    };
    if normalized.starts_with('$') || normalized.starts_with('@') || has_interpolation(normalized) {
        return false;
    }
    if let Some(dot) = value.find(".$")
        && dot > 0
    {
        return false;
    }
    let namespaced_call = value.char_indices().any(|(at, character)| {
        character == '.'
            && at > 0
            && value[at + 1..].find('(').is_some_and(|open| {
                open > 0
                    && value[at + 1..at + 1 + open]
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
            })
    });
    if namespaced_call {
        return false;
    }
    !(value.contains("__MSG_") && value[value.find("__MSG_").unwrap_or(0) + 6..].contains("__"))
}

/// Whether an at-rule is plain CSS: not `@charset`, not a bodiless `@content`
/// (`isStandardSyntaxAtRule`).
#[must_use]
pub fn is_standard_at_rule(node: &Node) -> bool {
    !(node.name.eq_ignore_ascii_case("charset")
        || (node.children.is_none() && node.params.is_empty())
        || (node.children.is_none() && node.after_name.is_empty() && node.params.starts_with('(')))
}

#[cfg(test)]
mod tests {
    use super::{
        Report, SheetRule, has_interpolation, is_standard_property, is_standard_selector,
        is_standard_value, message,
    };

    /// A rule with one message of two arguments.
    struct Probe;

    impl SheetRule for Probe {
        fn name(&self) -> &'static str {
            "stylelint/probe"
        }

        fn messages(&self) -> &'static [(&'static str, &'static str)] {
            &[("rejected", "Unexpected \"{}\" in \"{}\"")]
        }

        fn check(&self, _: &super::Context, _: &mut Vec<Report>) {}
    }

    #[test]
    fn fills_a_message_s_template_in_order() {
        assert_eq!(
            message(&Probe, "rejected", &["a".to_owned(), "b".to_owned()]),
            "Unexpected \"a\" in \"b\""
        );
        assert_eq!(message(&Probe, "other", &[]), "other");
        let report = Report::new(1, 2, "rejected").with(&["a"]).fixed(1, 2, "b");
        assert_eq!(
            (report.args.len(), report.fix.map(|fix| fix.text)),
            (1, Some("b".to_owned()))
        );
    }

    #[test]
    fn tells_plain_css_from_sass_syntax() {
        assert!(has_interpolation("a#{$b}") && !has_interpolation("a#{}"));
        assert!(is_standard_property("color") && !is_standard_property("$a"));
        assert!(!is_standard_property("ns.$a") && !is_standard_property("#{$a}-b"));
        assert!(is_standard_selector("a > b"));
        for selector in ["%a", "a:", ".a#{$b}", "a // b", ".m(@a)", ".m()", ".m().b"] {
            assert!(!is_standard_selector(selector), "{selector}");
        }
        assert!(is_standard_value("1px") && is_standard_value("calc(1px + 2px)"));
        for value in ["$a", "-$a", "ns.$a", "math.div(1, 2)", "#{$a}", "__MSG_a__"] {
            assert!(!is_standard_value(value), "{value}");
        }
    }
}

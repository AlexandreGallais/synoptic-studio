//! `jsdoc/check-param-names` with the project's options (destructured parameters checked, extra
//! properties reported, fixer on): the `@param` tags of a function's block name its parameters,
//! in order, once each, and the properties of a destructured parameter
//! (`eslint-plugin-jsdoc` 65.3.1, `src/rules/checkParamNames.js`). The order and a duplicate
//! are autofixed.

use oxc_ast::AstKind;
use oxc_ast::ast::{TSType, TSTypeAnnotation};
use oxc_semantic::AstNode;

use crate::finding::Finding;
use crate::jsdoc::block::{Spec, Tokens};
use crate::jsdoc::edit::{Edit, has_content, without_closing};
use crate::jsdoc::params::{Info, Inner, RestName, parameter_names};
use crate::jsdoc::{self, Doc, context};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct CheckParamNames;

/// The types of a `@param` whose properties are checked (`checkTypesPattern`).
const CHECKED_TYPES: [&str; 7] = [
    "object",
    "Object",
    "array",
    "Array",
    "PlainObject",
    "GenericObject",
    "GenericArray",
];

/// A path with `.'x'` and `."x"` read as `.x` (`dropPathSegmentQuotes`).
fn unquoted(path: &str) -> String {
    let mut result = String::new();
    let mut at = 0;
    while at < path.len() {
        let rest = &path[at..];
        if let Some(after) = rest.strip_prefix('.')
            && let Some(quote) = after
                .chars()
                .next()
                .filter(|each| *each == '\'' || *each == '"')
            && let Some(close) = after[1..].rfind(quote)
        {
            result.push('.');
            result.push_str(&after[1..=close]);
            at += 2 + close + 1;
            continue;
        }
        let character = rest.chars().next().unwrap_or(' ');
        result.push(character);
        at += character.len_utf8();
    }
    result
}

/// Whether two paths are the same, quotes aside (`comparePaths`).
fn same_path(one: &str, other: &str) -> bool {
    one == other || unquoted(one) == unquoted(other)
}

/// Whether a path does not start with another, quotes aside (`pathDoesNotBeginWith`).
fn does_not_begin_with(name: &str, other: &str) -> bool {
    !name.starts_with(other) && !unquoted(name).starts_with(&unquoted(other))
}

/// The simple name of a parameter: a name, or a rest element's (`getSimpleParameterName`).
fn simple_name(info: &Info) -> Option<String> {
    match info {
        Info::Name(name)
        | Info::Rest {
            name: RestName::Text(name),
            ..
        } => Some(name.clone()),
        _ => None,
    }
}

/// Whether a parameter is `this`.
fn is_this(info: &Info) -> bool {
    *info == Info::Name("this".to_owned())
}

/// What the check of one tag found.
enum Outcome {
    /// Nothing: go on with the next tag.
    Next,
    /// A fault: stop.
    Stop,
}

/// What a block's `@param` tags are checked against.
struct Check<'d, 'c> {
    /// The block.
    doc: &'d Doc<'c>,
    /// The function's parameters.
    parameters: Vec<Info>,
    /// The `@param` tags, with their index among the block's tags.
    tags: Vec<(usize, &'d Spec)>,
    /// The findings.
    findings: Vec<Finding>,
}

impl Check<'_, '_> {
    /// A finding on a line of the block.
    fn report(&mut self, source: &Source, line: usize, message: String, fix: Option<String>) {
        let mut finding =
            self.doc
                .finding_at(source, "jsdoc/check-param-names", line, None, message);
        finding.fix = fix.map(|text| self.doc.replace(text));
        self.findings.push(finding);
    }

    /// The block's text after a duplicate tag is removed.
    fn without_tag(&self, source: &Source, index: usize) -> String {
        let mut edit = Edit::new(
            &self.doc.block,
            &self.doc.indent,
            &jsdoc::new_line_end(source),
        );
        edit.remove_tag(index, 0);
        edit.text()
    }

    /// The block's text with its `@param` tags in the parameters' order (`makeParamOrderFix`).
    fn reordered(&self, source: &Source, expected: &[String]) -> String {
        let block = &self.doc.block;
        let mut ordered: Vec<(String, Vec<Tokens>)> = block
            .tags
            .iter()
            .map(|tag| {
                (
                    tag.tag.clone(),
                    tag.lines
                        .iter()
                        .map(|index| block.source[*index].tokens.clone())
                        .collect(),
                )
            })
            .collect();
        let non_nested: Vec<&(usize, &Spec)> = self
            .tags
            .iter()
            .filter(|(_, tag)| !tag.name.contains('.'))
            .collect();
        for (position, name) in expected.iter().enumerate() {
            if let Some((_, matching)) = non_nested.iter().find(|(_, tag)| tag.name.trim() == name)
            {
                ordered[non_nested[position].0] = (
                    matching.tag.clone(),
                    matching
                        .lines
                        .iter()
                        .map(|index| block.source[*index].tokens.clone())
                        .collect(),
                );
            }
        }
        let first = non_nested
            .iter()
            .zip(expected)
            .find(|((_, tag), name)| tag.name.trim() != name.as_str())
            .map_or(0, |((index, _), _)| *index);

        let line_end = jsdoc::new_line_end(source);
        let mut edit = Edit::new(block, &self.doc.indent, &line_end);
        edit.move_closing_to_own_line();
        let count = edit.tag_count() - first;
        let unchanged: usize = (0..first).map(|tag| edit.tag_length(tag) - 1).sum();
        let offset = edit.first_tag_line() + first + unchanged;
        for index in 0..count {
            edit.remove_tag(index + first, 0);
        }
        let mut extra = 0;
        for index in 0..count {
            let (_, lines) = &ordered[first + index];
            edit.add_tag(
                extra + offset + index,
                without_closing(&lines[0], &line_end),
            );
            for tokens in &lines[1..] {
                if tokens.end.is_empty() || has_content(tokens) {
                    edit.add_line(
                        extra + offset + index + 1,
                        without_closing(tokens, &line_end),
                    );
                    extra += 1;
                }
            }
        }
        edit.text()
    }

    /// The properties a destructured parameter's tags leave out, each with the line the plugin
    /// reports it on, plus one.
    fn missing_properties(
        &self,
        tag: &Spec,
        expected: &[String],
        actual: &[String],
        flat: &crate::jsdoc::params::Flat,
    ) -> Vec<(String, usize)> {
        let types: Vec<String> = self
            .tags
            .iter()
            .map(|(_, tag)| tag.r#type.clone())
            .collect();
        let mut missing = Vec::new();
        let mut skipped: Vec<String> = Vec::new();
        for (index, expected_name) in expected.iter().enumerate() {
            if skipped
                .iter()
                .any(|each| expected_name.starts_with(each.as_str()))
            {
                continue;
            }
            match actual
                .iter()
                .position(|each| same_path(expected_name, each))
            {
                None => {
                    if flat.rests.get(index).copied().unwrap_or(false) {
                        continue;
                    }
                    let found = actual
                        .iter()
                        .position(|each| does_not_begin_with(expected_name, each))
                        .unwrap_or(actual.len());
                    // The plugin's line, `number - 1 + found`, is -1 for a first tag on the
                    // block's first line: one line above the block.
                    let line = tag.lines[0] + found;
                    missing.push((expected_name.clone(), if line == 1 { 2 } else { line }));
                }
                Some(found) => {
                    if !CHECKED_TYPES.contains(&types[found].as_str()) && !types[found].is_empty() {
                        skipped.push(expected_name.clone());
                    }
                }
            }
        }
        missing
    }

    /// The check of a destructured or typed parameter's properties.
    fn check_properties(
        &mut self,
        source: &Source,
        tag: &Spec,
        parameter: Option<&str>,
        flat: &crate::jsdoc::params::Flat,
    ) -> Outcome {
        if !tag.r#type.is_empty() && !CHECKED_TYPES.contains(&tag.r#type.as_str()) {
            return Outcome::Next;
        }
        let name = tag.name.trim();
        if let Some(annotation) = &flat.annotation
            && name != annotation
        {
            self.report(
                source,
                tag.lines[0],
                format!("@param \"{name}\" does not match parameter name \"{annotation}\""),
                None,
            );
        }
        let root = parameter.unwrap_or(name);
        let expected: Vec<String> = flat
            .names
            .iter()
            .map(|each| format!("{root}.{each}"))
            .collect();
        let actual: Vec<String> = self
            .tags
            .iter()
            .map(|(_, tag)| tag.name.trim().to_owned())
            .collect();
        let missing = self.missing_properties(tag, &expected, &actual, flat);
        let has_missing = !missing.is_empty();
        for (missing_name, line) in missing {
            self.report(
                source,
                line,
                format!("Missing @param \"{missing_name}\""),
                None,
            );
            if let Some(finding) = self.findings.last_mut() {
                finding.line -= 1;
            }
        }
        if !flat.has_property_rest {
            let prefix = format!("{name}.");
            let extras: Vec<(String, usize)> = actual
                .iter()
                .enumerate()
                .filter(|(_, each)| {
                    each.starts_with(&prefix)
                        && !expected
                            .iter()
                            .any(|expected_name| same_path(each, expected_name))
                        && !same_path(each, &tag.name)
                })
                .map(|(index, each)| (each.clone(), self.tags[index].1.lines[0]))
                .collect();
            if !extras.is_empty() {
                for (extra, line) in extras {
                    self.report(
                        source,
                        line,
                        format!("@param \"{extra}\" does not exist on {}", tag.name),
                        None,
                    );
                }
                return Outcome::Stop;
            }
        }
        if has_missing {
            Outcome::Stop
        } else {
            Outcome::Next
        }
    }

    /// The check of every `@param` tag, up to the first fault (`validateParameterNames`).
    #[expect(
        clippy::too_many_lines,
        reason = "one port of the plugin's function, kept whole to read beside it"
    )]
    fn check_names(&mut self, source: &Source) -> bool {
        let tags = self.tags.clone();
        let non_nested: Vec<(usize, &Spec)> = tags
            .iter()
            .filter(|(_, tag)| !tag.name.contains('.'))
            .copied()
            .collect();
        let actual: Vec<String> = non_nested
            .iter()
            .map(|(_, tag)| tag.name.trim().to_owned())
            .collect();
        let documents_this = actual.iter().any(|each| each == "this");
        let simple: Vec<Option<String>> = self
            .parameters
            .iter()
            .filter(|each| documents_this || !is_this(each))
            .map(simple_name)
            .collect();
        let only_simple = simple.iter().all(Option::is_some);
        let expected: Vec<String> = simple.iter().flatten().cloned().collect();
        let mut sorted_actual = actual.clone();
        sorted_actual.sort();
        let mut sorted_expected = expected.clone();
        sorted_expected.sort();
        let same_names =
            only_simple && actual.len() == expected.len() && sorted_actual == sorted_expected;
        let bad_order = same_names
            && actual
                .iter()
                .zip(&expected)
                .any(|(one, other)| one != other);
        let mut unique = actual.clone();
        unique.sort();
        unique.dedup();
        let can_fix_order = tags.len() == non_nested.len() && unique.len() == actual.len();

        let mut dotted = 0;
        let mut this_offset: isize = 0;
        for (index, (_, tag)) in tags.iter().enumerate() {
            if let Some((duplicate, _)) = tags
                .iter()
                .enumerate()
                .find(|(other, (_, each))| each.name == tag.name && *other != index)
            {
                let (tag_index, duplicate_tag) = tags[duplicate];
                let fix = self.without_tag(source, tag_index);
                self.report(
                    source,
                    duplicate_tag.lines[0],
                    format!("Duplicate @param \"{}\"", tag.name),
                    Some(fix),
                );
                return true;
            }
            if tag.name.contains('.') {
                dotted += 1;
                continue;
            }
            let at = |offset: isize| {
                isize::try_from(index - dotted)
                    .ok()
                    .and_then(|base| usize::try_from(base + offset).ok())
                    .and_then(|position| self.parameters.get(position))
                    .cloned()
            };
            let mut parameter = at(this_offset);
            if !documents_this
                && parameter.as_ref().is_some_and(is_this)
                && tag.name.trim() != "this"
            {
                this_offset += 1;
                parameter = at(this_offset);
            }
            let Some(parameter) = parameter else {
                self.report(
                    source,
                    tag.lines[0],
                    format!(
                        "@param \"{}\" does not match an existing function parameter.",
                        tag.name
                    ),
                    None,
                );
                return true;
            };
            match &parameter {
                Info::Rest {
                    name: RestName::List(names),
                    ..
                } => {
                    let actual_name = tag.name.trim();
                    let expected_name = names
                        .get(index)
                        .cloned()
                        .flatten()
                        .unwrap_or_else(|| "undefined".to_owned());
                    if actual_name == expected_name {
                        this_offset -= 1;
                        continue;
                    }
                    self.report(
                        source,
                        tag.lines[0],
                        format!(
                            "Expected @param name to be \"{expected_name}\". Got \"{actual_name}\"."
                        ),
                        None,
                    );
                    return true;
                }
                Info::Pair(name, Inner::Flat(flat)) => {
                    match self.check_properties(source, tag, name.as_deref(), flat) {
                        Outcome::Next => continue,
                        Outcome::Stop => return true,
                    }
                }
                _ => {}
            }
            let parameter_name = simple_name(&parameter);
            if parameter_name.as_deref() == Some(tag.name.trim()) {
                continue;
            }
            let names: Vec<String> = self
                .parameters
                .iter()
                .enumerate()
                .map(|(position, item)| match item {
                    Info::Pair(_, Inner::Flat(_)) => {
                        actual.get(position).cloned().unwrap_or_default()
                    }
                    Info::Rest { name, .. } => format!("...{}", name.written()),
                    Info::Name(name) => name.clone(),
                    Info::Missing => String::new(),
                    _ => "[object Object]".to_owned(),
                })
                .filter(|name| documents_this || name != "this")
                .collect();
            let message = format!(
                "Expected @param names to be \"{}\". Got \"{}\".",
                names.join(", "),
                actual.join(", ")
            );
            if bad_order {
                let fix = can_fix_order.then(|| self.reordered(source, &expected));
                self.report(source, tag.lines[0], message, fix);
                return true;
            }
            self.report(source, tag.lines[0], message, None);
            return true;
        }
        false
    }

    /// The check of the paths of nested `@param` names (`validateParameterNamesDeep`).
    fn check_paths(&mut self, source: &Source) {
        let mut last_real: Option<String> = None;
        let tags = self.tags.clone();
        for (_, tag) in tags {
            let name = &tag.name;
            let Some(dot) = name.find('.') else {
                last_real = Some(name.clone());
                continue;
            };
            let Some(real) = &last_real else {
                self.report(
                    source,
                    tag.lines[0],
                    format!(
                        "@param path declaration (\"{name}\") appears before any real parameter."
                    ),
                    None,
                );
                return;
            };
            let root = name[..dot].strip_suffix("[]").unwrap_or(&name[..dot]);
            if root != real {
                self.report(
                    source,
                    tag.lines[0],
                    format!(
                        "@param path declaration (\"{name}\") root node name (\"{root}\") does not match previous real parameter name (\"{real}\")."
                    ),
                    None,
                );
                return;
            }
        }
    }
}

/// Whether a type annotation is a function type.
fn is_function_type(annotation: Option<&TSTypeAnnotation>) -> bool {
    annotation
        .is_some_and(|annotation| matches!(annotation.type_annotation, TSType::TSFunctionType(_)))
}

/// Whether a node is a function, a method signature or a property signature holding a function
/// type.
fn is_context(kind: &AstKind) -> bool {
    jsdoc::is_default_context(kind)
        || matches!(kind, AstKind::TSMethodSignature(_))
        || matches!(kind, AstKind::TSPropertySignature(property) if is_function_type(property.type_annotation.as_deref()))
}

/// The parameters of a node of the rule's contexts.
fn parameters(source: &Source, node: &AstNode) -> Vec<Info> {
    match node.kind() {
        AstKind::Function(function) => {
            parameter_names(source, function.this_param.as_deref(), &function.params)
        }
        AstKind::ArrowFunctionExpression(arrow) => parameter_names(source, None, &arrow.params),
        AstKind::TSMethodSignature(method) => {
            parameter_names(source, method.this_param.as_deref(), &method.params)
        }
        AstKind::TSPropertySignature(property) => match property
            .type_annotation
            .as_deref()
            .map(|annotation| &annotation.type_annotation)
        {
            Some(TSType::TSFunctionType(function)) => {
                parameter_names(source, function.this_param.as_deref(), &function.params)
            }
            _ => Vec::new(),
        },
        _ => Vec::new(),
    }
}

impl Rule for CheckParamNames {
    fn name(&self) -> &'static str {
        "jsdoc/check-param-names"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for (node, doc) in context::documented(source, is_context) {
            let tags: Vec<(usize, &Spec)> = doc
                .block
                .tags
                .iter()
                .enumerate()
                .filter(|(_, tag)| tag.tag == "param")
                .collect();
            if tags.is_empty() {
                continue;
            }
            let mut check = Check {
                doc: &doc,
                parameters: parameters(source, node),
                tags,
                findings: Vec::new(),
            };
            if !check.check_names(source) {
                check.check_paths(source);
            }
            findings.extend(check.findings);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CheckParamNames;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&CheckParamNames, "check-param-names", "src/a.ts");
    }
}

//! `@angular-eslint/template/prefer-contextual-for-variables`: a `@for` reads its contextual
//! variables (`$index`, `$count`, `$first`, `$last`, `$even`, `$odd`) rather than aliasing them,
//! and rather than computing them (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/prefer-contextual-for-variables.ts`). An alias is reported over its declaration,
//! unless `allowedAliases` names it or the loop is nested in another or holds one, where the
//! alias reaches an outer loop's variable; it is fixed by removing it and reading the variable
//! itself. A computation is reported over itself and fixed into its variable: `items.length`
//! of the loop's collection, `$index === 0`, `$index === $count - 1`, `$index % 2 === 0` and
//! their negations, each in the innermost loop around it, its own parameters and `@empty`
//! included. The project runs the plugin's default, no alias allowed.

use oxc_ast::AstKind;

use super::r3::{
    Literal, Ng, R3, equivalent, merged, of_bound, of_expression, replace, siblings, slice,
    unwrapped,
};
use super::{Report, Template, TemplateRule, every_bound, nested_at, placed};
use crate::template::bindings::{
    TemplateBinding, Value, block_parameters, interpolations, read_attribute,
};
use crate::template::expressions::{Bound, read};
use crate::template::lexer::Span;
use crate::template::tree::{Block, Node};

/// The rule and its option.
pub struct PreferContextualForVariables {
    /// The aliases allowed for each contextual variable.
    pub allowed_aliases: &'static [(&'static str, &'static [&'static str])],
}

impl PreferContextualForVariables {
    /// The plugin's default, which the project runs.
    pub const PROJECT: Self = Self {
        allowed_aliases: &[],
    };
}

/// The contextual variables a `let` may alias (`ALLOWED_FOR_LOOP_LET_VARIABLES`).
const CONTEXTUAL: [&str; 6] = ["$index", "$first", "$last", "$even", "$odd", "$count"];

/// An alias a `@for` declares: its name, the variable it reads, its declaration's span.
struct Alias {
    /// The alias.
    name: String,
    /// The contextual variable.
    value: String,
    /// From the alias to the variable's end.
    span: Span,
}

/// A loop being read (`ForLoopInfo`).
struct Loop {
    /// Whether nested, or holding a nested loop.
    can_alias: bool,
    /// Its collection.
    source: Ng,
    /// Its aliases.
    aliases: Vec<Alias>,
    /// The places each alias is read, by alias.
    references: Vec<Vec<(u32, u32)>>,
    /// The computations to simplify, each with its message's id, its span and its variable.
    simplifications: Vec<(&'static str, Span, &'static str)>,
}

/// The order the plugin reports the computations in.
const SIMPLIFICATIONS: [&str; 5] = [
    "preferCount",
    "preferFirst",
    "preferLast",
    "preferEven",
    "preferOdd",
];

/// The aliases of a `@for`'s `let` parameters, spanned as `parseLetParameters` spans them.
fn aliases(block: &Block) -> Vec<Alias> {
    let mut found = Vec::new();
    for parameter in block.parameters.iter().skip(1) {
        let text = parameter.expression.as_str();
        let Some(after_let) = text
            .strip_prefix("let")
            .filter(|rest| rest.starts_with(char::is_whitespace))
        else {
            continue;
        };
        let list = after_let.trim_start();
        let mut start = parameter.span.start as usize + text.len() - list.len();
        for part in list.split(',') {
            if let Some((name_part, value_part)) = part.split_once('=')
                && !value_part.contains('=')
            {
                let name = name_part.trim();
                let value = value_part.trim();
                let name_start = start + name_part.len() - name_part.trim_start().len();
                let value_start =
                    start + name_part.len() + 1 + value_part.len() - value_part.trim_start().len();
                if !name.is_empty() && CONTEXTUAL.contains(&value) {
                    found.push(Alias {
                        name: name.to_owned(),
                        value: value.to_owned(),
                        span: Span::new(name_start, value_start + value.len()),
                    });
                }
            }
            start += part.len() + 1;
        }
    }
    found
}

/// Whether an expression reads a contextual variable implicitly (`isContextualVariable`).
fn is_variable(node: &Ng, name: &str) -> bool {
    matches!(node, Ng::Read(receiver, read, false) if read == name && **receiver == Ng::Implicit)
}

/// Whether an expression is a number literal of a value (`isLiteralNumber`).
fn is_number(node: &Ng, value: f64) -> bool {
    matches!(node, Ng::Literal(Literal::Number(number)) if (*number - value).abs() < f64::EPSILON)
}

/// Whether an expression is `$index + 1` or `1 + $index`, parentheses aside.
fn is_index_plus_one(node: &Ng) -> bool {
    match unwrapped(node) {
        Ng::Binary(operator, left, right) if operator == "+" => {
            if is_variable(left, "$index") {
                is_number(right, 1.0)
            } else {
                is_variable(right, "$index") && is_number(left, 1.0)
            }
        }
        _ => false,
    }
}

/// Whether an expression is `$index % 2`, parentheses aside.
fn is_index_mod_two(node: &Ng) -> bool {
    matches!(unwrapped(node), Ng::Binary(operator, left, right)
        if operator == "%" && is_variable(left, "$index") && is_number(right, 2.0))
}

/// Whether an expression is `$count - 1` or `1 - $count`, parentheses aside.
fn is_count_minus_one(node: &Ng) -> bool {
    match unwrapped(node) {
        Ng::Binary(operator, left, right) if operator == "-" => {
            if is_variable(left, "$count") {
                is_number(right, 1.0)
            } else {
                is_variable(right, "$count") && is_number(left, 1.0)
            }
        }
        _ => false,
    }
}

/// The simplification of a comparison, as the plugin's `Binary` visitor finds it: its message's
/// id, the side simplified (`None` for the whole comparison) and the replacement.
fn simplify_binary(
    operator: &str,
    left: &Ng,
    right: &Ng,
) -> Vec<(&'static str, Option<bool>, &'static str)> {
    let equal = matches!(operator, "===" | "==");
    let unequal = matches!(operator, "!==" | "!=");
    let logical = matches!(operator, "&&" | "||");
    let pick = |is: bool, not_operator: &str, yes, no| {
        if equal && is {
            Some(yes)
        } else if unequal || operator == not_operator {
            Some(no)
        } else {
            None
        }
    };
    let mut found = Vec::new();
    let whole = |found: &mut Vec<_>, id, choice: Option<&'static str>| {
        if let Some(replacement) = choice {
            found.push((id, None, replacement));
        }
    };
    if is_variable(left, "$index") {
        if is_number(right, 0.0) {
            whole(
                &mut found,
                "preferFirst",
                pick(true, ">", "$first", "!$first"),
            );
        } else if is_count_minus_one(right) {
            whole(&mut found, "preferLast", pick(true, "<", "$last", "!$last"));
        }
    } else if is_number(left, 0.0) {
        if is_variable(right, "$index") {
            whole(
                &mut found,
                "preferFirst",
                pick(true, "<", "$first", "!$first"),
            );
        } else if is_index_mod_two(right) {
            let id = if equal { "preferEven" } else { "preferOdd" };
            whole(&mut found, id, pick(true, "<", "$even", "$odd"));
        }
    } else if is_number(left, 1.0) {
        if is_index_mod_two(right) {
            let id = if equal { "preferOdd" } else { "preferEven" };
            whole(&mut found, id, pick(true, ">", "$odd", "$even"));
        }
    } else if is_variable(left, "$count") {
        if is_index_plus_one(right) {
            whole(&mut found, "preferLast", pick(true, ">", "$last", "!$last"));
        }
    } else if is_index_plus_one(left) {
        if is_variable(right, "$count") {
            whole(&mut found, "preferLast", pick(true, "<", "$last", "!$last"));
        }
    } else if is_count_minus_one(left) {
        if is_variable(right, "$index") {
            whole(&mut found, "preferLast", pick(true, ">", "$last", "!$last"));
        }
    } else if is_index_mod_two(left) {
        if is_number(right, 0.0) {
            let id = if equal { "preferEven" } else { "preferOdd" };
            whole(&mut found, id, pick(true, ">", "$even", "$odd"));
        } else if is_number(right, 1.0) {
            let id = if equal { "preferOdd" } else { "preferEven" };
            whole(&mut found, id, pick(true, "<", "$odd", "$even"));
        } else if logical {
            found.push(("preferOdd", Some(true), "$odd"));
        }
    }
    if is_index_mod_two(right) && logical {
        found.push(("preferOdd", Some(false), "$odd"));
    }
    found
}

/// What a node of an expression gives: the name it reads, where, and a computation to
/// simplify there.
type Found = (
    Option<String>,
    Span,
    Option<(&'static str, Span, &'static str)>,
);

/// The rule's reading of a template: the loops open around the node being read.
struct Reader<'r, 't> {
    /// The template.
    template: &'r Template<'t>,
    /// The loops, the innermost last.
    loops: Vec<Loop>,
    /// The rule.
    rule: &'r PreferContextualForVariables,
    /// The reports.
    reports: &'r mut Vec<Report>,
}

impl Reader<'_, '_> {
    /// Records a computation to simplify in the innermost loop.
    fn record(&mut self, id: &'static str, span: Span, replacement: &'static str) {
        if let Some(innermost) = self.loops.last_mut() {
            innermost.simplifications.push((id, span, replacement));
        }
    }

    /// Reads an expression of the template in the innermost loop.
    fn read_bound(&mut self, written: &Bound) {
        if self.loops.is_empty() {
            return;
        }
        for bound in every_bound(written) {
            let Some(expression) = &bound.expression else {
                continue;
            };
            let mut found: Vec<Found> = Vec::new();
            let source = self.loops.last().map(|each| each.source.clone());
            visit(expression, &mut |kind| match kind {
                AstKind::IdentifierReference(identifier) => {
                    if nested_at(bound, identifier.span).is_none() {
                        found.push((
                            Some(identifier.name.to_string()),
                            placed(bound, identifier.span),
                            None,
                        ));
                    }
                }
                AstKind::StaticMemberExpression(member) if !member.optional => {
                    let span = placed(bound, member.span);
                    found.push((Some(member.property.name.to_string()), span, None));
                    let is_count = member.property.name == "length"
                        && source.as_ref().is_some_and(|source| {
                            equivalent(&of_expression(&member.object, bound), source)
                        });
                    if is_count {
                        found.push((None, span, Some(("preferCount", span, "$count"))));
                    }
                }
                AstKind::BinaryExpression(binary) => {
                    let sides = simplify_binary(
                        binary.operator.as_str(),
                        &of_expression(&binary.left, bound),
                        &of_expression(&binary.right, bound),
                    );
                    for (id, side, replacement) in sides {
                        let span = match side {
                            None => placed(bound, binary.span),
                            Some(true) => super::r3::span_of(bound, &binary.left),
                            Some(false) => super::r3::span_of(bound, &binary.right),
                        };
                        found.push((None, span, Some((id, span, replacement))));
                    }
                }
                AstKind::LogicalExpression(logical) => {
                    let sides = simplify_binary(
                        logical.operator.as_str(),
                        &of_expression(&logical.left, bound),
                        &of_expression(&logical.right, bound),
                    );
                    for (id, side, replacement) in sides {
                        let span = match side {
                            None => placed(bound, logical.span),
                            Some(true) => super::r3::span_of(bound, &logical.left),
                            Some(false) => super::r3::span_of(bound, &logical.right),
                        };
                        found.push((None, span, Some((id, span, replacement))));
                    }
                }
                AstKind::UnaryExpression(unary)
                    if unary.operator == oxc_ast::ast::UnaryOperator::LogicalNot =>
                {
                    let inner = of_expression(&unary.argument, bound);
                    let span = placed(bound, unary.span);
                    if is_variable(&inner, "$odd") || is_index_mod_two(&inner) {
                        found.push((None, span, Some(("preferEven", span, "$even"))));
                    } else if is_variable(&inner, "$even") {
                        found.push((None, span, Some(("preferOdd", span, "$odd"))));
                    }
                }
                AstKind::ConditionalExpression(conditional)
                    if is_index_mod_two(&of_expression(&conditional.test, bound)) =>
                {
                    let span = super::r3::span_of(bound, &conditional.test);
                    found.push((None, span, Some(("preferOdd", span, "$odd"))));
                }
                _ => {}
            });
            for (name, span, simplification) in found {
                if let Some(name) = name
                    && let Some(innermost) = self.loops.last_mut()
                    && let Some(index) = innermost
                        .aliases
                        .iter()
                        .position(|alias| alias.name == name)
                {
                    innermost.references[index].push((span.start, span.end));
                }
                if let Some((id, span, replacement)) = simplification {
                    self.record(id, span, replacement);
                }
            }
        }
    }

    /// Reads a block's parameters; an `@if`'s condition read as a truthy value too.
    fn read_parameters(&mut self, block: &Block) {
        let Ok(parameters) = block_parameters(self.template.allocator, block) else {
            return;
        };
        if matches!(block.name.as_str(), "if" | "else if")
            && let Some(condition) = &parameters.expression
            && condition.pipes.is_empty()
            && let Some(expression) = &condition.expression
            && is_index_mod_two(&of_bound(condition))
        {
            self.record(
                "preferOdd",
                super::r3::span_of(condition, expression),
                "$odd",
            );
        }
        for bound in parameters
            .expression
            .iter()
            .chain(&parameters.track)
            .chain(&parameters.conditions)
        {
            self.read_bound(bound);
        }
    }

    /// Reads a list of siblings.
    fn read_list(&mut self, nodes: &[Node]) {
        for node in siblings(nodes) {
            match node {
                R3::Element(element) => {
                    for attribute in &element.attrs {
                        let Ok(read) =
                            read_attribute(self.template.allocator, &element.name, attribute)
                        else {
                            continue;
                        };
                        match read.value {
                            Value::Binding(bound) => self.read_bound(&bound),
                            Value::Action(bounds) | Value::Interpolations(bounds) => {
                                for bound in &bounds {
                                    self.read_bound(bound);
                                }
                            }
                            Value::TemplateBindings(bindings) => {
                                for binding in bindings {
                                    if let TemplateBinding::Expression {
                                        value: Some(bound), ..
                                    } = binding
                                    {
                                        self.read_bound(&bound);
                                    }
                                }
                            }
                            Value::Text => {}
                        }
                    }
                    self.read_list(&element.children);
                }
                R3::Text(text) => {
                    let found = interpolations(self.template.allocator, &text.tokens);
                    for bound in found.unwrap_or_default() {
                        self.read_bound(&bound);
                    }
                }
                R3::If(branches) => {
                    for branch in branches {
                        self.read_parameters(branch);
                        self.read_list(&branch.children);
                    }
                }
                R3::For(main, empty) => self.read_for(main, empty),
                R3::Other(node) => match node {
                    Node::Block(block) => {
                        self.read_parameters(block);
                        self.read_list(&block.children);
                    }
                    Node::LetDeclaration(declaration) => {
                        let value = read(
                            self.template.allocator,
                            &declaration.value,
                            declaration.value_span.start,
                        );
                        if let Ok(bound) = value {
                            self.read_bound(&bound);
                        }
                    }
                    Node::Expansion(expansion) => {
                        for case in &expansion.cases {
                            self.read_list(&case.expression);
                        }
                    }
                    _ => {}
                },
            }
        }
    }

    /// Reads a `@for`: its loop open over its parameters, its content and its `@empty`.
    fn read_for(&mut self, main: &Block, empty: Option<&Block>) {
        let nested = !self.loops.is_empty();
        if let Some(outer) = self.loops.last_mut() {
            outer.can_alias = true;
        }
        let source = block_parameters(self.template.allocator, main)
            .ok()
            .and_then(|parameters| parameters.expression.as_ref().map(of_bound))
            .unwrap_or(Ng::Other);
        let found = aliases(main);
        self.loops.push(Loop {
            can_alias: nested,
            source,
            references: found.iter().map(|_| Vec::new()).collect(),
            aliases: found,
            simplifications: Vec::new(),
        });
        self.read_list(&main.children);
        if let Some(empty) = empty {
            self.read_list(&empty.children);
        }
        self.read_parameters(main);
        if let Some(done) = self.loops.pop() {
            self.report(&done);
        }
    }

    /// Reports a loop once read (`ForLoopBlock:exit`).
    fn report(&mut self, done: &Loop) {
        let text = self.template.text;
        if !done.can_alias {
            for (index, alias) in done.aliases.iter().enumerate() {
                let allowed = self
                    .rule
                    .allowed_aliases
                    .iter()
                    .find(|(variable, _)| *variable == alias.value)
                    .is_some_and(|(_, names)| names.contains(&alias.name.as_str()));
                if allowed {
                    continue;
                }
                let (start, end) = range_to_remove(text, alias, index, done.aliases.len());
                let mut fixes = vec![replace(start, end, "")];
                for (from, to) in &done.references[index] {
                    fixes.push(replace(*from, *to, &alias.value));
                }
                self.reports.push(Report {
                    data: vec![("name", alias.value.clone())],
                    fix: merged(text, fixes),
                    ..Report::new(alias.span, "preferContextualVariable")
                });
            }
        }
        for kind in SIMPLIFICATIONS {
            for (id, span, replacement) in &done.simplifications {
                if *id != kind {
                    continue;
                }
                self.reports.push(Report {
                    data: vec![("expression", slice(text, span.start, span.end).to_owned())],
                    fix: Some(replace(span.start, span.end, replacement)),
                    ..Report::new(*span, id)
                });
            }
        }
    }
}

/// Visits each node of an Oxc expression.
fn visit<'a>(expression: &oxc_ast::ast::Expression<'a>, each: &mut dyn FnMut(AstKind<'a>)) {
    /// Calls the visit on each node.
    struct Visitor<'v, 'a>(&'v mut dyn FnMut(AstKind<'a>));
    impl<'a> oxc_ast_visit::Visit<'a> for Visitor<'_, 'a> {
        fn enter_node(&mut self, kind: AstKind<'a>) {
            (self.0)(kind);
        }
    }
    oxc_ast_visit::Visit::visit_expression(&mut Visitor(each), expression);
}

/// The text before a byte where a token last starts (`getStartOfPreviousToken`).
fn previous_token(text: &str, token: &str, before: usize) -> Option<usize> {
    let mut at = before.checked_sub(token.len())?;
    loop {
        if text.get(at..at + token.len()) == Some(token) {
            return Some(at);
        }
        at = at.checked_sub(1)?;
    }
}

/// The range of an alias's declaration to remove (`getVariableRangeToRemove`): with its `let`
/// and the `;` before when it is alone, with the `,` before or after it otherwise.
fn range_to_remove(text: &str, alias: &Alias, index: usize, count: usize) -> (u32, u32) {
    let mut start = alias.span.start as usize;
    let mut end = alias.span.end as usize;
    let let_at = previous_token(text, "let", start);
    let own_let = let_at.is_some_and(|let_at| {
        let between = text.get(let_at + 3..start).unwrap_or_default();
        let after = text.get(end..).unwrap_or_default().trim_start();
        !between.contains(',') && !after.starts_with(',')
    });
    if count == 1 || own_let {
        if let Some(let_at) = let_at {
            start = previous_token(text, ";", let_at).unwrap_or(let_at);
        }
    } else if index == 0 {
        if let Some(comma) = text.get(end..).and_then(|rest| rest.find(',')) {
            let after = end + comma + 1;
            let rest = text.get(after..).unwrap_or_default();
            end = after + rest.len() - rest.trim_start().len();
        }
    } else {
        start = previous_token(text, ",", start).unwrap_or(start);
    }
    (
        crate::template::lexer::offset(start),
        crate::template::lexer::offset(end),
    )
}

impl TemplateRule for PreferContextualForVariables {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/prefer-contextual-for-variables"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[
            (
                "preferContextualVariable",
                "Use the '{{name}}' contextual variable instead of aliasing it.",
            ),
            ("preferCount", "Use '$count' instead of '{{ expression }}'."),
            ("preferFirst", "Use '$first' instead of '{{ expression }}'."),
            ("preferLast", "Use '$last' instead of '{{ expression }}'."),
            ("preferEven", "Use '$even' instead of '{{ expression }}'."),
            ("preferOdd", "Use '$odd' instead of '{{ expression }}'."),
        ]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let mut reader = Reader {
            template,
            loops: Vec::new(),
            rule: self,
            reports,
        };
        reader.read_list(template.nodes);
    }
}

#[cfg(test)]
mod tests {
    use super::PreferContextualForVariables;
    use crate::template::corpus::Value;
    use crate::template::rules::tester::run;

    /// The rule with a case's `allowedAliases`.
    fn build(options: Option<&Value>) -> PreferContextualForVariables {
        let mut allowed: Vec<(&'static str, &'static [&'static str])> = Vec::new();
        if let Some(Value::Object(variables)) = options.and_then(|each| each.get("allowedAliases"))
        {
            for (variable, names) in variables {
                let names: Vec<&'static str> = match names {
                    Value::List(list) => list
                        .iter()
                        .filter_map(|name| match name {
                            Value::Text(text) => Some(&*String::leak(text.clone())),
                            _ => None,
                        })
                        .collect(),
                    _ => Vec::new(),
                };
                allowed.push((String::leak(variable.clone()), Vec::leak(names)));
            }
        }
        PreferContextualForVariables {
            allowed_aliases: Vec::leak(allowed),
        }
    }

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("prefer-contextual-for-variables.txt", &[], &|options| {
            Box::new(build(options))
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 50 valid and 88 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (50, 88, 0));
    }
}

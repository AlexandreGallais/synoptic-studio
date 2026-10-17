//! `unicorn/name-replacements`: no abbreviated name — `e`, `err`, `cb`, `args`, `idx` and the
//! other names of the plugin's list — for a declared name or a file, but the full word
//! (`eslint-plugin-unicorn` 78.0.0, its default options: names declared and file names checked,
//! properties not, imports of packages and shorthand destructurings left alone). A name with one
//! replacement is autofixed everywhere it is used, as in `ESLint`, unless it is exported or a
//! parameter documented by `@param`; several replacements are suggestions there: reported only.

use oxc_ast::AstKind;
use oxc_semantic::{NodeId, SymbolId};

use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::identifiers::{
    GeneratedNames, available_name, checked_symbols, declared_span, is_renamable, is_upper_case,
    is_upper_first, lower_first, rename_edits, scopes_of, upper_first,
};
use crate::rules::unicorn::syntax::combined_fix;
use crate::source::Source;

/// The rule.
pub struct NameReplacements;

/// The names replaced, each with the names it may become, in the plugin's order.
const REPLACEMENTS: [(&str, &[&str]); 92] = [
    ("acc", &["accumulator"]),
    ("application", &["app"]),
    ("applications", &["apps"]),
    ("arg", &["argument"]),
    ("args", &["arguments"]),
    ("arr", &["array"]),
    ("attr", &["attribute"]),
    ("attrs", &["attributes"]),
    ("btn", &["button"]),
    ("buf", &["buffer"]),
    ("cb", &["callback"]),
    ("cfg", &["config"]),
    ("cmd", &["command"]),
    ("conf", &["config"]),
    ("configuration", &["config"]),
    ("ctx", &["context"]),
    ("cur", &["current"]),
    ("curr", &["current"]),
    ("db", &["database"]),
    ("decl", &["declaration"]),
    ("decls", &["declarations"]),
    ("def", &["defer", "deferred", "define", "definition"]),
    ("dep", &["dependency"]),
    ("deps", &["dependencies"]),
    ("dest", &["destination"]),
    ("dev", &["development"]),
    ("dir", &["direction", "directory"]),
    ("dirs", &["directories"]),
    ("dist", &["distribution"]),
    ("doc", &["document"]),
    ("docs", &["documentation", "documents"]),
    (
        "dst",
        &["daylightSavingTime", "destination", "distribution"],
    ),
    ("e", &["error", "event"]),
    ("el", &["element"]),
    ("elem", &["element"]),
    ("elems", &["elements"]),
    ("env", &["environment"]),
    ("envs", &["environments"]),
    ("err", &["error"]),
    ("ev", &["event"]),
    ("evt", &["event"]),
    ("expr", &["expression"]),
    ("exprs", &["expressions"]),
    ("ext", &["extension"]),
    ("exts", &["extensions"]),
    ("fn", &["function"]),
    ("func", &["function"]),
    ("i", &["index"]),
    ("ident", &["identifier"]),
    ("idents", &["identifiers"]),
    ("idx", &["index"]),
    ("j", &["index"]),
    ("len", &["length"]),
    ("lib", &["library"]),
    ("mod", &["module"]),
    ("msg", &["message"]),
    ("num", &["number"]),
    ("obj", &["object"]),
    ("opts", &["options"]),
    ("param", &["parameter"]),
    ("params", &["parameters"]),
    ("perf", &["performance"]),
    ("pkg", &["package"]),
    ("prev", &["previous"]),
    ("prod", &["production"]),
    ("prop", &["property"]),
    ("props", &["properties"]),
    ("proto", &["prototype"]),
    ("ref", &["reference"]),
    ("refs", &["references"]),
    ("rel", &["related", "relationship", "relative"]),
    ("repo", &["repository"]),
    ("req", &["request"]),
    ("res", &["resource", "response", "result"]),
    ("ret", &["returnValue"]),
    ("retval", &["returnValue"]),
    ("sep", &["separator"]),
    ("src", &["source"]),
    ("stdDev", &["standardDeviation"]),
    ("stmt", &["statement"]),
    ("stmts", &["statements"]),
    ("str", &["string"]),
    ("tbl", &["table"]),
    ("temp", &["temporary"]),
    ("tit", &["title"]),
    ("tmp", &["temporary"]),
    ("util", &["utility"]),
    ("utils", &["utilities"]),
    ("val", &["value"]),
    ("var", &["variable"]),
    ("vars", &["variables"]),
    ("ver", &["version"]),
];

/// The names never replaced, the plugin's allow list.
const ALLOW_LIST: [&str; 12] = [
    "defaultProps",
    "devDependencies",
    "EmberENV",
    "getDerivedStateFromProps",
    "getInitialProps",
    "getServerSideProps",
    "getStaticProps",
    "ImportMetaEnv",
    "iOS",
    "ProcessEnv",
    "propTypes",
    "setupFilesAfterEnv",
];

/// The texts a name holding one is left alone for: internationalization, accessibility, tests
/// and libraries known by their abbreviation.
const IGNORED: [&str; 6] = ["i18n", "l10n", "a11y", "e2e", "jQuery", "vite-env"];

/// The most replacements a message shows.
const SAMPLES: usize = 3;

/// The replacements of one word: none for a constant or an allowed word, else the list's names
/// for it, in its case, sorted.
fn word_replacements(word: &str) -> Vec<String> {
    if is_upper_case(word) || ALLOW_LIST.contains(&word) {
        return Vec::new();
    }
    let lower = lower_first(word);
    let upper = upper_first(word);
    let Some((_, names)) = REPLACEMENTS
        .iter()
        .find(|(name, _)| *name == lower)
        .or_else(|| REPLACEMENTS.iter().find(|(name, _)| *name == word))
        .or_else(|| REPLACEMENTS.iter().find(|(name, _)| *name == upper))
    else {
        return Vec::new();
    };
    let mut found: Vec<String> = names
        .iter()
        .map(|name| {
            if is_upper_first(word) {
                upper_first(name)
            } else {
                lower_first(name)
            }
        })
        .collect();
    found.sort_by(|a, b| {
        a.to_lowercase()
            .cmp(&b.to_lowercase())
            .then_with(|| b.cmp(a))
    });
    found
}

/// The words of a name: a boundary before each character that is no lowercase letter, and after
/// each that is no letter.
fn words(name: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    let mut previous: Option<char> = None;
    for character in name.chars() {
        let boundary =
            previous.is_some_and(|before| !character.is_lowercase() || !before.is_alphabetic());
        if boundary || found.is_empty() {
            found.push(String::new());
        }
        if let Some(last) = found.last_mut() {
            last.push(character);
        }
        previous = Some(character);
    }
    found
}

/// How many names replace a name, and the first few.
struct Replacements {
    /// Their number.
    total: usize,
    /// The first ones.
    samples: Vec<String>,
}

/// The replacements of a name: of the whole name, else of each of its words, combined.
fn name_replacements(name: &str) -> Replacements {
    let none = Replacements {
        total: 0,
        samples: Vec::new(),
    };
    if is_upper_case(name)
        || ALLOW_LIST.contains(&name)
        || IGNORED.iter().any(|each| name.contains(each))
    {
        return none;
    }
    let exact = word_replacements(name);
    if !exact.is_empty() {
        return Replacements {
            total: exact.len(),
            samples: exact.into_iter().take(SAMPLES).collect(),
        };
    }
    if name.bytes().all(|byte| byte.is_ascii_lowercase()) {
        return none;
    }
    let combinations: Vec<Vec<String>> = words(name)
        .into_iter()
        .map(|word| {
            let replaced = word_replacements(&word);
            if replaced.is_empty() {
                vec![word]
            } else {
                replaced
            }
        })
        .collect();
    let replaced = words(name)
        .iter()
        .any(|word| !word_replacements(word).is_empty());
    if !replaced {
        return none;
    }
    let total = combinations.iter().map(Vec::len).product::<usize>();
    let samples = (0..total.min(SAMPLES))
        .map(|sample| {
            let mut remaining = sample;
            let mut parts: Vec<String> = Vec::new();
            for items in combinations.iter().rev() {
                let index = remaining % items.len();
                remaining /= items.len();
                parts.insert(0, items[index].clone());
            }
            let mut index = parts.len() - 1;
            while index > 0 {
                let word = &parts[index];
                if word.chars().all(|each| each.is_ascii_alphabetic())
                    && parts[index - 1].ends_with(word.as_str())
                {
                    parts.remove(index);
                }
                index -= 1;
            }
            parts.concat()
        })
        .collect();
    Replacements { total, samples }
}

/// The message naming the replacements of a name of a kind.
fn message(kind: &str, name: &str, replacements: &Replacements) -> String {
    let another = "A more descriptive name will do too.";
    if replacements.total == 1 {
        return format!(
            "The {kind} `{name}` should be named `{}`. {another}",
            replacements.samples.first().map_or("", String::as_str)
        );
    }
    let mut listed = replacements
        .samples
        .iter()
        .map(|each| format!("`{each}`"))
        .collect::<Vec<_>>()
        .join(", ");
    let omitted = replacements.total - replacements.samples.len();
    if omitted > 0 {
        let count = if omitted > 99 {
            "99+".to_owned()
        } else {
            omitted.to_string()
        };
        listed.push_str(", ... (");
        listed.push_str(&count);
        listed.push_str(" more omitted)");
    }
    format!("Please rename the {kind} `{name}`. Suggested names are: {listed}. {another}")
}

/// The offset of the file's first token, where `ESLint` places the program.
fn first_token(text: &str) -> u32 {
    let mut at = 0;
    loop {
        let rest = &text[at..];
        let trimmed = rest.trim_start();
        at += rest.len() - trimmed.len();
        if trimmed.starts_with("//") {
            at += trimmed.find('\n').unwrap_or(trimmed.len());
        } else if trimmed.starts_with("/*") {
            at += trimmed.find("*/").map_or(trimmed.len(), |end| end + 2);
        } else {
            return u32::try_from(at).unwrap_or(0);
        }
    }
}

/// The node of the function a parameter belongs to, when the symbol is one.
fn parameter_function(source: &Source, symbol: SymbolId) -> Option<NodeId> {
    let nodes = source.semantic.nodes();
    let declaration = source.semantic.scoping().symbol_declaration(symbol);
    let in_parameters = matches!(
        nodes.kind(declaration),
        AstKind::FormalParameter(_) | AstKind::FormalParameterRest(_)
    ) || nodes
        .ancestors(declaration)
        .take_while(|node| {
            !matches!(
                node.kind(),
                AstKind::Function(_) | AstKind::ArrowFunctionExpression(_)
            )
        })
        .any(|node| matches!(node.kind(), AstKind::FormalParameters(_)));
    if !in_parameters {
        return None;
    }
    nodes
        .ancestors(declaration)
        .find(|node| {
            matches!(
                node.kind(),
                AstKind::Function(_)
                    | AstKind::ArrowFunctionExpression(_)
                    | AstKind::TSCallSignatureDeclaration(_)
                    | AstKind::TSConstructSignatureDeclaration(_)
                    | AstKind::TSConstructorType(_)
                    | AstKind::TSFunctionType(_)
                    | AstKind::TSMethodSignature(_)
            )
        })
        .map(oxc_semantic::AstNode::id)
}

/// Whether a function carries, right above it or its declaration, a `JSDoc` block with `@param`.
fn has_param_doc(source: &Source, function: NodeId) -> bool {
    let nodes = source.semantic.nodes();
    let mut node = function;
    loop {
        let start = oxc_span::GetSpan::span(&nodes.kind(node)).start;
        let comment = source
            .program
            .comments
            .iter()
            .filter(|comment| comment.span.end <= start)
            .max_by_key(|comment| comment.span.end)
            .filter(|comment| {
                source.text[comment.span.end as usize..start as usize]
                    .trim()
                    .is_empty()
            });
        if let Some(comment) = comment {
            let adjacent = source.line(comment.span.end) + 1 >= source.line(start);
            let value = source.comment_value(comment);
            return adjacent
                && comment.is_block()
                && value.trim_start().starts_with('*')
                && value.contains("@param");
        }
        let parent = nodes.parent_node(node);
        let attaches = matches!(
            parent.kind(),
            AstKind::AssignmentExpression(_)
                | AstKind::ExportDefaultDeclaration(_)
                | AstKind::ExportDeclaration(_)
                | AstKind::ExpressionStatement(_)
                | AstKind::MethodDefinition(_)
                | AstKind::ObjectProperty(_)
                | AstKind::PropertyDefinition(_)
                | AstKind::TSPropertySignature(_)
                | AstKind::TSTypeAliasDeclaration(_)
                | AstKind::TSTypeAnnotation(_)
                | AstKind::VariableDeclaration(_)
                | AstKind::VariableDeclarator(_)
        );
        if !attaches {
            return false;
        }
        node = parent.id();
    }
}

/// Whether a parameter property of a constructor declares a symbol.
fn is_parameter_property(source: &Source, symbol: SymbolId) -> bool {
    let nodes = source.semantic.nodes();
    let declaration = source.semantic.scoping().symbol_declaration(symbol);
    matches!(nodes.kind(declaration), AstKind::FormalParameter(parameter) if parameter.has_modifier())
}

impl NameReplacements {
    /// Reports a declared name the list replaces, fixed when one name replaces it.
    fn report_symbol(
        &self,
        source: &Source,
        symbol: SymbolId,
        generated: &mut GeneratedNames,
        findings: &mut Vec<Finding>,
    ) {
        let name = source.semantic.scoping().symbol_name(symbol);
        let mut replacements = name_replacements(name);
        if replacements.total == 0 {
            return;
        }
        let scopes = scopes_of(source, symbol);
        replacements.samples = replacements
            .samples
            .iter()
            .filter_map(|each| available_name(source, each, &scopes, generated))
            .collect();
        if replacements.samples.is_empty() {
            return;
        }
        let at = declared_span(source, symbol).start;
        let text = message("variable", name, &replacements);
        let fixable = replacements.total == 1
            && is_renamable(source, symbol)
            && !is_parameter_property(source, symbol)
            && parameter_function(source, symbol)
                .is_none_or(|function| !has_param_doc(source, function));
        let fix = fixable
            .then(|| {
                let replacement = &replacements.samples[0];
                generated.add(replacement, &scopes);
                combined_fix(source, rename_edits(source, symbol, replacement))
            })
            .flatten();
        findings.push(match fix {
            Some(fix) => source.fixed(self.name(), at, text, fix),
            None => source.finding(self.name(), at, text),
        });
    }
}

impl Rule for NameReplacements {
    fn name(&self) -> &'static str {
        "unicorn/name-replacements"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let file = source.name();
        let (stem, extension) = file
            .rfind('.')
            .filter(|at| *at > 0)
            .map_or((file, ""), |at| (&file[..at], &file[at..]));
        let mut replacements = name_replacements(stem);
        if replacements.total > 0 {
            replacements.samples = replacements
                .samples
                .iter()
                .map(|each| format!("{each}{extension}"))
                .collect();
            findings.push(source.finding(
                self.name(),
                first_token(source.text),
                message("filename", file, &replacements),
            ));
        }
        let mut generated = GeneratedNames::default();
        for symbol in checked_symbols(source) {
            self.report_symbol(source, symbol, &mut generated, findings);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NameReplacements;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code in a file of a name.
    fn check_at(path: &str, code: &str) -> Vec<String> {
        messages(&lint(&NameReplacements, path, code))
    }

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        check_at("playground/a.ts", code)
    }

    /// The code once fixed.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&NameReplacements, "playground/a.ts", code))
    }

    #[test]
    fn fixes_a_name_with_one_replacement_everywhere() {
        let code = "const err = new Error();\nconsole.log(err, {err});";
        assert_eq!(
            check(code),
            ["The variable `err` should be named `error`. A more descriptive name will do too."]
        );
        assert_eq!(
            fix(code),
            "const error = new Error();\nconsole.log(error, {err: error});"
        );
        assert_eq!(
            fix("const errCb = 1;\nconst errorCallback = 2;\nuse(errCb, errorCallback);"),
            "const errorCallback_ = 1;\nconst errorCallback = 2;\nuse(errorCallback_, errorCallback);"
        );
        assert_eq!(
            fix("function f(args) { return args; }"),
            "function f(arguments_) { return arguments_; }"
        );
        assert_eq!(
            fix("const retVal = 1; use(retVal);"),
            "const returnValue = 1; use(returnValue);"
        );
        assert_eq!(
            fix("type Props = {}; let a: Props;"),
            "type Properties = {}; let a: Properties;"
        );
    }

    #[test]
    fn reports_several_replacements_without_a_fix() {
        let code = "const e = 1; use(e);";
        assert_eq!(
            check(code),
            [
                "Please rename the variable `e`. Suggested names are: `error`, `event`. A more descriptive name will do too."
            ]
        );
        assert_eq!(fix(code), code);
        assert_eq!(
            check("const resDef = 1;"),
            [
                "Please rename the variable `resDef`. Suggested names are: `resourceDefer`, `resourceDeferred`, `resourceDefine`, ... (9 more omitted). A more descriptive name will do too."
            ]
        );
    }

    #[test]
    fn reports_the_documented_examples() {
        assert_eq!(
            check("const e = new Error();"),
            [
                "Please rename the variable `e`. Suggested names are: `error`, `event`. A more descriptive name will do too."
            ]
        );
        assert_eq!(fix("class Btn {}"), "class Button {}");
        for code in [
            "const levels = {\n\terr: 0\n};",
            "this.evt = 'click';",
            "enum Level { err }",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
        let documented_rest =
            "/**\n * @param args - the rest\n */\nfunction f(...args) { return args; }";
        assert_eq!(fix(documented_rest), documented_rest);
    }

    #[test]
    fn carries_the_default_options_eslint_runs_it_with() {
        // `eslint/rules/unicorn/naming.ts` sets no option: names and file names are checked,
        // properties, shorthand destructurings and imports of packages are not.
        assert_eq!(check("const err = 1;").len(), 1);
        assert_eq!(check_at("src/err.ts", "export {};").len(), 1);
        assert_eq!(check("import * as utils from './utilities';").len(), 1);
        for code in [
            "const object = {err: 1};",
            "const {err} = object;",
            "import * as utils from 'package';",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }

    #[test]
    fn reports_a_file_name() {
        assert_eq!(
            check_at("src/utils.ts", "export {};"),
            [
                "The filename `utils.ts` should be named `utilities.ts`. A more descriptive name will do too."
            ]
        );
        assert_eq!(check_at("src/vite-env.d.ts", "export {};"), NONE);
    }

    #[test]
    fn leaves_exports_documented_parameters_packages_and_constants_unfixed_or_alone() {
        let exported = "export const err = 1;";
        assert_eq!(check(exported).len(), 1);
        assert_eq!(fix(exported), exported);
        let documented = "/**\n * @param err - the error\n */\nfunction f(err) { return err; }";
        assert_eq!(fix(documented), documented);
        for code in [
            "import err from 'some-package';\nuse(err);",
            "import {err} from 'some-package';\nuse(err);",
            "const {err} = object;",
            "const ERR = 1;",
            "const error = 1;",
            "const i18nKey = 1;",
            "const propTypes = 1;",
            "const object = {err: 1};",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
        assert_eq!(check("import err from './error';").len(), 1);
    }
}

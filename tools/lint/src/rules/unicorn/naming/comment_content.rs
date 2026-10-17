//! `unicorn/comment-content`: a comment writes a known name or acronym as it is spelled,
//! `GitHub`, `JSON`, `Node.js`, and not `github`, `json`, `nodejs` (`eslint-plugin-unicorn`
//! 78.0.0). It reads prose only: code, commented-out code, links, paths, domains, packages,
//! markup, quoted and inline code, and the syntax of `JSDoc` tags are masked first. One
//! replacement per comment, the first in it; autofixed, as in `ESLint`.

use std::sync::LazyLock;

use oxc_ast::ast::Comment;

use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::pattern::{Pattern, is_space};
use crate::source::Source;

/// The rule.
pub struct CommentContent;

/// The character a masked region is written with, the same width as what it hides.
const MASK: char = '\u{FFFF}';

/// A word character of any script, for the boundaries of a replacement.
const WORD: &str = r"[\p{Letter}\p{Mark}\p{Number}_]";

/// The plugin's replacements: a pattern, its replacement, and whether it is case-sensitive.
const DEFAULT_REPLACEMENTS: [(&str, &str, bool); 120] = [
    (r"\bapplication\b(?!/)", "app", true),
    (r"\bapplications\b", "apps", true),
    (r"\bapi\b", "API", false),
    (r"\bascii\b", "ASCII", false),
    (r"\bast\b", "AST", false),
    (r"\bavif\b", "AVIF", false),
    (r"\baws\b", "AWS", false),
    (r"\bazure\b", "Azure", false),
    (r"\bbitcoin\b", "Bitcoin", false),
    (r"\bbom\b", "BOM", false),
    (r"\bcd\b", "CD", false),
    (r"\bcdn\b", "CDN", false),
    (r"\bci\b", "CI", false),
    (r"\bcjs\b", "CJS", false),
    (r"\bcli\b", "CLI", false),
    (r"\bcors\b", "CORS", false),
    (r"\bcpu\b", "CPU", false),
    (r"\bcrud\b", "CRUD", false),
    (r"\bcsrf\b", "CSRF", false),
    (r"\bcsv\b", "CSV", false),
    (r"\bcss\b", "CSS", false),
    (r"\bdeno\b", "Deno", false),
    (r"\bdevops\b", "DevOps", false),
    (r"\bdns\b", "DNS", false),
    (r"\bdocker\b", "Docker", false),
    (r"\bdom\b", "DOM", false),
    (r"\bdiscord\b", "Discord", false),
    (r"\bec2\b", "EC2", false),
    (r"\beof\b", "EOF", false),
    (r"\beol\b", "EOL", false),
    (r"\beslint(?:\.js)?\b", "ESLint", false),
    (r"\besm\b", "ESM", false),
    (r"\bfacebook\b", "Facebook", false),
    (r"\bftp\b", "FTP", false),
    (r"\bgcp\b", "GCP", false),
    (r"\bgif\b", "GIF", false),
    (r"\bgit\b", "Git", false),
    (r"\bgithub\b", "GitHub", false),
    (r"\bgpu\b", "GPU", false),
    (r"\bgraphql\b", "GraphQL", false),
    (r"\bgrunt(?:\.js)?\b", "Grunt", false),
    (r"\bgulp(?:\.js)?\b", "Gulp", false),
    (r"\bhtml\b", "HTML", false),
    (r"\bhttps\b", "HTTPS", false),
    (r"\bhttp\b", "HTTP", false),
    (r"\bide\b", "IDE", false),
    (r"\biife\b", "IIFE", false),
    (r"\bios\b", "iOS", false),
    (r"\bjavascript\b", "JavaScript", false),
    (r"\bjpeg\b", "JPEG", false),
    (r"\bjpg\b", "JPG", false),
    (r"\bjson\b", "JSON", false),
    (r"\bjsx\b", "JSX", false),
    (r"\bjquery\b", "jQuery", false),
    (r"\bjwt\b", "JWT", false),
    (r"\bk8s\b", "K8s", false),
    (r"\bkubernetes\b", "Kubernetes", false),
    (r"\blinux\b", "Linux", false),
    (r"\blsp\b", "LSP", false),
    (r"\bltr\b", "LTR", false),
    (r"\bmdn\b", "MDN", false),
    (r"\bmime\b", "MIME", false),
    (r"\bmongodb\b", "MongoDB", false),
    (r"\bmysql\b", "MySQL", false),
    (r"\bnfc\b", "NFC", false),
    (r"\bnfd\b", "NFD", false),
    (r"\bnginx\b", "NGINX", false),
    (r"\bnode\.?js\b", "Node.js", false),
    (r"\bnpm\b", "npm", false),
    (r"\boauth\b", "OAuth", false),
    (r"\bpdf\b", "PDF", false),
    (r"\bpng\b", "PNG", false),
    (r"\bposix\b", "POSIX", false),
    (r"\bpostgresql\b", "PostgreSQL", false),
    (r"\bram\b", "RAM", false),
    (r"\breact\.?js\b", "React", false),
    (r"\breddit\b", "Reddit", false),
    (r"\bredis\b", "Redis", false),
    (r"\brtl\b", "RTL", false),
    (r"\bs3\b", "S3", false),
    (r"\bsdk\b", "SDK", false),
    (r"\bsftp\b", "SFTP", false),
    (r"\bsql\b", "SQL", false),
    (r"\bsqlite\b", "SQLite", false),
    (r"\bssh\b", "SSH", false),
    (r"\bssl\b", "SSL", false),
    (r"\bstack\s?overflow\b", "Stack Overflow", false),
    (r"\bstderr\b", "stderr", false),
    (r"\bstdin\b", "stdin", false),
    (r"\bstdout\b", "stdout", false),
    (r"\bsvelte\b", "Svelte", false),
    (r"\bsvg\b", "SVG", false),
    (r"\btcp\b", "TCP", false),
    (r"\btls\b", "TLS", false),
    (r"\btsv\b", "TSV", false),
    (r"\btwitch(?:tv)?\b", "Twitch", false),
    (r"\btypescript\b", "TypeScript", false),
    (r"\bui\b", "UI", false),
    (r"\budp\b", "UDP", false),
    (r"\bumd\b", "UMD", false),
    (r"\bunicode\b", "Unicode", false),
    (r"\bunix\b", "Unix", false),
    (r"\burl\b", "URL", false),
    (r"\buri\b", "URI", false),
    (r"\butf-?8\b", "UTF-8", false),
    (r"\bux\b", "UX", false),
    (r"\buuid\b", "UUID", false),
    (r"\bvite\b", "Vite", false),
    (r"\bvue(?:\.?js)?\b", "Vue.js", false),
    (r"\bvs\s?code\b", "VS Code", false),
    (r"\b(?:webassembly|wasm)\b", "WebAssembly", false),
    (r"\bwebgl\b", "WebGL", false),
    (r"\bwebp\b", "WebP", false),
    (r"\bwebrtc\b", "WebRTC", false),
    (r"\bwebsocket\b", "WebSocket", false),
    (r"\bxml\b", "XML", false),
    (r"\bxss\b", "XSS", false),
    (r"\byaml\b", "YAML", false),
    (r"\byou\s?tube\b", "YouTube", false),
    (r"\b(?:mac\s?os(?!\s?x)|(mac\s?)?os\s?x)\b", "macOS", false),
];

/// A replacement ready to run: its pattern, compiled, the word any match holds, and what it
/// writes.
struct Replacement {
    /// The compiled pattern.
    pattern: Pattern,
    /// The literal word the pattern starts with, lower case when the pattern ignores case.
    required: Option<String>,
    /// What a match becomes.
    text: String,
}

/// The pattern of a replacement of the plugin's, bounded by word characters of any script.
fn bounded(pattern: &str) -> String {
    format!("(?<!{WORD}){pattern}(?!{WORD})")
}

/// The replacements the project's configuration adds or drops, by pattern, `None` dropping one:
/// `application` and `applications` kept in comments, the words of the domain and the ADRs (the
/// library and its applications). `eslint/rules/unicorn/naming.ts` named it `application` until
/// EN-080, a key no pattern of the plugin has, so `ESLint` kept the replacement; the key is now
/// the plugin's own pattern, so that the option does what it meant; the plural kept by EN-093.
const PROJECT_REPLACEMENTS: [(&str, Option<&str>); 2] =
    [(r"\bapplication\b(?!/)", None), (r"\bapplications\b", None)];

/// The replacements in force: the plugin's, but those the project drops or rewrites, then the
/// project's own (`prepareReplacements`).
static REPLACEMENTS: LazyLock<Vec<Replacement>> = LazyLock::new(|| {
    let own = PROJECT_REPLACEMENTS
        .iter()
        .filter_map(|(pattern, replacement)| {
            replacement.map(|replacement| Replacement {
                pattern: Pattern::new(pattern, "u"),
                required: required_text(pattern),
                text: replacement.to_owned(),
            })
        });
    DEFAULT_REPLACEMENTS
        .iter()
        .filter(|(pattern, _, _)| !PROJECT_REPLACEMENTS.iter().any(|(each, _)| each == pattern))
        .map(|(pattern, replacement, case_sensitive)| Replacement {
            pattern: Pattern::new(&bounded(pattern), if *case_sensitive { "u" } else { "iu" }),
            required: required_text(pattern).map(|word| {
                if *case_sensitive {
                    word
                } else {
                    word.to_lowercase()
                }
            }),
            text: (*replacement).to_owned(),
        })
        .chain(own)
        .collect()
});

/// The plugin's acronyms, whole words in capitals, as one pattern matching a whole text.
static ACRONYM_TERM: LazyLock<Pattern> = LazyLock::new(|| {
    let terms: Vec<String> = acronyms()
        .map(|(pattern, _, _)| format!("(?:{})", bounded(pattern)))
        .collect();
    Pattern::new(&format!("^(?:{})$", terms.join("|")), "iv")
});

/// The longest text an acronym's pattern can match, when each is a run of plain characters.
static MAX_ACRONYM_LENGTH: LazyLock<usize> = LazyLock::new(|| {
    acronyms()
        .map(|(pattern, _, _)| {
            pattern
                .strip_prefix(r"\b")
                .and_then(|rest| rest.strip_suffix(r"\b"))
                .filter(|inner| {
                    inner
                        .chars()
                        .all(|each| each.is_ascii_alphanumeric() || each == '-' || each == '?')
                })
                .map_or(usize::MAX, str::len)
        })
        .max()
        .unwrap_or(0)
});

/// The plugin's replacements whose replacement is written in capitals.
fn acronyms() -> impl Iterator<Item = &'static (&'static str, &'static str, bool)> {
    DEFAULT_REPLACEMENTS
        .iter()
        .filter(|(_, replacement, _)| *replacement == replacement.to_uppercase())
}

/// A compiled pattern, built once.
macro_rules! pattern {
    ($name:ident, $flags:expr, $source:expr) => {
        static $name: LazyLock<Pattern> = LazyLock::new(|| Pattern::new(&$source, $flags));
    };
}

/// A word ends here.
fn bound_end() -> String {
    format!("(?!{WORD})")
}

/// `[\w$]+`.
const IDENT: &str = r"[\w$]+";
/// A name of code.
const CODE_IDENT: &str = r"[$A-Z_a-z][\w$]*";
/// A list marker.
const LIST_MARKER: &str = r"(?:[*+\-]|\d+\.)";
/// `[…]`.
const BRACKET_PROPERTY: &str = r"\[[^\]]+\]";

/// An optional list marker and its space.
fn list_prefix() -> String {
    format!(r"{LIST_MARKER}?\s*")
}

/// `.name`, `?.name` or `?.[…]`.
fn member_call_property() -> String {
    format!(r"(?:(?:\?\.|\.)\s*{IDENT}|\??\.\s*{BRACKET_PROPERTY})")
}

/// An optional `label:` or `const target =`.
fn assignment_prefix() -> String {
    let member_target = format!(
        r"{IDENT}(?:\s*{}|\s*{BRACKET_PROPERTY})+",
        member_call_property()
    );
    let target = format!("(?:{IDENT}|{member_target})");
    format!(r"(?:(?:{IDENT}:|(?:(?:const|let|var)\s+)?{target}\s*=)\s*)?")
}

/// The start of a link's destination.
const LINK_DESTINATION_START: &str =
    r"(?:[A-Za-z][\d+\-.A-Za-z]*:\/\/|www\.|[\u{2D}0-9A-Za-z]+\.|[\w\-.]+\/|\/|#)";
/// A Markdown link's label.
const LINK_LABEL: &str = r"\[[^\n\]]{1,200}\]";
/// A Markdown link's destination.
const LINK_DESTINATION: &str = r"[^\s\)\[]+";
/// A Markdown link's title.
const LINK_TITLE: &str = r#"(?:"[^\n"]*"|'[^\n']*'|\([^\n\)]+\))"#;

pattern!(
    ESLINT_DIRECTIVE,
    "v",
    format!(r"^\s*(?:eslint(?:-env)?|global|exported){}", bound_end())
);
pattern!(URL, "giv", r"\b(?:[a-z][\d+\-.a-z]{0,31}:\/\/|www\.)\S+");
pattern!(
    MIME_TYPE,
    "giv",
    r"\b(?:application|audio|font|image|message|model|multipart|text|video)\/[\da-z][\d+\-.a-z]*\b"
);
pattern!(
    CODE_LINE_START,
    "v",
    format!(
        r"^(?:import|export|const|let|var|type|interface|class|function|return|await|throw){}",
        bound_end()
    )
);
pattern!(
    CONTROL_FLOW_LINE_START,
    "v",
    format!(
        r"^(?:(?:if|for|while|switch|catch)\s*\(|else(?:\s+if\s*\(|\s*(?:\{{|$))|(?:try|do|finally)\s*(?:\{{|$)|(?:case{}[^:]+|default\s*):)",
        bound_end()
    )
);
pattern!(
    BRACKET_MEMBER_LINE,
    "v",
    format!(
        r"^(?:{IDENT}:\s*)?{IDENT}(?:(?:\??\.\s*)?{BRACKET_PROPERTY}(?:\s*\.[\w$]|\s*$)|\s+{BRACKET_PROPERTY}\s*\.[\w$])"
    )
);
pattern!(
    CHAINED_BRACKET_LINE,
    "v",
    format!(
        r"^(?:{IDENT}:\s*)?{CODE_IDENT}\s*(?:\??\.\s*)?{BRACKET_PROPERTY}\s*{BRACKET_PROPERTY}\s*$"
    )
);
pattern!(
    SPACED_BRACKET_CALL_LINE,
    "v",
    format!(
        r"^{}(?:\(\s*)?{}{CODE_IDENT}\s+{BRACKET_PROPERTY}\s*\((?!\s*{LINK_DESTINATION_START})[^\)]*\)\s*\)?\s*;?$",
        list_prefix(),
        assignment_prefix()
    )
);
pattern!(
    MARKDOWN_INLINE_LINK,
    "gv",
    format!(r"(?:^|[\s\(]){LINK_LABEL}\({LINK_DESTINATION}(?:[^\S\n]+{LINK_TITLE})?\)")
);
pattern!(
    MARKDOWN_LINK,
    "gv",
    format!(
        r"(?:^|[\s\(]){LINK_LABEL}(?::[^\S\n]*{LINK_DESTINATION}(?:[^\S\n]+{LINK_TITLE})?|(?:{LINK_LABEL})+|\({LINK_DESTINATION}(?:[^\S\n]+{LINK_TITLE})?\))"
    )
);
pattern!(
    MEMBER_CALL_LINE,
    "v",
    format!(
        r"^{}(?:\(\s*)?{}{IDENT}(?:\s*{}\s*(?:\?\.\s*)?\(|{BRACKET_PROPERTY}\()",
        list_prefix(),
        assignment_prefix(),
        member_call_property()
    )
);
pattern!(
    BARE_CALL_LINE,
    "v",
    format!(
        r"^{}(?:\(\s*)?{}(?:new\s+)?{CODE_IDENT}(?:\(|\s+\(\s*$)",
        list_prefix(),
        assignment_prefix()
    )
);
pattern!(
    SHELL_PROMPT_LINE,
    "v",
    format!(r"^{}\$\s+\S+", list_prefix())
);
pattern!(
    SECONDARY_SHELL_PROMPT_LINE,
    "v",
    format!(
        r"^{}>\s*(?:bun|curl|deno|docker|git|node(?:js)?|npm|npx|pnpm|yarn){}",
        list_prefix(),
        bound_end()
    )
);
pattern!(JSDOC_EXAMPLE, "v", format!("^@example{}", bound_end()));
pattern!(LIST_MARKER_START, "v", format!(r"^{LIST_MARKER}\s*"));
pattern!(
    DOT_MEMBER_AFTER,
    "v",
    r"^(?:\?\.|\.|\s+\.)\s*[\w$]+(?:\s*(?:\[|\.[\w$]|\()|\s*$)"
);
pattern!(
    BRACKET_MEMBER_AFTER,
    "v",
    r"^(?:\?\.\s*)?\[[^\]]+\](?:\s*(?:\[|\.[\w$]|\()|\s*$)"
);
pattern!(
    SPACED_BRACKET_AFTER,
    "v",
    r"^\s+\[[^\]]+\]\s*(?:\[|\.[\w$])"
);
pattern!(
    MEMBER_BEFORE,
    "v",
    r"(?:^|[^\w$])[\w$]+\s*(?:\?\.|\.|\s+\.)\s*$"
);
pattern!(MEMBER_AFTER, "v", r"^\s*(?:$|[\(.\[])");
pattern!(FENCE, "gmv", r"^[\t ]*(?:\*\s?)?(?<fence>`{3,}|~{3,})");
pattern!(MARKUP_TAG, "iv", r"^<\/?[a-z][\w\-:]*(?:[^\S\n]|\/?>)");
pattern!(
    JSDOC_TAG,
    "v",
    r"^\s*(?:\*\s*)?(?<tag>@[\d\p{Letter}][\d\p{Letter}\-]*!?)(?=\s|$)"
);
pattern!(
    JSDOC_INLINE_TAG,
    "giv",
    r"\{\s*(?<tag>@(?:link|linkcode|linkplain|tutorial|inheritdoc))(?=\s|\})"
);
pattern!(
    JSDOC_INLINE_TAG_START,
    "iv",
    r"^\{\s*@(?:link|linkcode|linkplain|tutorial|inheritdoc)(?=\s|\})"
);
pattern!(STRUCTURED_KEY, "v", r"^[\w$\-]+$");
pattern!(STRUCTURED_URL_VALUE, "v", r"^https?:\/\/\S+$");
pattern!(STRUCTURED_PLAIN_VALUE, "v", r"^[\w$\-.\/]+$");
pattern!(SPACED_HASH, "v", r"\s+#");
pattern!(PATH_EXTENSION, "v", r"\.[\p{Letter}\p{Number}_]");

/// A text of characters.
fn string(characters: &[char]) -> String {
    characters.iter().collect()
}

/// Whether a pattern matches a text of characters, from its start.
fn starts(pattern: &Pattern, text: &[char]) -> bool {
    pattern.match_at(text, 0).is_some()
}

/// Whether a pattern matches somewhere in a text of characters.
fn found(pattern: &Pattern, text: &[char]) -> bool {
    pattern.find_at(text, 0).is_some()
}

/// The index of the next line ending from an index, or the text's length.
fn line_end(text: &[char], from: usize) -> usize {
    text[from.min(text.len())..]
        .iter()
        .position(|each| *each == '\n')
        .map_or(text.len(), |at| from + at)
}

/// The index of the last line ending before an index, plus one: the line's start.
fn line_start(text: &[char], index: usize) -> usize {
    text[..index.min(text.len())]
        .iter()
        .rposition(|each| *each == '\n')
        .map_or(0, |at| at + 1)
}

/// Masks a range, its line endings kept.
fn mask_range(characters: &mut [char], start: usize, end: usize) {
    let end = end.min(characters.len());
    for each in &mut characters[start.min(end)..end] {
        if *each != '\n' {
            *each = MASK;
        }
    }
}

/// Masks every match of a pattern.
fn mask_pattern(characters: &mut [char], text: &[char], pattern: &Pattern) {
    for each in pattern.find_all(text) {
        mask_range(characters, each.start, each.end);
    }
}

/// Whether a character is a word character of any script.
fn is_identifier_like(character: Option<char>) -> bool {
    character.is_some_and(|each| each.is_alphanumeric() || each == '_')
}

/// Whether a character is an ASCII letter.
fn is_ascii_letter(character: Option<char>) -> bool {
    character.is_some_and(|each| each.is_ascii_alphabetic())
}

/// The net change of open brackets across a line, outside strings and comments.
fn bracket_depth_delta(text: &[char]) -> i64 {
    let mut delta = 0;
    let mut quote: Option<char> = None;
    let mut index = 0;
    while index < text.len() {
        let character = text[index];
        if let Some(open) = quote {
            if character == '\\' {
                index += 2;
                continue;
            }
            if character == open {
                quote = None;
            }
            index += 1;
            continue;
        }
        if character == '/' && text.get(index + 1) == Some(&'/') {
            break;
        }
        if character == '/' && text.get(index + 1) == Some(&'*') {
            let close = (index + 2..text.len().saturating_sub(1))
                .find(|at| text[*at] == '*' && text[*at + 1] == '/');
            index = close.map_or(text.len(), |at| at + 2);
            continue;
        }
        if "\"'`".contains(character) {
            quote = Some(character);
        } else if "([{".contains(character) {
            delta += 1;
        } else if ")]}".contains(character) {
            delta -= 1;
        }
        index += 1;
    }
    delta
}

/// A line without its `*` margin, trimmed.
fn clean_line(line: &[char]) -> Vec<char> {
    let mut at = 0;
    while at < line.len() && is_space(line[at]) {
        at += 1;
    }
    let rest = if line.get(at) == Some(&'*') {
        let mut after = at + 1;
        if line.get(after).is_some_and(|each| is_space(*each)) {
            after += 1;
        }
        &line[after..]
    } else {
        line
    };
    string(rest).trim_matches(is_space).chars().collect()
}

/// A line with its Markdown inline links removed.
fn without_inline_links(line: &[char]) -> Vec<char> {
    let mut out = Vec::with_capacity(line.len());
    let mut at = 0;
    for each in MARKDOWN_INLINE_LINK.find_all(line) {
        out.extend_from_slice(&line[at..each.start]);
        at = each.end;
    }
    out.extend_from_slice(&line[at..]);
    out
}

/// Whether a text is quoted with one kind of quote it does not hold inside.
fn is_quoted(text: &[char]) -> bool {
    let Some(quote) = text.first().copied() else {
        return false;
    };
    text.len() >= 2
        && (quote == '"' || quote == '\'')
        && text.last() == Some(&quote)
        && !text[1..text.len() - 1].contains(&quote)
}

/// Whether a text is a simple value of a structured line: quoted, a URL, a word or path, or a
/// bracketed value.
fn is_simple_value(text: &[char]) -> bool {
    let mut value: Vec<char> = text.to_vec();
    if value.last() == Some(&',') {
        value.pop();
        value = string(&value).trim_end_matches(is_space).chars().collect();
    }
    if let Some(each) = SPACED_HASH.find_at(&value, 0) {
        value.truncate(each.start);
    }
    let wrapped = |open: char, close: char| {
        value.first() == Some(&open) && value.last() == Some(&close) && value.len() >= 2
    };
    is_quoted(&value)
        || starts(&STRUCTURED_URL_VALUE, &value)
        || starts(&STRUCTURED_PLAIN_VALUE, &value)
        || wrapped('{', '}')
        || wrapped('[', ']')
}

/// A line without its list marker.
fn without_list_marker(line: &[char]) -> Vec<char> {
    LIST_MARKER_START
        .match_at(line, 0)
        .map_or_else(|| line.to_vec(), |each| line[each.end..].to_vec())
}

/// Whether a line reads `key: value`, both simple.
fn is_structured_key_value(line: &[char]) -> bool {
    let line = without_list_marker(line);
    let Some(colon) = line.iter().position(|each| *each == ':') else {
        return false;
    };
    let key: Vec<char> = string(&line[..colon])
        .trim_matches(is_space)
        .chars()
        .collect();
    let value: Vec<char> = string(&line[colon + 1..])
        .trim_matches(is_space)
        .chars()
        .collect();
    (is_quoted(&key) || starts(&STRUCTURED_KEY, &key)) && is_simple_value(&value)
}

/// Whether a line is an object literal, `{a: 1}`.
fn is_object_literal(line: &[char]) -> bool {
    let mut line = without_list_marker(line);
    if line.last() == Some(&',') {
        line.pop();
        line = string(&line).trim_end_matches(is_space).chars().collect();
    }
    line.first() == Some(&'{') && line.last() == Some(&'}') && line.contains(&':')
}

/// Whether a comment line looks like code or a command.
fn is_ignored_line(line: &[char]) -> bool {
    let line = clean_line(line);
    let text = string(&line);
    starts(&CODE_LINE_START, &line)
        || starts(&CONTROL_FLOW_LINE_START, &line)
        || starts(&BRACKET_MEMBER_LINE, &line)
        || starts(&CHAINED_BRACKET_LINE, &line)
        || starts(&SPACED_BRACKET_CALL_LINE, &line)
        || starts(&SHELL_PROMPT_LINE, &line)
        || starts(&SECONDARY_SHELL_PROMPT_LINE, &line)
        || is_structured_key_value(&line)
        || is_object_literal(&line)
        || text.trim_end_matches(is_space).ends_with(';')
        || text.contains("=>")
        || starts(&MEMBER_CALL_LINE, &without_inline_links(&line))
}

/// The depth of brackets a code-like line leaves open, when it starts a call.
fn continuation_opening_depth(line: &[char]) -> i64 {
    let line = without_inline_links(&clean_line(line));
    if starts(&BARE_CALL_LINE, &line) || starts(&MEMBER_CALL_LINE, &line) {
        bracket_depth_delta(&line).max(0)
    } else {
        0
    }
}

/// The open bracket depth after a line: continued, or opened by a call.
fn next_depth(depth: i64, line: &[char]) -> i64 {
    if depth > 0 {
        (depth + bracket_depth_delta(line)).max(0)
    } else {
        continuation_opening_depth(line)
    }
}

/// Masks the lines that look like code, and the lines continuing their brackets.
fn mask_ignored_lines(characters: &mut [char], text: &[char]) {
    let mut start = 0;
    let mut depth = 0;
    while start < text.len() {
        let end = line_end(text, start);
        let line = &text[start..end];
        if characters[start..end].contains(&MASK) {
            depth = 0;
            start = end + 1;
            continue;
        }
        let opening = continuation_opening_depth(line);
        if depth > 0 || is_ignored_line(line) || opening > 0 {
            mask_range(characters, start, end);
            depth = if depth > 0 {
                (depth + bracket_depth_delta(line)).max(0)
            } else {
                opening
            };
        }
        start = end + 1;
    }
}

/// Masks fenced code blocks, an unclosed one to the end.
fn mask_fenced_code_blocks(characters: &mut [char], text: &[char]) {
    let group = FENCE.group("fence").unwrap_or(0);
    let mut active: Option<(char, usize, usize)> = None;
    for each in FENCE.find_all(text) {
        let Some((from, to)) = each.groups[group] else {
            continue;
        };
        let fence = &text[from..to];
        match active {
            None => active = Some((fence[0], fence.len(), each.start)),
            Some((character, size, start)) => {
                if fence[0] == character && fence.len() >= size {
                    mask_range(characters, start, line_end(text, each.start));
                    active = None;
                }
            }
        }
    }
    if let Some((_, _, start)) = active {
        mask_range(characters, start, text.len());
    }
}

/// Masks `@example` sections, up to the next tag.
fn mask_jsdoc_examples(characters: &mut [char], text: &[char]) {
    let mut example: Option<usize> = None;
    let mut start = 0;
    while start < text.len() {
        let end = line_end(text, start);
        let line = clean_line(&text[start..end]);
        let is_tag = line.first() == Some(&'@')
            && line
                .get(1)
                .is_some_and(|each| crate::rules::unicorn::pattern::is_word(*each));
        if example.is_some() && is_tag {
            mask_range(characters, example.unwrap_or(start), start);
            example = None;
        }
        if example.is_none() && starts(&JSDOC_EXAMPLE, &line) {
            example = Some(start);
        }
        start = end + 1;
    }
    if let Some(from) = example {
        mask_range(characters, from, text.len());
    }
}

/// The `JSDoc` tags whose type and name are syntax.
const TYPE_AND_NAME_TAGS: [&str; 15] = [
    "param",
    "arg",
    "argument",
    "property",
    "prop",
    "typedef",
    "callback",
    "member",
    "var",
    "const",
    "constant",
    "class",
    "constructor",
    "module",
    "namespace",
];
/// The tags whose type or value is syntax.
const STRUCTURED_TYPE_TAGS: [&str; 9] = [
    "type",
    "this",
    "extends",
    "augments",
    "implements",
    "satisfies",
    "import",
    "package",
    "enum",
];
/// The tags whose type is syntax.
const TYPE_TAGS: [&str; 8] = [
    "returns",
    "return",
    "throws",
    "exception",
    "yields",
    "yield",
    "private",
    "protected",
];
/// The tag `public`, also typed.
const PUBLIC_TAG: &str = "public";
/// The tags whose name is syntax.
const NAME_TAGS: [&str; 28] = [
    "access",
    "alias",
    "constructs",
    "function",
    "method",
    "event",
    "emits",
    "external",
    "exports",
    "fires",
    "func",
    "global",
    "host",
    "inner",
    "instance",
    "interface",
    "kind",
    "lends",
    "listens",
    "memberof",
    "mixes",
    "mixin",
    "name",
    "requires",
    "see",
    "template",
    "tutorial",
    "variation",
];
/// The tags whose whole line is metadata.
const METADATA_TAGS: [&str; 11] = [
    "author",
    "borrows",
    "copyright",
    "license",
    "version",
    "since",
    "date",
    "created",
    "modified",
    "default",
    "defaultvalue",
];

/// The first index from an index that is no horizontal space.
fn skip_horizontal(text: &[char], mut start: usize, end: usize) -> usize {
    while start < end && is_space(text[start]) && text[start] != '\n' {
        start += 1;
    }
    start
}

/// The first index from an index past spaces, line breaks and the `*` margins after them.
fn skip_template_space(text: &[char], mut start: usize, end: usize) -> usize {
    while start < end {
        start = skip_horizontal(text, start, end);
        if text.get(start) != Some(&'\n') {
            return start;
        }
        start += 1;
        start = skip_horizontal(text, start, end);
        if text.get(start) == Some(&'*') {
            start += 1;
        }
    }
    start
}

/// The end of a value delimited by brackets, quotes skipped.
fn delimited_end(text: &[char], start: usize, end: usize, opening: char) -> Option<usize> {
    let closing = match opening {
        '{' => '}',
        '[' => ']',
        _ => ')',
    };
    let mut depth = 0;
    let mut quote: Option<char> = None;
    let mut index = start;
    while index < end {
        let character = text[index];
        if let Some(open) = quote {
            if character == '\\' {
                index += 2;
                continue;
            }
            if character == open {
                quote = None;
            }
            index += 1;
            continue;
        }
        if "\"'`".contains(character) {
            quote = Some(character);
        } else if character == opening {
            depth += 1;
        } else if character == closing {
            depth -= 1;
            if depth == 0 {
                return Some(index + 1);
            }
        }
        index += 1;
    }
    None
}

/// The end of a tag's value: bracketed, quoted, or up to a space.
fn value_end(text: &[char], start: usize, end: usize) -> Option<usize> {
    let opening = *text.get(start)?;
    if "{[(".contains(opening) {
        return delimited_end(text, start, end, opening);
    }
    if "\"'`".contains(opening) {
        let mut index = start + 1;
        while index < end {
            if text[index] == '\\' {
                index += 2;
                continue;
            }
            if text[index] == opening {
                return Some(index + 1);
            }
            index += 1;
        }
        return None;
    }
    let mut index = start;
    while index < end && !is_space(text[index]) {
        index += 1;
    }
    Some(index)
}

/// Masks a tag's value, unless a `-` starts the description; the index after it.
fn mask_value(characters: &mut [char], text: &[char], start: usize, end: usize) -> usize {
    let start = skip_horizontal(text, start, end);
    if start >= end || text[start] == '-' {
        return start;
    }
    match value_end(text, start, end) {
        Some(value) => {
            mask_range(characters, start, value);
            value
        }
        None => start,
    }
}

/// Masks a tag's `{type}`, not an inline tag; the index after it.
fn mask_type(characters: &mut [char], text: &[char], start: usize, end: usize) -> usize {
    let start = skip_horizontal(text, start, end);
    if text.get(start) != Some(&'{')
        || starts(&JSDOC_INLINE_TAG_START, &text[start..end.max(start)])
    {
        return start;
    }
    match delimited_end(text, start, end, '{') {
        Some(type_end) => {
            mask_range(characters, start, type_end);
            type_end
        }
        None => start,
    }
}

/// Masks a type, or else a value.
fn mask_type_or_value(characters: &mut [char], text: &[char], start: usize, end: usize) {
    let value_start = skip_horizontal(text, start, end);
    if mask_type(characters, text, value_start, end) == value_start {
        mask_value(characters, text, value_start, end);
    }
}

/// Masks a type and then a name.
fn mask_type_and_name(characters: &mut [char], text: &[char], start: usize, end: usize) {
    let mut value_start = skip_horizontal(text, start, end);
    if text.get(value_start) == Some(&'{') {
        let type_end = mask_type(characters, text, value_start, end);
        if type_end == value_start {
            return;
        }
        value_start = skip_horizontal(text, type_end, end);
    }
    mask_value(characters, text, value_start, end);
}

/// Masks a `@template`'s type and its list of names.
fn mask_template(characters: &mut [char], text: &[char], start: usize, end: usize) {
    let mut name_start = skip_horizontal(text, start, end);
    if text.get(name_start) == Some(&'{') {
        let type_end = mask_type(characters, text, name_start, end);
        if type_end == name_start {
            return;
        }
        name_start = skip_template_space(text, type_end, end);
    }
    let mut value_end_at = mask_value(characters, text, name_start, end);
    if value_end_at > 0 && text.get(value_end_at - 1) == Some(&',') {
        value_end_at -= 1;
    }
    while value_end_at < end {
        let comma = skip_template_space(text, value_end_at, end);
        if text.get(comma) != Some(&',') {
            return;
        }
        let next = skip_template_space(text, comma + 1, end);
        value_end_at = mask_value(characters, text, next, end);
    }
}

/// Masks a `@see`'s target when nothing but a description follows it.
fn mask_see(characters: &mut [char], text: &[char], start: usize, end: usize) {
    let value_start = skip_horizontal(text, start, end);
    let Some(value) = value_end(text, value_start, end) else {
        return;
    };
    if starts(&JSDOC_INLINE_TAG_START, &text[value_start..value]) {
        return;
    }
    let remainder = skip_horizontal(text, value, end);
    if remainder < end && text[remainder] != '-' {
        return;
    }
    mask_range(characters, value_start, value);
}

/// Masks the syntax of the `JSDoc` block tags: their names and, by tag, types, names and values.
fn mask_jsdoc_tags(characters: &mut [char], text: &[char]) {
    let group = JSDOC_TAG.group("tag").unwrap_or(0);
    let mut tags: Vec<(String, usize, usize, usize, usize, usize)> = Vec::new();
    let mut start = 0;
    while start < text.len() {
        let end = line_end(text, start);
        if let Some(each) = JSDOC_TAG.match_at(&text[start..end], 0)
            && let Some((from, to)) = each.groups[group]
        {
            let name = string(&text[start + from + 1..start + to])
                .to_lowercase()
                .trim_end_matches('!')
                .to_owned();
            tags.push((name, start + from, start + to, start + each.end, start, end));
        }
        start = end + 1;
    }
    for (index, (name, tag_start, tag_end, content, _, line_end_at)) in tags.iter().enumerate() {
        let section_end = tags.get(index + 1).map_or(text.len(), |next| next.4);
        mask_range(characters, *tag_start, *tag_end);
        let name = name.as_str();
        if TYPE_AND_NAME_TAGS.contains(&name) {
            mask_type_and_name(characters, text, *content, section_end);
        } else if STRUCTURED_TYPE_TAGS.contains(&name) {
            mask_type_or_value(characters, text, *content, section_end);
        } else if TYPE_TAGS.contains(&name) || name == PUBLIC_TAG {
            mask_type(characters, text, *content, section_end);
        } else if name == "see" {
            mask_see(characters, text, *content, *line_end_at);
        } else if NAME_TAGS.contains(&name) {
            if name == "template" {
                mask_template(characters, text, *content, section_end);
            } else {
                mask_value(characters, text, *content, *line_end_at);
            }
        } else if METADATA_TAGS.contains(&name) {
            mask_range(characters, *content, *line_end_at);
        }
    }
}

/// Masks the inline tags' names and targets, `{@link target}`.
fn mask_jsdoc_inline_tags(characters: &mut [char], text: &[char]) {
    let group = JSDOC_INLINE_TAG.group("tag").unwrap_or(0);
    let mut from = 0;
    while let Some(each) = JSDOC_INLINE_TAG.find_at(text, from) {
        let Some((tag_start, tag_end)) = each.groups[group] else {
            break;
        };
        mask_range(characters, tag_start, tag_end);
        let value_start = each.end;
        let limit = line_end(text, each.start);
        let (mut quote, mut target_end, mut has_target): (Option<char>, Option<usize>, bool) =
            (None, None, false);
        let mut close = limit;
        let mut index = value_start;
        while index < limit {
            let character = text[index];
            if character == '\\' {
                index += 2;
                continue;
            }
            if let Some(open) = quote {
                if character == open {
                    quote = None;
                }
            } else if "\"'`".contains(character) {
                quote = Some(character);
                has_target = true;
            } else if character == '|' && target_end.is_none() {
                target_end = Some(index);
            } else if is_space(character) {
                if has_target && target_end.is_none() {
                    target_end = Some(index);
                }
            } else if character == '}' {
                close = index;
                break;
            } else {
                has_target = true;
            }
            index += 1;
        }
        let target = target_end.unwrap_or(close);
        mask_range(characters, value_start, target);
        from = close + 1;
    }
}

/// The size of a run of backticks at an index.
fn backtick_run(text: &[char], index: usize) -> usize {
    text[index..]
        .iter()
        .take_while(|each| **each == '`')
        .count()
}

/// Masks inline code and quoted strings; an unclosed one to the end of its line.
fn mask_inline_code_and_quotes(characters: &mut [char], text: &[char]) {
    let mut index = 0;
    while index < text.len() {
        if characters[index] == MASK {
            index += 1;
            continue;
        }
        let character = text[index];
        let end = if character == '`' {
            let size = backtick_run(text, index);
            let mut found = None;
            let mut at = index + size;
            while at < text.len() {
                if characters[at] == MASK || text[at] != '`' {
                    at += 1;
                    continue;
                }
                let closing = backtick_run(text, at);
                if closing == size {
                    found = Some(at + closing);
                    break;
                }
                at += closing;
            }
            found
        } else if character == '"'
            || (character == '\'' && !is_identifier_like(index.checked_sub(1).map(|at| text[at])))
        {
            let mut found = None;
            let mut at = index + 1;
            while at < text.len() {
                if characters[at] == MASK {
                    at += 1;
                    continue;
                }
                if text[at] == '\\' {
                    at += 2;
                    continue;
                }
                if text[at] == character {
                    found = Some(at + 1);
                    break;
                }
                at += 1;
            }
            found
        } else {
            index += 1;
            continue;
        };
        let end = end.unwrap_or_else(|| line_end(text, index));
        mask_range(characters, index, end);
        index = end.max(index + 1);
    }
}

/// Whether a token is a bare domain, `example.com`, not a `.js` file.
fn is_bare_domain(text: &str) -> bool {
    if text.chars().count() > 253 || !text.contains('.') {
        return false;
    }
    let lower = text.to_lowercase();
    let parts: Vec<&str> = lower.split('.').collect();
    let Some((top, rest)) = parts.split_last() else {
        return false;
    };
    *top != "js"
        && (2..=63).contains(&top.len())
        && top.chars().all(|each| each.is_ascii_lowercase())
        && rest.iter().all(|part| {
            (1..=63).contains(&part.len())
                && part
                    .chars()
                    .all(|each| each == '-' || each.is_ascii_digit() || each.is_ascii_lowercase())
        })
}

/// Masks bare domains and oversized dotted tokens.
fn mask_bare_domains(characters: &mut [char], text: &[char]) {
    let mut start = 0;
    while start < text.len() {
        while start < text.len() && is_space(text[start]) {
            start += 1;
        }
        let mut end = start;
        while end < text.len() && !is_space(text[end]) {
            end += 1;
        }
        let (mut from, mut to) = (start, end);
        while from < to && "([{<".contains(text[from]) {
            from += 1;
        }
        while to > from && ".,;:!?)]}>".contains(text[to - 1]) {
            to -= 1;
        }
        let token = string(&text[from..to]);
        let oversized = token.chars().count() > 253 && token.contains('.');
        if is_bare_domain(&token) || oversized {
            mask_range(characters, from, to);
        }
        start = end + 1;
    }
}

/// Whether a two-part `a/b` text names two acronyms, prose rather than a path.
fn is_slash_pair_prose(text: &str) -> bool {
    let trimmed = text.strip_suffix(['!', '.', '?']).unwrap_or(text);
    let parts: Vec<&str> = trimmed.split('/').collect();
    parts.len() == 2
        && parts
            .iter()
            .all(|part| part.chars().count() <= *MAX_ACRONYM_LENGTH && ACRONYM_TERM.test(part))
}

/// Whether a token is a package specifier, `@scope/name` or `name/path`.
fn is_package_specifier(text: &str) -> bool {
    if !text.contains('/') || text.contains("://") {
        return false;
    }
    let suffix_start = usize::from(text.starts_with('@'));
    let base_end = text
        .char_indices()
        .skip(suffix_start)
        .find(|(_, each)| "#?@".contains(*each))
        .map_or(text.len(), |(at, _)| at);
    let parts: Vec<&str> = text[..base_end].split('/').collect();
    if parts.len() < 2 {
        return false;
    }
    let plain = |part: &str| {
        !part.is_empty()
            && part.chars().all(|each| {
                crate::rules::unicorn::pattern::is_word(each) || each == '-' || each == '.'
            })
    };
    let shaped = match parts[0].strip_prefix('@') {
        Some(scope) => plain(scope) && parts[1..].iter().all(|part| plain(part)),
        None => parts.iter().all(|part| plain(part)),
    };
    shaped && !is_slash_pair_prose(text)
}

/// Masks package specifiers and oversized slashed tokens.
fn mask_package_specifiers(characters: &mut [char], text: &[char]) {
    let terminator = |each: char| "\"'`()[]{}<>,".contains(each) || is_space(each);
    let mut start = 0;
    while start < text.len() {
        while start < text.len() && terminator(text[start]) {
            start += 1;
        }
        let mut end = start;
        while end < text.len() && !terminator(text[end]) {
            end += 1;
        }
        let token = string(&text[start..end]);
        let oversized = token.chars().count() > 253 && token.contains('/');
        if is_package_specifier(&token) || oversized {
            mask_range(characters, start, end);
        }
        start = end + 1;
    }
}

/// Masks Markdown links whose `[` is not masked already.
fn mask_markdown_links(characters: &mut [char], text: &[char]) {
    for each in MARKDOWN_LINK.find_all(text) {
        let bracket = (each.start..text.len()).find(|at| text[*at] == '[');
        if bracket.is_some_and(|at| characters[at] == MASK) {
            continue;
        }
        mask_range(characters, each.start, each.end);
    }
}

/// Masks HTML and XML tags written on one line.
fn mask_markup_tags(characters: &mut [char], text: &[char]) {
    let mut start = 0;
    while let Some(offset) = text[start.min(text.len())..]
        .iter()
        .position(|each| *each == '<')
    {
        let tag_start = start + offset;
        if characters[tag_start] == MASK {
            start = tag_start + 1;
            continue;
        }
        let next = text.get(tag_start + 1).copied();
        let opens = is_ascii_letter(next)
            || (next == Some('/') && is_ascii_letter(text.get(tag_start + 2).copied()));
        if !opens {
            start = tag_start + 1;
            continue;
        }
        let limit = line_end(text, tag_start);
        let Some(tag_end) = (tag_start + 1..limit).find(|at| text[*at] == '>') else {
            start = limit + 1;
            continue;
        };
        if starts(&MARKUP_TAG, &text[tag_start..=tag_end]) {
            mask_range(characters, tag_start, tag_end + 1);
        }
        start = tag_end + 1;
    }
}

/// A comment's value with every region the rule does not read masked.
fn searchable(value: &[char], is_jsdoc: bool) -> Vec<char> {
    let mut characters = value.to_vec();
    mask_fenced_code_blocks(&mut characters, value);
    mask_jsdoc_examples(&mut characters, value);
    if is_jsdoc {
        mask_jsdoc_tags(&mut characters, value);
        mask_jsdoc_inline_tags(&mut characters, value);
    }
    mask_ignored_lines(&mut characters, value);
    mask_inline_code_and_quotes(&mut characters, value);
    mask_pattern(&mut characters, value, &URL);
    mask_bare_domains(&mut characters, value);
    mask_pattern(&mut characters, value, &MIME_TYPE);
    mask_package_specifiers(&mut characters, value);
    mask_markdown_links(&mut characters, value);
    mask_markup_tags(&mut characters, value);
    characters
}

/// The path text after an index, up to a quote, a bracket or a space.
fn path_after(value: &[char], index: usize) -> Vec<char> {
    value[index.min(value.len())..]
        .iter()
        .take_while(|each| !("\"'`<>".contains(**each) || is_space(**each)))
        .copied()
        .collect()
}

/// Whether a match is part of a path or a file name.
fn is_path_like(value: &[char], start: usize, end: usize) -> bool {
    let at = |index: Option<usize>| index.and_then(|each| value.get(each).copied());
    let previous = at(start.checked_sub(1));
    let next = at(Some(end));
    let after_next = at(Some(end + 1));
    let matched = &value[start..end];
    let before_extension = next == Some('.') && is_identifier_like(after_next);
    let before_path_with_extension =
        next == Some('/') && found(&PATH_EXTENSION, &path_after(value, end + 1));
    (matched.contains(&'.') && previous == Some('/'))
        || (previous == Some('/') && (next == Some('/') || before_extension))
        || before_path_with_extension
        || matches!(previous, Some('.' | '_' | '\\'))
        || matches!(next, Some('_' | '\\'))
        || before_extension
}

/// Whether a match is read as a property or an object of a member access.
fn is_property_access(value: &[char], start: usize, end: usize) -> bool {
    let at = |index: Option<usize>| index.and_then(|each| value.get(each).copied());
    let previous = at(start.checked_sub(1));
    let before_previous = at(start.checked_sub(2));
    let next = at(Some(end));
    let line_from = line_start(value, start);
    let line_to = line_end(value, start);
    let before = &value[line_from..start];
    let after = &value[end.min(line_to)..line_to];
    let matched = &value[start..end];
    let property =
        !matched.contains(&'.') && found(&MEMBER_BEFORE, before) && starts(&MEMBER_AFTER, after);
    let object = !matched.contains(&'.')
        && (starts(&DOT_MEMBER_AFTER, after)
            || starts(&BRACKET_MEMBER_AFTER, after)
            || starts(&SPACED_BRACKET_AFTER, after));
    let slice = |from: usize, to: usize| {
        value
            .get(from..to.min(value.len()))
            .map(string)
            .unwrap_or_default()
    };
    start >= 2 && slice(start - 2, start) == "?."
        || slice(end, end + 2) == "?."
        || (previous == Some('[')
            && (is_identifier_like(before_previous)
                || (start >= 3 && slice(start - 3, start) == "?.[")))
        || next == Some('[')
        || property
        || object
}

/// Whether a match is skipped: a path, a member access, or joined by `-` or followed by `(`.
fn should_skip(value: &[char], start: usize, end: usize) -> bool {
    is_path_like(value, start, end)
        || is_property_access(value, start, end)
        || (start > 0 && value[start - 1] == '-')
        || value.get(end) == Some(&'-')
        || value.get(end) == Some(&'(')
}

/// The first replacement in a comment: its range in the value, what it matched and what it
/// becomes.
fn first_problem(value: &[char], is_jsdoc: bool) -> Option<(usize, usize, String, String)> {
    let masked = searchable(value, is_jsdoc);
    let text = string(&masked);
    let folds = text.contains('\u{17F}') || text.contains('\u{212A}');
    let words: Vec<String> = text
        .split(|each: char| !each.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .flat_map(|word| [word.to_owned(), word.to_ascii_lowercase()])
        .collect();
    let mut best: Option<(usize, usize, String, String)> = None;
    for replacement in REPLACEMENTS.iter() {
        let present = folds
            || replacement
                .required
                .as_ref()
                .is_none_or(|word| words.iter().any(|each| each.starts_with(word.as_str())));
        if !present {
            continue;
        }
        let mut from = 0;
        while let Some(each) = replacement.pattern.find_at(&masked, from) {
            from = if each.end == each.start {
                each.end + 1
            } else {
                each.end
            };
            if each.end == each.start {
                continue;
            }
            let matched = string(&masked[each.start..each.end]);
            if matched == replacement.text || matched.contains(MASK) {
                continue;
            }
            if should_skip(value, each.start, each.end) {
                continue;
            }
            if best
                .as_ref()
                .is_some_and(|(start, ..)| each.start >= *start)
            {
                break;
            }
            best = Some((each.start, each.end, matched, replacement.text.clone()));
            break;
        }
    }
    best
}

/// Whether a comment is an `ESLint` directive: a disable or enable comment, or a configuration.
fn is_eslint_directive(source: &Source, comment: &Comment) -> bool {
    let value = source.comment_value(comment);
    let trimmed = value.trim_start();
    let directive = |name: &str| {
        trimmed
            .strip_prefix(name)
            .is_some_and(|rest| rest.is_empty() || rest.starts_with(char::is_whitespace))
    };
    let disable = directive("eslint-disable-line") || directive("eslint-disable-next-line");
    if comment.is_line() {
        return disable;
    }
    disable
        || directive("eslint-disable")
        || directive("eslint-enable")
        || ESLINT_DIRECTIVE.test(value)
}

impl Rule for CommentContent {
    fn name(&self) -> &'static str {
        "unicorn/comment-content"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let mut depth = 0;
        let mut previous_line_end: Option<u32> = None;
        for comment in &source.program.comments {
            if is_eslint_directive(source, comment) {
                depth = 0;
                previous_line_end = None;
                continue;
            }
            let between = previous_line_end
                .map(|end| &source.text[end as usize..comment.span.start as usize]);
            let adjacent = comment.is_line()
                && between.is_some_and(|text| {
                    text.matches('\n').count() == 1 && text.chars().all(is_space)
                });
            if !adjacent {
                depth = 0;
            }
            let value: Vec<char> = source.comment_value(comment).chars().collect();
            if depth <= 0 {
                let is_jsdoc = comment.is_block()
                    && source.text[comment.span.start as usize..].starts_with("/**")
                    && !source.text[comment.span.start as usize..].starts_with("/***");
                if let Some((start, end, matched, replacement)) = first_problem(&value, is_jsdoc) {
                    let value_start = comment.content_span().start as usize;
                    let offset = |index: usize| {
                        u32::try_from(
                            value_start
                                + value[..index]
                                    .iter()
                                    .map(|each| each.len_utf8())
                                    .sum::<usize>(),
                        )
                        .unwrap_or(0)
                    };
                    findings.push(source.fixed(
                        self.name(),
                        offset(start),
                        format!("Prefer `{replacement}` over `{matched}`."),
                        Fix {
                            start: offset(start),
                            end: offset(end),
                            text: replacement,
                        },
                    ));
                }
            }
            if comment.is_line() {
                depth = next_depth(depth, &value);
                previous_line_end = Some(comment.span.end);
            } else {
                depth = 0;
                previous_line_end = None;
            }
        }
    }
}

/// The literal word a pattern without alternation starts with after `\b`, its last letter
/// dropped when a quantifier follows it (`getRequiredText`): a text without it cannot match.
fn required_text(pattern: &str) -> Option<String> {
    if pattern.contains('|') {
        return None;
    }
    let rest = pattern.strip_prefix(r"\b")?;
    let word: String = rest
        .chars()
        .take_while(char::is_ascii_alphanumeric)
        .collect();
    if word.is_empty() {
        return None;
    }
    let quantified = rest[word.len()..].starts_with(['*', '?', '{']);
    Some(if quantified {
        word[..word.len() - 1].to_owned()
    } else {
        word
    })
}

#[cfg(test)]
mod tests {
    use super::CommentContent;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&CommentContent, "playground/a.ts", code))
    }

    /// The code once fixed.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&CommentContent, "playground/a.ts", code))
    }

    #[test]
    fn fixes_the_documented_comments() {
        assert_eq!(
            check("// nodejs uses javascript."),
            ["Prefer `Node.js` over `nodejs`."]
        );
        assert_eq!(
            fix("// nodejs uses javascript."),
            "// Node.js uses javascript."
        );
        assert_eq!(
            check("// See the github issue."),
            ["Prefer `GitHub` over `github`."]
        );
        assert_eq!(
            check("// The application stores png files."),
            ["Prefer `PNG` over `png`."]
        );
        assert_eq!(
            check("/* Parse the json. */"),
            ["Prefer `JSON` over `json`."]
        );
        assert_eq!(
            check("/** Draws an svg path. */"),
            ["Prefer `SVG` over `svg`."]
        );
    }

    #[test]
    fn leaves_code_links_paths_and_tag_syntax() {
        for code in [
            "// Node.js uses JavaScript.",
            "// const json = parse(text);",
            "// See https://github.com/a/b.",
            "// Reads `json` from disk.",
            "// Reads \"json\" from disk.",
            "// Reads package.json and src/svg/index.ts.",
            "// Uses github.io pages.",
            "// Uses foo.json.",
            "// Uses @scope/json here.",
            "/**\n * @param json - the text\n */",
            "/**\n * @see svg\n */",
            "// eslint-disable-next-line json",
            "// The json-schema file.",
            "// Calls json(x).",
            "// Reads object.json now.",
            "// <svg> is the root.",
            "/*\n```\nsvg\n```\n*/",
            "// The application/json type.",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }

    #[test]
    fn keeps_application_as_the_project_option_means() {
        assert_eq!(check("// The application runs."), NONE);
        assert_eq!(check("// See application/json."), NONE);
        assert_eq!(check("// The applications run."), NONE);
    }

    #[test]
    fn reports_one_replacement_per_comment_the_first() {
        assert_eq!(check("// json and svg"), ["Prefer `JSON` over `json`."]);
        assert_eq!(check("// svg and json"), ["Prefer `SVG` over `svg`."]);
    }
    /// Comment shapes of every kind the masks read, the places of their findings those `ESLint` gives.
    const SHAPES: &str = r#"// foo(
//   json,
// )
// bar(a, '(', "x", `y`, /* ( */ b) // c
// json after bar
// [link](http://x) json.parse(x)
// key: 'json',
// key: json # note
// key: [json]
// { json: 1 },
// - { json: 2 }
/*
~~~
json
*/
/**
 * @example
 * json
 * @returns the json
 */
/**
 * @template {T} json, K
 * @template T,
 * json
 * @see json - the json
 * @see {@link json}
 * @see json is here
 * @type {json}
 * @returns {json} the svg
 * @param {string} json the html
 * @param [json='a'] the css
 * @param "json" the dom
 * @author json
 * @kind json
 * @public {json}
 * @throws json
 * @param {unclosed json
 */
/** Read {@link json|label} and {@link 'json' text} and {@link json} and {@linkcode a\b json}. */
// it's 'json' and "json" and ``json`` and `json
// json.io and (json.com). and a.js and json.example.co.uk
// @scope/json and json/svg and svg/json! and a/b#c and json/file.ts
// [json] [ref]: http://x and [json](x) and [a][json]
// <svg json> and </json> and a < json > b
// a?.json and json?.x and x[json] and json[0] and obj.json( and x ?.[json] and y.json
/* global json */
// eslint-disable-line json
/* eslint-enable json */
// foo(
/* json */
// json
// application/json and image/png and text/x.json
// $ npm json
// > npm json
// if (json) {
// case json:
// default:
// else {
// a[b].json
// x[json][0]
// f [x] ( json )
// a = b.json(
// new Json(
// html, json; svg
// json => svg"#;

    #[test]
    fn masks_code_tags_links_and_markup_as_eslint_does() {
        let places: Vec<(usize, usize)> = lint(&CommentContent, "playground/a.ts", SHAPES)
            .iter()
            .map(|finding| (finding.line, finding.column))
            .collect();
        assert_eq!(
            places,
            [
                (5, 4),
                (19, 17),
                (25, 20),
                (39, 89),
                (42, 20),
                (43, 5),
                (44, 35),
                (50, 4),
                (51, 4)
            ]
        );
    }
}

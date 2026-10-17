//! `unicorn/prefer-https`: a URL of the public web anywhere in a file — string, template,
//! comment — uses `https:` (`eslint-plugin-unicorn` 78.0.0). A host without a public top-level
//! domain, `localhost`, an IP or a reserved name, a known XML namespace or vocabulary, and an
//! `xmlns` value are left alone. Autofixed, as in `ESLint`.

use std::sync::LazyLock;

use super::pattern::Pattern;
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferHttps;

/// A URL of the web: its scheme, its authority, then its path (the plugin's `URL_PATTERN`).
static URL: LazyLock<Pattern> = LazyLock::new(|| {
    Pattern::new(
        r##"(?<![\w+\-.])(?<protocol>https?):\/\/(?<authority>[^\s!"#'(),/;<>?\\\]`{}]+)[^\s!"'(),;<>\\\]`{}]*"##,
        "giu",
    )
});

/// The identifiers defined with `http:` that are no requests.
const IGNORED_IDENTIFIER_URIS: [&str; 29] = [
    "http://www.w3.org/2000/svg",
    "http://www.w3.org/1999/xhtml",
    "http://www.w3.org/1999/xlink",
    "http://www.w3.org/2000/xmlns/",
    "http://www.w3.org/XML/1998/namespace",
    "http://www.w3.org/1998/Math/MathML",
    "http://www.w3.org/2001/XInclude",
    "http://www.w3.org/2005/07/scxml",
    "http://www.w3.org/1999/XSL/Transform",
    "http://www.w3.org/1999/XSL/Format",
    "http://www.w3.org/2001/XMLSchema",
    "http://www.w3.org/2001/XMLSchema-instance",
    "http://www.w3.org/2005/xpath-functions",
    "http://www.w3.org/2005/xqt-errors",
    "http://www.w3.org/TR/2001/REC-xml-c14n-20010315",
    "http://www.w3.org/TR/2001/REC-xml-c14n-20010315#WithComments",
    "http://www.w3.org/TR/1999/REC-xpath-19991116",
    "http://www.w3.org/TR/1999/REC-xslt-19991116",
    "http://schemas.xmlsoap.org/soap/envelope/",
    "http://schemas.xmlsoap.org/soap/encoding/",
    "http://schemas.xmlsoap.org/wsdl/",
    "http://www.w3.org/2003/05/soap-envelope",
    "http://www.w3.org/2003/05/soap-encoding",
    "http://www.w3.org/2005/Atom",
    "http://purl.org/rss/1.0/",
    "http://purl.org/dc/elements/1.1/",
    "http://purl.org/dc/terms/",
    "http://schemas.microsoft.com/developer/msbuild/2003",
    "http://www.sitemaps.org/schemas/sitemap/0.9",
];

/// The families of identifiers defined with `http:`, as the plugin writes them.
static IGNORED_URI_PATTERNS: LazyLock<Vec<Pattern>> = LazyLock::new(|| {
    [
        r"^http:\/\/www\.w3\.org\/\d{4}\/(?:\d{2}\/)?(?:xmldsig(?:-more|\d+)?|xmlenc\d*|xml-exc-c14n|decrypt)#",
        r"^http:\/\/www\.w3\.org\/\d{4}\/(?:\d{2}\/)?(?:xmldsig-filter\d+|xml-c14n\d+)(?:#.*)?$",
        r"^http:\/\/www\.w3\.org\/(?:1999\/02\/22-rdf-syntax-ns|2000\/01\/rdf-schema|2001\/XMLSchema|2002\/07\/owl|2004\/02\/skos\/core|2005\/xqt-errors|ns\/(?:dcat|prov|shacl))#.*$",
        r"^http:\/\/www\.w3\.org\/2005\/xpath-functions(?:#.*|\/(?:array|map|math)(?:#.*)?)$",
        r"^http:\/\/purl\.org\/(?:rss\/1\.0|dc\/(?:elements\/1\.1|terms))\/[\w\-]+$",
        r"^http:\/\/schemas\.android\.com\/(?:apk\/(?:res\/(?:android|[A-Za-z]\w*(?:\.[A-Za-z]\w*)+)|res-auto)|aapt|tools)$",
        r"^http:\/\/maven\.apache\.org\/(?:EXTENSIONS|METADATA|POM|SETTINGS|TOOLCHAINS)\/\d+(?:\.\d+)+$",
        r"^http:\/\/www\.springframework\.org\/schema\/[\w\-]+(?:\/[\w\-]+)*$",
    ]
    .iter()
    .map(|source| Pattern::new(source, "v"))
    .collect()
});

/// An `xmlns` attribute just before a value.
static XMLNS: LazyLock<Pattern> =
    LazyLock::new(|| Pattern::new(r#"(?<![\w#$\-.:])xmlns(?::[\w\-.]+)?\s*=\s*["']?$"#, "i"));

/// The top-level names that never resolve on the public internet.
const NON_PUBLIC_SUFFIXES: [&str; 8] = [
    "example",
    "internal",
    "invalid",
    "local",
    "localdomain",
    "localhost",
    "onion",
    "test",
];

/// The code points no host name may hold, `URL` refusing them.
const FORBIDDEN_HOST: &str = "\0\t\n\r #/:<>?@[\\]^|%";

/// The host name of an authority, as `new URL()` reads it: lower case, without credentials nor
/// port; none when `URL` would refuse it.
fn host_name(authority: &str) -> Option<String> {
    let host_and_port = authority
        .rsplit_once('@')
        .map_or(authority, |(_, rest)| rest);
    if host_and_port.starts_with('[') {
        let close = host_and_port.find(']')?;
        return Some(host_and_port[..=close].to_lowercase());
    }
    let (host, port) = host_and_port.split_once(':').unwrap_or((host_and_port, ""));
    if !port.chars().all(|each| each.is_ascii_digit())
        || port.parse::<u32>().is_ok_and(|each| each > 65_535)
    {
        return None;
    }
    if host.is_empty()
        || host
            .chars()
            .any(|each| FORBIDDEN_HOST.contains(each) || each.is_control())
    {
        return None;
    }
    Some(host.to_lowercase())
}

/// Whether a host name ends with a public top-level name: one with a letter, not reserved.
fn has_public_tld(host: &str) -> bool {
    let trimmed = host.trim_end_matches('.');
    let Some((_, top)) = trimmed.rsplit_once('.') else {
        return false;
    };
    top.chars().any(|each| each.is_ascii_alphabetic()) && !NON_PUBLIC_SUFFIXES.contains(&top)
}

impl Rule for PreferHttps {
    fn name(&self) -> &'static str {
        "unicorn/prefer-https"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        if !source.text.contains("http") && !source.text.to_lowercase().contains("http") {
            return;
        }
        let text: Vec<char> = source.text.chars().collect();
        let protocol = URL.group("protocol").unwrap_or(0);
        let authority_group = URL.group("authority").unwrap_or(1);
        let mut offsets: Vec<u32> = Vec::with_capacity(text.len() + 1);
        let mut at = 0_u32;
        for each in &text {
            offsets.push(at);
            at += u32::try_from(each.len_utf8()).unwrap_or(1);
        }
        offsets.push(at);
        let mut next = 0;
        let candidates = (0..text.len().saturating_sub(3)).filter(|at| {
            text[*at..*at + 4]
                .iter()
                .zip("http".chars())
                .all(|(each, wanted)| each.to_ascii_lowercase() == wanted)
        });
        for candidate in candidates {
            if candidate < next {
                continue;
            }
            let Some(each) = URL.match_at(&text, candidate) else {
                continue;
            };
            next = each.end;
            let Some((protocol_start, protocol_end)) = each.groups[protocol] else {
                continue;
            };
            if text[protocol_start..protocol_end]
                .iter()
                .collect::<String>()
                != "http"
            {
                continue;
            }
            let url: String = text[each.start..each.end].iter().collect();
            let Some((authority_start, authority_end)) = each.groups[authority_group] else {
                continue;
            };
            let authority: String = text[authority_start..authority_end].iter().collect();
            let preceding: String = text[each.start.saturating_sub(30)..each.start]
                .iter()
                .collect();
            let ignored = IGNORED_IDENTIFIER_URIS.contains(&url.as_str())
                || IGNORED_URI_PATTERNS
                    .iter()
                    .any(|pattern| pattern.test(&url))
                || XMLNS.test(&preceding)
                || !host_name(&authority).is_some_and(|host| has_public_tld(&host));
            if ignored {
                continue;
            }
            let start = offsets[each.start];
            findings.push(source.fixed(
                self.name(),
                start,
                "Prefer HTTPS over HTTP.".to_owned(),
                Fix {
                    start,
                    end: start + 4,
                    text: "https".to_owned(),
                },
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferHttps;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferHttps, "playground/a.ts", code))
    }

    /// The code once fixed.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&PreferHttps, "playground/a.ts", code))
    }

    #[test]
    fn fixes_the_documented_urls() {
        assert_eq!(
            check("const url = 'http://sindresorhus.com';"),
            ["Prefer HTTPS over HTTP."]
        );
        assert_eq!(
            fix("const url = 'http://sindresorhus.com';"),
            "const url = 'https://sindresorhus.com';"
        );
        assert_eq!(
            fix("// See http://sindresorhus.com"),
            "// See https://sindresorhus.com"
        );
        assert_eq!(
            fix("const a = `x http://user@a.b.co:80/p?q=1 y`;"),
            "const a = `x https://user@a.b.co:80/p?q=1 y`;"
        );
    }

    #[test]
    fn accepts_local_reserved_and_identifier_urls() {
        for code in [
            "const url = 'https://sindresorhus.com';",
            "const a = 'http://localhost:5173';",
            "const a = 'http://example';",
            "const a = 'http://127.0.0.1';",
            "const a = 'http://site.test/a';",
            "const a = 'http://www.w3.org/2000/svg';",
            "const a = 'http://www.w3.org/2000/09/xmldsig#rsa';",
            "const a = '<svg xmlns=\"http://other.org/ns\">';",
            "const a = 'HTTP://SITE.COM';",
            "const a = 'xhttp://site.com';",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}

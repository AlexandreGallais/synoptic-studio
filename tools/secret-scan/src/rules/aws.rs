//! `@secretlint/secretlint-rule-aws` of `@secretlint/secretlint-rule-preset-recommend` 13.0.7: a
//! secret access key assigned to a name such as `AWS_SECRET_ACCESS_KEY`, and, with the option
//! `enableIDScanRule`, access key IDs and account IDs. The two example values of AWS's own
//! documentation are never reported. The secret key's range starts at the match, the name
//! included, and spans the key's length, as the rule gives it.

use std::sync::LazyLock;

use crate::js::{BOUNDARY, SPACE};
use crate::pattern::Pattern;
use crate::source::{Finding, Source};

use super::{Options, finding};

/// The rule's id.
const RULE: &str = "@secretlint/secretlint-rule-aws";

/// The example access key ID of AWS's documentation, never reported.
const EXAMPLE_KEY_ID: &str = "AKIAIOSFODNN7EXAMPLE";

/// The example secret access key of AWS's documentation, never reported.
const EXAMPLE_SECRET: &str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";

/// An access key ID: a known prefix and 16 capitals or digits.
static ACCESS_KEY_ID: LazyLock<Pattern> = LazyLock::new(|| {
    Pattern::plain(&format!(
        "{BOUNDARY}(A3T[A-Z0-9]|AKIA|AGPA|AIDA|AROA|AIPA|ANPA|ANVA|ASIA)[A-Z0-9]{{16}}{BOUNDARY}"
    ))
});

/// A secret access key, 40 characters, after `AWS_SECRET_ACCESS_KEY` and `=`, `:` or `=>`.
static SECRET_ACCESS_KEY: LazyLock<Pattern> = LazyLock::new(|| {
    Pattern::plain(&format!(
        r#"["']?(?:AWS|aws|Aws)?_?(?:SECRET|secret|Secret)_?(?:ACCESS|access|Access)_?(?:KEY|key|Key)["']?[{SPACE}]*(?::|=>|=)[{SPACE}]*["']?(?P<key>[A-Za-z0-9/\+=]{{40}})["']?{BOUNDARY}"#
    )).needing(&["SECRET", "secret", "Secret"])
});

/// An account ID, twelve digits in three groups, after `AWS_ACCOUNT_ID` and `=`, `:` or `=>`.
static ACCOUNT_ID: LazyLock<Pattern> = LazyLock::new(|| {
    Pattern::plain(&format!(
        r#"("|')?(AWS|aws|Aws)?_?(ACCOUNT|account|Account)_?(ID|id|Id)?("|')?[{SPACE}]*(:|=>|=)[{SPACE}]*("|')?[0-9]{{4}}-?[0-9]{{4}}-?[0-9]{{4}}("|')?{BOUNDARY}"#
    ))
});

/// The rule's findings: the IDs first when their option is on, then the secret keys.
pub fn report(source: &Source<'_>, options: Options, findings: &mut Vec<Finding>) {
    if options.aws_id_scan {
        for found in ACCESS_KEY_ID.find_all(&source.content) {
            let id = &source.content[found.start..found.end];

            if id != EXAMPLE_KEY_ID {
                findings.push(finding(
                    RULE,
                    "AWSAccessKeyID",
                    format!("found AWS Access Key ID: {id}"),
                    vec![id.to_owned()],
                    source.range(found.start, id),
                ));
            }
        }
        for found in ACCOUNT_ID.find_all(&source.content) {
            let id = &source.content[found.start..found.end];

            findings.push(finding(
                RULE,
                "AWSAccountID",
                format!("found AWS Account ID: {id}"),
                vec![id.to_owned()],
                source.range(found.start, id),
            ));
        }
    }
    for found in SECRET_ACCESS_KEY.find_all(&source.content) {
        let key = found.group("key").unwrap_or_default();

        if key != EXAMPLE_SECRET {
            findings.push(finding(
                RULE,
                "AWSSecretAccessKey",
                format!("found AWS Secret Access Key: {key}"),
                vec![key.to_owned()],
                source.range(found.start, key),
            ));
        }
    }
}

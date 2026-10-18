//! The preset's own cases (EN-110, F21.AC4): every snapshot of
//! `@secretlint/secretlint-rule-preset-recommend` 13.0.7's tests, the whole preset run on each
//! input as `check:all` runs it, copied from secretlint's repository at the tag `v13.0.7`
//! (`packages/@secretlint/secretlint-rule-preset-recommend/test/snapshots/<rule>/<case>/`) into
//! one file per rule, `tests/fixtures/preset/<rule>.json`: each input in base64, so that no
//! secret of the cases is in the repository's text, and each expected finding's rule, message id,
//! range, line, column and masked message. A case `ng.…` has findings, `ok.…` none.
//!
//! The scan takes no option, as the repository sets none: the four cases of an `allows` list
//! (`options.allows`, `ok.allows`) are not run, their option absent, and their allowed tokens are
//! not copied; the cases of
//! `enableIDScanRule` run with `Options::aws_id_scan`.

use std::fs;
use std::path::Path;

use secret_scan::base64;
use secret_scan::rules::Options;
use serde_json::Value;

/// A finding as a case expects it and as the scan gives it.
type Expected = (String, String, (u64, u64), u64, u64, String);

/// The cases whose option the scan does not take.
const NOT_RUN: [&str; 4] = [
    "secretlint-rule-aws/options.allows",
    "secretlint-rule-figma/ok.allows",
    "secretlint-rule-groq/ok.allows",
    "secretlint-rule-tailscale/ok.allows",
];

/// The rules of the preset, one fixture file each.
const RULES: [&str; 28] = [
    "secretlint-rule-1password",
    "secretlint-rule-anthropic",
    "secretlint-rule-aws",
    "secretlint-rule-basicauth",
    "secretlint-rule-cloudflare",
    "secretlint-rule-database-connection-string",
    "secretlint-rule-databricks",
    "secretlint-rule-docker",
    "secretlint-rule-figma",
    "secretlint-rule-filter-comments",
    "secretlint-rule-gcp",
    "secretlint-rule-github",
    "secretlint-rule-gitlab",
    "secretlint-rule-grafana",
    "secretlint-rule-groq",
    "secretlint-rule-hashicorp-vault",
    "secretlint-rule-huggingface",
    "secretlint-rule-linear",
    "secretlint-rule-notion",
    "secretlint-rule-npm",
    "secretlint-rule-openai",
    "secretlint-rule-privatekey",
    "secretlint-rule-sendgrid",
    "secretlint-rule-shopify",
    "secretlint-rule-slack",
    "secretlint-rule-stripe",
    "secretlint-rule-tailscale",
    "secretlint-rule-vercel",
];

/// A rule's fixture file.
fn fixture(rule: &str) -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/preset")
        .join(format!("{rule}.json"));
    let text =
        fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));

    serde_json::from_str(&text).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

/// A string field.
fn text(value: &Value, field: &str) -> String {
    value[field].as_str().unwrap_or_default().to_owned()
}

/// A number field.
fn number(value: &Value, field: &str) -> u64 {
    value[field].as_u64().unwrap_or(u64::MAX)
}

/// What a case expects.
fn expected(case: &Value) -> Vec<Expected> {
    case["messages"]
        .as_array()
        .map(|messages| {
            messages
                .iter()
                .map(|message| {
                    (
                        text(message, "ruleId"),
                        text(message, "messageId"),
                        (
                            message["range"][0].as_u64().unwrap_or(u64::MAX),
                            message["range"][1].as_u64().unwrap_or(u64::MAX),
                        ),
                        number(message, "line"),
                        number(message, "column"),
                        text(message, "masked"),
                    )
                })
                .collect()
        })
        .unwrap_or_default()
}

/// What the scan finds in a case.
fn found(case: &Value) -> Vec<Expected> {
    let bytes = base64::decode(&text(case, "input")).expect("the input in base64");
    let path = format!("{}/{}", text(case, "case"), text(case, "file"));
    let options = Options {
        aws_id_scan: case["options"]["enableIDScanRule"]
            .as_bool()
            .unwrap_or(false),
    };

    secret_scan::scan(&path, &bytes, options)
        .into_iter()
        .map(|located| {
            (
                located.finding.rule_id.to_owned(),
                located.finding.message_id.to_owned(),
                (
                    located.finding.range.0 as u64,
                    located.finding.range.1 as u64,
                ),
                located.line as u64,
                located.column as u64,
                secret_scan::report::masked(&located.finding),
            )
        })
        .collect()
}

/// Runs every case of a rule, and returns the names of those it ran and those it did not.
fn run(rule: &str) -> (Vec<String>, Vec<String>) {
    let file = fixture(rule);
    let mut ran = Vec::new();
    let mut not_run = Vec::new();

    for case in file["cases"].as_array().expect("cases") {
        let name = format!("{rule}/{}", text(case, "case"));

        if case["options"]["allows"].is_array() {
            not_run.push(name);
            continue;
        }
        assert_eq!(found(case), expected(case), "{name}");
        ran.push(name);
    }
    (ran, not_run)
}

#[test]
fn f21_ac4_each_rule_finds_the_presets_positive_cases_and_leaves_its_negative_ones() {
    let mut ran = Vec::new();
    let mut not_run = Vec::new();
    let mut positive = 0;

    for rule in RULES {
        let (rule_ran, rule_not_run) = run(rule);
        let file = fixture(rule);

        positive += file["cases"]
            .as_array()
            .expect("cases")
            .iter()
            .filter(|case| !expected(case).is_empty())
            .count();
        ran.extend(rule_ran);
        not_run.extend(rule_not_run);
    }

    // 115 snapshots at v13.0.7, 4 of them with an `allows` list.
    assert_eq!(ran.len(), 111);
    assert_eq!(not_run, NOT_RUN);
    assert!(positive > 50);
}

#[test]
fn every_rule_of_the_preset_has_a_positive_case() {
    // The comment filter reports nothing itself: its cases test what it drops.
    for rule in RULES
        .iter()
        .filter(|rule| **rule != "secretlint-rule-filter-comments")
    {
        let file = fixture(rule);
        let rule_id = format!("@secretlint/{rule}");
        let finds = file["cases"]
            .as_array()
            .expect("cases")
            .iter()
            .any(|case| expected(case).iter().any(|finding| finding.0 == rule_id));

        assert!(finds, "{rule} has no positive case");
    }
}

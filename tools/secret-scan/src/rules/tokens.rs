//! The rules of `@secretlint/secretlint-rule-preset-recommend` 13.0.7 that report each match of
//! their patterns as it is, a prefix choosing the message at most: `slack`, `sendgrid`, `shopify`,
//! `stripe`, `github`, `gitlab`, `grafana`, `openai`, `anthropic`, `groq`, `huggingface`,
//! `linear`, `notion`, `hashicorp-vault`, `vercel`, `databricks`, `docker`, `figma`, `cloudflare`
//! and `tailscale`, each `@secretlint/secretlint-rule-<name>`. Each pattern is
//! the bundle's, its look-arounds as `Pattern` checks them, `\b`, `\w` and `\d` ASCII (`crate::js`).

use std::sync::LazyLock;

use crate::js::{BOUNDARY, utf16_len};
use crate::pattern::{Found, Pattern};
use crate::source::{Finding, Source};

use super::finding;

/// A message id, its message and the values it masks.
type Message = (&'static str, String, Vec<String>);

/// One pattern of a rule and how a match becomes a finding.
struct Token {
    /// The rule's id.
    rule_id: &'static str,
    /// The pattern.
    pattern: Pattern,
    /// The message id, its message and its masked values for a match, `None` to skip it.
    message: fn(&Found<'_>) -> Option<Message>,
}

/// One message id and its text for the matched token, the token the one value masked.
fn one(id: &'static str, label: &str, token: &str) -> Message {
    (id, format!("{label}: {token}"), vec![token.to_owned()])
}

/// The whole match.
fn whole<'t>(found: &Found<'t>) -> &'t str {
    found.group("m").unwrap_or_default()
}

/// The patterns run before the 1Password and database rules, in the preset's order.
static BEFORE: LazyLock<Vec<Token>> = LazyLock::new(|| {
    vec![
        // @secretlint/secretlint-rule-slack: tokens, then incoming webhooks (the `gi` flags, ASCII
        // case folding as JavaScript's without `u`).
        Token {
            rule_id: "@secretlint/secretlint-rule-slack",
            pattern: Pattern::plain(&format!(
                "{BOUNDARY}(?:xoxb|xoxp|xapp|xoxa|xoxo|xoxr)-(?:[0-9]-)?(?:[a-zA-Z0-9]{{1,40}}-)+[a-zA-Z0-9]{{1,40}}{BOUNDARY}"
            )).needing(&["xox", "xapp-"]),
            message: |found| Some(one("SLACK_TOKEN", "found slack token", whole(found))),
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-slack",
            pattern: Pattern::plain(
                r"(?i-u:https://hooks\.slack\.com/services/T[a-zA-Z0-9]{1,40}/B[a-zA-Z0-9]{1,40}/[a-zA-Z0-9]{1,40})",
            )
            .needing(&["://"]),
            message: |found| {
                Some(one(
                    "IncomingWebhook",
                    "found Slack Incoming Webhook",
                    whole(found),
                ))
            },
        },
        // @secretlint/secretlint-rule-sendgrid: a key is 69 characters long.
        Token {
            rule_id: "@secretlint/secretlint-rule-sendgrid",
            pattern: Pattern::bounded(
                r"SG\.[A-Za-z0-9_]{1,128}\.[A-Za-z0-9_]{1,128}([-_]?)[A-Za-z0-9_]{1,128}",
                "A-Za-z0-9_",
            ).needing(&["SG."]),
            message: |found| {
                let key = whole(found);

                (utf16_len(key) == 69).then(|| one("SENDGRID_KEY", "found Sendgrid api key", key))
            },
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-shopify",
            pattern: Pattern::bounded("(shppa|shpca|shpat|shpss)_[a-zA-Z0-9]{32,64}", "a-zA-Z0-9").needing(&["shp"]),
            message: |found| Some(one("SHOPIFY_KEY", "found Shopify api key", whole(found))),
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-stripe",
            pattern: Pattern::bounded(
                "(?P<prefix>sk_live|sk_test|rk_live|rk_test)_[a-zA-Z0-9]{24,99}",
                "a-zA-Z0-9",
            ).needing(&["k_live_", "k_test_"]),
            message: |found| {
                let (id, label) = match found.group("prefix")? {
                    "sk_live" => ("STRIPE_SECRET_KEY_LIVE", "found Stripe Live Secret Key"),
                    "sk_test" => ("STRIPE_SECRET_KEY_TEST", "found Stripe Test Secret Key"),
                    "rk_live" => (
                        "STRIPE_RESTRICTED_KEY_LIVE",
                        "found Stripe Live Restricted Key",
                    ),
                    _ => (
                        "STRIPE_RESTRICTED_KEY_TEST",
                        "found Stripe Test Restricted Key",
                    ),
                };

                Some(one(id, label, whole(found)))
            },
        },
        // @secretlint/secretlint-rule-github: classic, fine-grained and app installation tokens.
        Token {
            rule_id: "@secretlint/secretlint-rule-github",
            pattern: Pattern::bounded(
                "(?P<type>ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9_]{36}",
                "A-Za-z0-9_",
            ).needing(&["gh"]),
            message: github,
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-github",
            pattern: Pattern::bounded("(?P<type>github_pat)_[A-Za-z0-9_]{82}", "A-Za-z0-9_").needing(&["github_pat_"]),
            message: github,
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-github",
            pattern: Pattern::bounded(
                r"(?P<type>ghs)_[0-9]{1,20}_[A-Za-z0-9_-]{1,1000}\.[A-Za-z0-9_-]{1,10000}\.[A-Za-z0-9_-]{1,1000}",
                "A-Za-z0-9_-",
            ).needing(&["ghs_"]),
            message: github,
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-gitlab",
            pattern: Pattern::bounded("glpat-[A-Za-z0-9_-]{20,128}", "A-Za-z0-9_-").needing(&["glpat-"]),
            message: |found| {
                Some(one(
                    "GITLAB_PERSONAL_ACCESS_TOKEN",
                    "found GitLab Personal Access Token",
                    whole(found),
                ))
            },
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-grafana",
            pattern: Pattern::bounded("glc_[A-Za-z0-9+/]{32,400}={0,2}", "A-Za-z0-9+/=").needing(&["glc_"]),
            message: |found| {
                Some(one(
                    "GRAFANA_CLOUD_API_TOKEN",
                    "found Grafana Cloud API token",
                    whole(found),
                ))
            },
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-grafana",
            pattern: Pattern::bounded("glsa_[A-Za-z0-9]{32}_[A-Fa-f0-9]{8}", "A-Fa-f0-9").needing(&["glsa_"]),
            message: |found| {
                Some(one(
                    "GRAFANA_SERVICE_ACCOUNT_TOKEN",
                    "found Grafana Service Account token",
                    whole(found),
                ))
            },
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-openai",
            pattern: Pattern::plain(&format!(
                "sk-(?:proj|svcacct|admin)-(?:[A-Za-z0-9_-]{{74}}|[A-Za-z0-9_-]{{58}})T3BlbkFJ(?:[A-Za-z0-9_-]{{74}}|[A-Za-z0-9_-]{{58}}){BOUNDARY}|sk-[a-zA-Z0-9]{{20}}T3BlbkFJ[a-zA-Z0-9]{{20}}"
            )).needing(&["sk-"]),
            message: |found| Some(one("OPENAI_TOKEN", "found OpenAI API token", whole(found))),
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-anthropic",
            pattern: Pattern::bounded("sk-ant-api0[0-9]-[A-Za-z0-9_-]{90,128}AA", "A-Za-z0-9_-").needing(&["sk-ant-api0"]),
            message: |found| {
                Some(one(
                    "ANTHROPIC_API_KEY",
                    "found Anthropic Claude API key",
                    whole(found),
                ))
            },
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-groq",
            pattern: Pattern::bounded("gsk_[a-zA-Z0-9]{52}", "a-zA-Z0-9").needing(&["gsk_"]),
            message: |found| Some(one("GROQ_API_KEY", "found Groq API key", whole(found))),
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-huggingface",
            pattern: Pattern::bounded("hf_[a-zA-Z]{34}", "a-zA-Z").needing(&["hf_"]),
            message: |found| {
                Some(one(
                    "HUGGINGFACE_USER_ACCESS_TOKEN",
                    "found Hugging Face User Access Token",
                    whole(found),
                ))
            },
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-linear",
            pattern: Pattern::bounded("lin_api_[a-zA-Z0-9_]{32,128}", "a-zA-Z0-9_").needing(&["lin_api_"]),
            message: |found| {
                Some(one(
                    "LINEAR_API_TOKEN",
                    "found linear api token",
                    whole(found),
                ))
            },
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-notion",
            pattern: Pattern::bounded("ntn_[0-9]{11}[A-Za-z0-9]{35}", "A-Za-z0-9").needing(&["ntn_"]),
            message: |found| {
                Some(one(
                    "NOTION_INTEGRATION_TOKEN",
                    "found Notion integration token",
                    whole(found),
                ))
            },
        },
    ]
});

/// The patterns run after the database rule, in the preset's order.
static AFTER: LazyLock<Vec<Token>> = LazyLock::new(|| {
    vec![
        // @secretlint/secretlint-rule-hashicorp-vault: service, batch and recovery tokens.
        Token {
            rule_id: "@secretlint/secretlint-rule-hashicorp-vault",
            pattern: Pattern::bounded(r"hvs\.[A-Za-z0-9_-]{90,120}", "A-Za-z0-9_-")
                .needing(&["hvs."]),
            message: |found| {
                Some(one(
                    "HASHICORP_VAULT_SERVICE_TOKEN",
                    "found HashiCorp Vault service token",
                    whole(found),
                ))
            },
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-hashicorp-vault",
            pattern: Pattern::bounded(r"hvb\.[A-Za-z0-9_-]{138,300}", "A-Za-z0-9_-")
                .needing(&["hvb."]),
            message: |found| {
                Some(one(
                    "HASHICORP_VAULT_BATCH_TOKEN",
                    "found HashiCorp Vault batch token",
                    whole(found),
                ))
            },
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-hashicorp-vault",
            pattern: Pattern::bounded(r"hvr\.[A-Za-z0-9_-]{90,120}", "A-Za-z0-9_-")
                .needing(&["hvr."]),
            message: |found| {
                Some(one(
                    "HASHICORP_VAULT_RECOVERY_TOKEN",
                    "found HashiCorp Vault recovery token",
                    whole(found),
                ))
            },
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-vercel",
            pattern: Pattern::bounded(
                "(?P<prefix>vcp|vci|vca|vcr|vck)_[A-Za-z0-9]{20,60}",
                "A-Za-z0-9",
            )
            .needing(&["vc"]),
            message: |found| {
                let (id, label) = match found.group("prefix")? {
                    "vcp" => (
                        "VERCEL_PERSONAL_ACCESS_TOKEN",
                        "found Vercel personal access token",
                    ),
                    "vci" => ("VERCEL_INTEGRATION_TOKEN", "found Vercel integration token"),
                    "vca" => ("VERCEL_APP_ACCESS_TOKEN", "found Vercel app access token"),
                    "vcr" => ("VERCEL_APP_REFRESH_TOKEN", "found Vercel app refresh token"),
                    _ => (
                        "VERCEL_AI_GATEWAY_API_KEY",
                        "found Vercel AI Gateway API key",
                    ),
                };

                Some(one(id, label, whole(found)))
            },
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-databricks",
            pattern: Pattern::bounded("dapi[A-Fa-f0-9]{32}(?:-[0-9])?", "A-Fa-f0-9")
                .needing(&["dapi"]),
            message: |found| {
                Some(one(
                    "DATABRICKS_PERSONAL_ACCESS_TOKEN",
                    "found Databricks personal access token",
                    whole(found),
                ))
            },
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-docker",
            pattern: Pattern::bounded("dckr_pat_[a-zA-Z0-9_-]{27}", "a-zA-Z0-9_-")
                .needing(&["dckr_pat_"]),
            message: |found| {
                Some(one(
                    "DOCKER_PERSONAL_ACCESS_TOKEN",
                    "found Docker Personal Access Token",
                    whole(found),
                ))
            },
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-figma",
            pattern: Pattern::bounded("figd_[A-Za-z0-9_-]{40,200}", "A-Za-z0-9_-")
                .needing(&["figd_"]),
            message: |found| {
                Some(one(
                    "FIGMA_PERSONAL_ACCESS_TOKEN",
                    "found Figma Personal Access Token",
                    whole(found),
                ))
            },
        },
        Token {
            rule_id: "@secretlint/secretlint-rule-cloudflare",
            pattern: Pattern::bounded(
                "(?P<prefix>cfk|cfut|cfat)_[A-Za-z0-9]{40}[0-9a-f]{8}",
                "A-Za-z0-9",
            )
            .needing(&["cf"]),
            message: |found| {
                let (id, label) = match found.group("prefix")? {
                    "cfk" => (
                        "CLOUDFLARE_GLOBAL_API_KEY",
                        "found Cloudflare Global API Key",
                    ),
                    "cfut" => (
                        "CLOUDFLARE_USER_API_TOKEN",
                        "found Cloudflare User API Token",
                    ),
                    _ => (
                        "CLOUDFLARE_ACCOUNT_API_TOKEN",
                        "found Cloudflare Account API Token",
                    ),
                };

                Some(one(id, label, whole(found)))
            },
        },
        // @secretlint/secretlint-rule-tailscale: `tskey-{type}-{id}-{secret}`, an unknown type the
        // generic message.
        Token {
            rule_id: "@secretlint/secretlint-rule-tailscale",
            pattern: Pattern::bounded(
                "tskey-(?P<type>[a-z]{2,20})-[0-9A-Za-z_]{8,40}-[0-9A-Za-z_]{16,60}",
                "0-9A-Za-z_",
            )
            .needing(&["tskey-"]),
            message: |found| {
                let (id, label) = match found.group("type")? {
                    "api" => ("TAILSCALE_API_KEY", "found Tailscale API access token"),
                    "auth" => (
                        "TAILSCALE_AUTH_KEY",
                        "found Tailscale pre-authentication key",
                    ),
                    "client" => ("TAILSCALE_CLIENT_KEY", "found Tailscale OAuth client key"),
                    "scim" => ("TAILSCALE_SCIM_KEY", "found Tailscale SCIM key"),
                    "webhook" => ("TAILSCALE_WEBHOOK_KEY", "found Tailscale webhook key"),
                    _ => ("TAILSCALE_KEY", "found Tailscale key"),
                };

                Some(one(id, label, whole(found)))
            },
        },
    ]
});

/// A GitHub token: its type named in the message, both values masked.
fn github(found: &Found<'_>) -> Option<Message> {
    let key = whole(found);
    let type_name = match found.group("type")? {
        "ghp" => "GitHub personal access tokens",
        "gho" => "OAuth access tokens",
        "ghu" => "GitHub user-to-server tokens",
        "ghs" => "GitHub server-to-server tokens",
        "ghr" => "refresh tokens",
        _ => "fine-grained personal access tokens",
    };

    Some((
        "GITHUB_TOKEN",
        format!("found GitHub Token({type_name}): {key}"),
        vec![key.to_owned(), type_name.to_owned()],
    ))
}

/// Runs the tokens on the file.
fn run(tokens: &[Token], source: &Source<'_>, findings: &mut Vec<Finding>) {
    for token in tokens {
        for found in token.pattern.find_all(&source.content) {
            if let Some((id, message, data)) = (token.message)(&found) {
                let range = source.range(found.start, whole(&found));

                findings.push(finding(token.rule_id, id, message, data, range));
            }
        }
    }
}

/// The rules from `slack` to `notion`.
pub fn report(source: &Source<'_>, findings: &mut Vec<Finding>) {
    run(&BEFORE, source, findings);
}

/// The rules from `hashicorp-vault` to `tailscale`.
pub fn report_after_database(source: &Source<'_>, findings: &mut Vec<Finding>) {
    run(&AFTER, source, findings);
}

//! `dependencies outdated` on recorded answers of the registries, GitHub and Node.js's index
//! (`tests/fixtures/`, recorded on 2026-10-18 and trimmed to the fields read), never the network.
//! The instants are those of the recording: `NOW` is 2026-10-18T01:00:00Z, so the cooldown takes
//! what was published before 2026-10-11T01:00:00Z.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use deps::actions::{self, ActionUse};
use deps::fetch::{self, Answer};
use deps::outdated::{self, Network, Pins, Report};
use deps::version::Version;
use deps::{Behind, crates, node, npm};
use hooks::time::epoch_ms;

/// The instant of the recording.
const NOW: &str = "2026-10-18T01:00:00Z";

fn now() -> i64 {
    epoch_ms(NOW)
}

fn fixture(path: &str) -> String {
    fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(path),
    )
    .expect("a fixture")
}

fn json(path: &str) -> serde_json::Value {
    serde_json::from_str(&fixture(path)).expect("JSON")
}

fn version(text: &str) -> Version {
    Version::parse(text).expect("a version")
}

// --- Version numbers (REF-SEMVER) ---

#[test]
fn parses_a_version_and_refuses_what_is_none() {
    assert_eq!(version("1.2.3").major, 1);
    assert_eq!(version("1.2.3").minor, 2);
    assert_eq!(version("1.2.3").patch, 3);
    assert!(version("2.0.0-alpha.20").is_pre());
    assert!(!version("1.0.0+build.5").is_pre());
    // Build metadata takes no part in precedence.
    assert_eq!(version("1.0.0+build.5"), version("1.0.0"));
    for text in [
        "1.2",
        "v1.2.3",
        "1.02.3",
        "1.2.3.4",
        "^1.2.3",
        "1.2.3-",
        "1.2.3-a..b",
        "1.2.3-01",
        "",
    ] {
        assert_eq!(Version::parse(text), None, "{text}");
    }
}

#[test]
fn orders_versions_by_the_precedence_of_the_specification() {
    // The specification's own chain (semver.org, item 11), then numeric minors.
    let chain = [
        "1.0.0-alpha",
        "1.0.0-alpha.1",
        "1.0.0-alpha.beta",
        "1.0.0-beta",
        "1.0.0-beta.2",
        "1.0.0-beta.11",
        "1.0.0-rc.1",
        "1.0.0",
        "1.9.0",
        "1.10.0",
        "2.0.0",
    ];

    for pair in chain.windows(2) {
        assert!(
            version(pair[0]) < version(pair[1]),
            "{} < {}",
            pair[0],
            pair[1]
        );
        assert!(
            version(pair[1]) > version(pair[0]),
            "{} > {}",
            pair[1],
            pair[0]
        );
    }
}

#[test]
fn groups_compatible_versions_by_their_left_most_non_zero_component() {
    assert!(version("1.2.3").is_compatible(&version("1.9.0")));
    assert!(!version("1.2.3").is_compatible(&version("2.0.0")));
    assert!(version("0.2.3").is_compatible(&version("0.2.9")));
    assert!(!version("0.2.3").is_compatible(&version("0.3.0")));
    assert!(version("0.0.3").is_compatible(&version("0.0.3")));
    assert!(!version("0.0.3").is_compatible(&version("0.0.4")));
}

// --- npm (REF-NCU-COOLDOWN) ---

#[test]
fn f21_ac4_npm_takes_the_latest_tag_once_it_is_7_days_old() {
    let oxlint = json("npm/oxlint.json");
    // 1.88.0, the latest tag, published 2026-10-13T09:47:46.926Z: 4 days old at NOW, so the
    // greatest version 7 days old above 1.87.0 is looked for, and there is none.
    assert_eq!(npm::behind("oxlint", "1.87.0", &oxlint, now()), None);
    // Exactly 7 days after its publication it is taken; one millisecond before, not.
    let seven_days = epoch_ms("2026-10-20T09:47:46.926Z");

    assert_eq!(
        npm::behind("oxlint", "1.87.0", &oxlint, seven_days),
        Some(Behind {
            name: "oxlint".to_owned(),
            current: "1.87.0".to_owned(),
            latest: "1.88.0".to_owned(),
            published_ms: Some(epoch_ms("2026-10-13T09:47:46.926Z")),
        })
    );
    assert_eq!(
        npm::behind("oxlint", "1.87.0", &oxlint, seven_days - 1),
        None
    );
}

#[test]
fn f21_ac4_npm_falls_back_to_the_greatest_version_7_days_old() {
    // From 1.85.0 at NOW: 1.88.0 is too new, 1.87.0 (2026-10-05) is 12 days old.
    let behind = npm::behind("oxlint", "1.85.0", &json("npm/oxlint.json"), now()).expect("behind");

    assert_eq!(behind.latest, "1.87.0");
    assert_eq!(outdated::date(behind.published_ms), "2026-10-05");
}

#[test]
fn f21_ac4_npm_keeps_a_prerelease_pin_as_ncu_does() {
    let vitepress = json("npm/vitepress.json");
    // The latest tag names 1.6.4, below 2.0.0-alpha.20: no upgrade, no newer alpha either, since
    // the latest tag is old enough and taken as the target (ncu's `target: "latest"`).
    assert_eq!(
        npm::behind("vitepress", "2.0.0-alpha.20", &vitepress, now()),
        None
    );
    assert_eq!(
        npm::behind("vitepress", "2.0.0-alpha.19", &vitepress, now()),
        None
    );
    // A stable pin below the latest tag is upgraded to it.
    assert_eq!(
        npm::behind("vitepress", "1.6.3", &vitepress, now()).map(|behind| behind.latest),
        Some("1.6.4".to_owned())
    );
}

#[test]
fn npm_leaves_out_deprecated_versions_and_prereleases_of_a_stable_pin() {
    let metadata = serde_json::json!({
        "dist-tags": { "latest": "3.0.0" },
        "time": {
            "1.0.0": "2026-01-01T00:00:00.000Z",
            "2.0.0": "2026-02-01T00:00:00.000Z",
            "2.1.0-beta.1": "2026-03-01T00:00:00.000Z",
            "3.0.0": "2026-04-01T00:00:00.000Z"
        },
        "versions": {
            "1.0.0": {},
            "2.0.0": {},
            "2.1.0-beta.1": {},
            "3.0.0": { "deprecated": "broken" }
        }
    });

    // 3.0.0 is deprecated: the greatest other is 2.1.0-beta.1, a pre-release, left out for a
    // stable pin; 2.0.0 is taken.
    assert_eq!(
        npm::behind("planted", "1.0.0", &metadata, now()).map(|behind| behind.latest),
        Some("2.0.0".to_owned())
    );
    // A pre-release pin takes pre-releases.
    assert_eq!(
        npm::behind("planted", "2.1.0-alpha.1", &metadata, now()).map(|behind| behind.latest),
        Some("2.1.0-beta.1".to_owned())
    );
}

#[test]
fn npm_reads_nothing_from_a_range_or_metadata_without_a_latest_tag() {
    let oxlint = json("npm/oxlint.json");

    assert_eq!(npm::behind("oxlint", "^1.85.0", &oxlint, now()), None);
    assert_eq!(
        npm::behind("oxlint", "1.85.0", &serde_json::json!({}), now()),
        None
    );
}

#[test]
fn npm_takes_a_version_without_a_date_as_old_enough() {
    let metadata = serde_json::json!({
        "dist-tags": { "latest": "2.0.0" },
        "time": {},
        "versions": { "1.0.0": {}, "2.0.0": {} }
    });
    let behind = npm::behind("planted", "1.0.0", &metadata, now()).expect("behind");

    assert_eq!(behind.latest, "2.0.0");
    assert_eq!(outdated::date(behind.published_ms), "date unknown");
}

// --- crates (REF-CARGO-REGISTRY-INDEX) ---

#[test]
fn writes_the_path_of_a_crate_in_the_index() {
    assert_eq!(crates::index_path("a"), "1/a");
    assert_eq!(crates::index_path("ab"), "2/ab");
    assert_eq!(crates::index_path("syn"), "3/s/syn");
    assert_eq!(crates::index_path("serde_json"), "se/rd/serde_json");
    assert_eq!(crates::index_path("Cargo"), "ca/rg/cargo");
}

#[test]
fn f21_ac4_crates_take_the_latest_version_7_days_old() {
    let releases = crates::releases(&fixture("crates/serde_json"));

    assert_eq!(releases.len(), 6);
    // 1.0.153 and 1.0.154 were published on 2026-10-11 at 14:19 and 15:31: under 7 days at NOW.
    assert_eq!(
        crates::behind("serde_json", "1.0.152", &releases, now()),
        None
    );
    // At 16:00 on 2026-10-18 both are; 1.0.154 is the greatest.
    assert_eq!(
        crates::behind(
            "serde_json",
            "1.0.152",
            &releases,
            epoch_ms("2026-10-18T16:00:00Z")
        ),
        Some(Behind {
            name: "serde_json".to_owned(),
            current: "1.0.152".to_owned(),
            latest: "1.0.154".to_owned(),
            published_ms: Some(epoch_ms("2026-10-11T15:31:46Z")),
        })
    );
    // From 1.0.151 at NOW: 1.0.152, of 2026-10-10T17:04:20Z, is 7 days and 8 hours old.
    assert_eq!(
        crates::behind("serde_json", "1.0.151", &releases, now()).map(|behind| behind.latest),
        Some("1.0.152".to_owned())
    );
}

#[test]
fn f21_ac4_a_locked_crate_under_7_days_is_pinned_back() {
    let syn = crates::releases(&fixture("crates/syn"));
    let ident = crates::releases(&fixture("crates/unicode-ident"));
    // syn 3.0.9 (2026-10-12) and 3.0.8 (2026-10-11T16:12) are under 7 days; 3.0.7 (2026-10-10)
    // is not.
    let too_new = crates::too_new("syn", "3.0.9", &syn, now()).expect("too new");

    assert_eq!(too_new.pin.as_deref(), Some("3.0.7"));
    assert_eq!(outdated::date(Some(too_new.published_ms)), "2026-10-12");
    assert_eq!(crates::too_new("syn", "3.0.7", &syn, now()), None);
    assert_eq!(
        crates::too_new("unicode-ident", "1.0.27", &ident, now()).and_then(|new| new.pin),
        Some("1.0.26".to_owned())
    );
}

#[test]
fn crates_leave_out_yanked_versions_and_take_one_without_a_date() {
    let index = [
        r#"{"name":"planted","vers":"0.1.0","yanked":false}"#,
        r#"{"name":"planted","vers":"0.1.1","yanked":true,"pubtime":"2026-01-01T00:00:00Z"}"#,
        r#"{"name":"planted","vers":"0.2.0-rc.1","yanked":false,"pubtime":"2026-01-01T00:00:00Z"}"#,
        r#"{"name":"planted","vers":"0.1.2","yanked":false,"pubtime":"2026-10-17T00:00:00Z"}"#,
        "not a line of the index",
    ]
    .join("\n");
    let releases = crates::releases(&index);

    assert_eq!(releases.len(), 4);
    // 0.1.1 is yanked, 0.2.0-rc.1 a pre-release, 0.1.2 a day old: 0.1.0 is up to date.
    assert_eq!(crates::behind("planted", "0.1.0", &releases, now()), None);
    // 0.1.2 locked: no compatible version 7 days old above 0.1.0 but 0.1.0 itself, without a date.
    assert_eq!(
        crates::too_new("planted", "0.1.2", &releases, now()).and_then(|new| new.pin),
        Some("0.1.0".to_owned())
    );
    // A version without a date is never too new; a version the index lacks is not judged.
    assert_eq!(crates::too_new("planted", "0.1.0", &releases, now()), None);
    assert_eq!(crates::too_new("planted", "9.9.9", &releases, now()), None);
    assert_eq!(crates::behind("planted", "*", &releases, now()), None);
}

// --- GitHub Actions and Node.js ---

#[test]
fn reads_the_actions_a_workflow_uses() {
    let workflow = "steps:\n  - uses: actions/checkout@v7\n    with:\n      fetch-depth: 0\n  - name: x\n    uses: github/codeql-action/init@v4.1.0\n  - uses: \"actions/setup-node@v7\"\n  - uses: ./.github/local\n  - uses: docker://alpine:3\n  - uses: owner/repository@0123abcd\n";

    assert_eq!(
        actions::uses(workflow),
        [
            ActionUse {
                repository: "actions/checkout".to_owned(),
                major: 7
            },
            ActionUse {
                repository: "github/codeql-action".to_owned(),
                major: 4
            },
            ActionUse {
                repository: "actions/setup-node".to_owned(),
                major: 7
            },
        ]
    );
}

#[test]
fn reads_the_tags_of_the_next_major_and_its_first_one() {
    let mut references = json("github/checkout-v7-refs.json");

    references
        .as_array_mut()
        .expect("an array")
        .push(serde_json::json!({ "ref": "refs/tags/v70.0.0" }));
    let tags = actions::major_tags(&references, 7);

    assert_eq!(tags, ["v7", "v7.0.0", "v7.0.1"]);
    assert_eq!(
        actions::first_tag(&tags).map(String::as_str),
        Some("v7.0.0")
    );
    assert_eq!(
        actions::first_tag(&["v7".to_owned()]).map(String::as_str),
        Some("v7")
    );
    assert_eq!(
        actions::major_tags(&json("github/checkout-v8-refs.json"), 8),
        Vec::<String>::new()
    );
    assert_eq!(
        actions::commit_ms(&json("github/checkout-v7.0.0-commit.json")),
        Some(epoch_ms("2026-06-17T17:51:53Z"))
    );
}

#[test]
fn f21_ac4_node_takes_the_newest_lts_release_7_days_old() {
    let index = json("node-index.json");
    // v26 is no LTS yet; v24.22.0 (2026-10-14) is 4 days old at NOW, v24.21.0 (2026-09-07) is taken.
    let lts = node::latest_lts(&index, now()).expect("an LTS release");

    assert_eq!((lts.major, lts.version.as_str()), (24, "v24.21.0"));
    // On 2026-10-21 at midnight, exactly 7 days after 2026-10-14.
    assert_eq!(
        node::latest_lts(&index, epoch_ms("2026-10-21T00:00:00Z")).map(|lts| lts.version),
        Some("v24.22.0".to_owned())
    );
    assert_eq!(node::latest_lts(&serde_json::json!([]), now()), None);
}

// --- The check, from the files to the report ---

/// The recorded answers, by address or GitHub path; anything else is a 404.
struct Recorded {
    answers: HashMap<String, String>,
}

impl Recorded {
    fn new() -> Self {
        let answers = [
            ("https://registry.npmjs.org/oxlint", "npm/oxlint.json"),
            ("https://registry.npmjs.org/vitepress", "npm/vitepress.json"),
            (
                "https://index.crates.io/se/rd/serde_json",
                "crates/serde_json",
            ),
            ("https://index.crates.io/3/s/syn", "crates/syn"),
            ("https://nodejs.org/dist/index.json", "node-index.json"),
            (
                "repos/actions/checkout/git/matching-refs/tags/v7",
                "github/checkout-v7-refs.json",
            ),
            (
                "repos/actions/checkout/git/matching-refs/tags/v8",
                "github/checkout-v8-refs.json",
            ),
            (
                "repos/actions/checkout/commits/tags/v7.0.0",
                "github/checkout-v7.0.0-commit.json",
            ),
        ]
        .into_iter()
        .map(|(key, path)| (key.to_owned(), fixture(path)))
        .collect();

        Self { answers }
    }

    fn answer(&self, key: &str) -> Answer {
        self.answers
            .get(key)
            .cloned()
            .ok_or_else(|| "HTTP 404".to_owned())
    }
}

impl Network for Recorded {
    fn urls(&self, addresses: &[String]) -> Vec<Answer> {
        addresses
            .iter()
            .map(|address| self.answer(address))
            .collect()
    }

    fn github(&self, paths: &[String]) -> Vec<Answer> {
        paths.iter().map(|path| self.answer(path)).collect()
    }
}

/// No answer from anywhere.
struct Offline;

impl Network for Offline {
    fn urls(&self, addresses: &[String]) -> Vec<Answer> {
        addresses
            .iter()
            .map(|_| Err("Could not resolve host".to_owned()))
            .collect()
    }

    fn github(&self, paths: &[String]) -> Vec<Answer> {
        paths
            .iter()
            .map(|_| Err("error connecting to api.github.com".to_owned()))
            .collect()
    }
}

fn pins() -> Pins {
    Pins {
        npm: vec![
            ("oxlint".to_owned(), "1.85.0".to_owned()),
            ("vitepress".to_owned(), "2.0.0-alpha.20".to_owned()),
        ],
        actions: vec![
            ActionUse {
                repository: "actions/checkout".to_owned(),
                major: 6,
            },
            ActionUse {
                repository: "actions/checkout".to_owned(),
                major: 7,
            },
        ],
        node: Some(22),
        crates: vec![("serde_json".to_owned(), "1.0.151".to_owned())],
        locked: vec![
            ("serde_json".to_owned(), "1.0.152".to_owned()),
            ("syn".to_owned(), "3.0.9".to_owned()),
        ],
    }
}

#[test]
fn f21_ac4_lists_each_version_behind_with_its_date_and_exits_1() {
    let report = outdated::outdated(&pins(), &Recorded::new(), now());

    assert_eq!(
        report,
        Report {
            lines: vec![
                "npm oxlint 1.85.0 → 1.87.0, 2026-10-05".to_owned(),
                "action actions/checkout v6 → v7 or later (v7.0.0), 2026-06-17".to_owned(),
                "node .nvmrc 22 → 24 (v24.21.0), 2026-09-07".to_owned(),
                "crate serde_json 1.0.151 → 1.0.152, 2026-10-10".to_owned(),
                "lock syn 3.0.9, 2026-10-12, under 7 days: cargo update -p syn@3.0.9 --precise 3.0.7"
                    .to_owned(),
            ],
            failures: Vec::new(),
        }
    );
    assert_eq!(report.exit_code(), 1);
}

#[test]
fn f21_ac4_a_source_that_does_not_answer_is_named_and_exits_2() {
    let report = outdated::outdated(&pins(), &Offline, now());

    assert_eq!(report.lines, Vec::<String>::new());
    assert_eq!(
        report.failures,
        [
            "npm registry: oxlint: Could not resolve host",
            "npm registry: vitepress: Could not resolve host",
            "GitHub: actions/checkout: error connecting to api.github.com",
            "GitHub: actions/checkout: error connecting to api.github.com",
            "Node.js index: Could not resolve host",
            "crates.io index: serde_json: Could not resolve host",
            "crates.io index: syn: Could not resolve host",
        ]
    );
    assert_eq!(report.exit_code(), 2);
}

#[test]
fn an_answer_that_is_not_json_is_a_failure_too() {
    struct Garbled;

    impl Network for Garbled {
        fn urls(&self, addresses: &[String]) -> Vec<Answer> {
            addresses.iter().map(|_| Ok("<html>".to_owned())).collect()
        }

        fn github(&self, paths: &[String]) -> Vec<Answer> {
            paths.iter().map(|_| Ok("[]".to_owned())).collect()
        }
    }
    let pins = Pins {
        npm: vec![("oxlint".to_owned(), "1.85.0".to_owned())],
        node: Some(24),
        ..Pins::default()
    };
    let report = outdated::outdated(&pins, &Garbled, now());

    assert_eq!(report.failures.len(), 2);
    assert!(report.failures[0].starts_with("npm registry: oxlint: not JSON"));
    assert!(report.failures[1].starts_with("Node.js index: not JSON"));
}

#[test]
fn an_up_to_date_tree_lists_nothing_and_exits_0() {
    let pins = Pins {
        npm: vec![("oxlint".to_owned(), "1.87.0".to_owned())],
        actions: vec![ActionUse {
            repository: "actions/checkout".to_owned(),
            major: 7,
        }],
        node: Some(24),
        crates: vec![("serde_json".to_owned(), "1.0.152".to_owned())],
        locked: vec![("syn".to_owned(), "3.0.7".to_owned())],
    };
    let report = outdated::outdated(&pins, &Recorded::new(), now());

    assert_eq!(report, Report::default());
    assert_eq!(report.exit_code(), 0);
}

#[test]
fn a_next_major_under_7_days_is_not_listed_and_a_failed_commit_is_named() {
    struct Fresh;

    impl Network for Fresh {
        fn urls(&self, addresses: &[String]) -> Vec<Answer> {
            addresses.iter().map(|_| Err("unused".to_owned())).collect()
        }

        fn github(&self, paths: &[String]) -> Vec<Answer> {
            paths
                .iter()
                .map(|path| match path.as_str() {
                    "repos/fresh/action/git/matching-refs/tags/v2" => {
                        Ok(r#"[{"ref":"refs/tags/v2.0.0"}]"#.to_owned())
                    }
                    "repos/fresh/action/commits/tags/v2.0.0" => Ok(
                        r#"{"commit":{"committer":{"date":"2026-10-17T00:00:00Z"}}}"#.to_owned(),
                    ),
                    "repos/broken/action/git/matching-refs/tags/v2" => {
                        Ok(r#"[{"ref":"refs/tags/v2"}]"#.to_owned())
                    }
                    _ => Err("HTTP 502".to_owned()),
                })
                .collect()
        }
    }
    let pins = Pins {
        actions: vec![
            ActionUse {
                repository: "broken/action".to_owned(),
                major: 1,
            },
            ActionUse {
                repository: "fresh/action".to_owned(),
                major: 1,
            },
        ],
        ..Pins::default()
    };
    let report = outdated::outdated(&pins, &Fresh, now());

    assert_eq!(report.lines, Vec::<String>::new());
    assert_eq!(report.failures, ["GitHub: broken/action: HTTP 502"]);
}

// --- The pins, read from a checkout ---

fn checkout(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("deps-outdated-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);

    fs::create_dir_all(root.join(".github/workflows")).expect("a folder");
    root
}

#[test]
fn reads_the_pins_of_a_checkout() {
    let root = checkout("pins");
    let files = [
        (
            "package.json",
            r#"{ "dependencies": {}, "devDependencies": { "oxlint": "1.87.0", "vitepress": "2.0.0-alpha.20" } }"#,
        ),
        (".nvmrc", "24\n"),
        (
            ".github/workflows/b.yml",
            "    - uses: actions/checkout@v7\n    - uses: actions/setup-node@v7\n",
        ),
        (
            ".github/workflows/a.yml",
            "    - uses: actions/checkout@v7\n",
        ),
        (".github/workflows/notes.md", "uses: other/action@v1\n"),
        (
            "Cargo.toml",
            "[workspace]\nmembers = [\"tools/*\"]\n\n[workspace.dependencies]\nregex = \"=1.13.1\"\nrmcp = { version = \"=3.5.1\", default-features = false }\n\n[profile.release]\nlto = true\n",
        ),
        (
            "Cargo.lock",
            "version = 4\n\n[[package]]\nname = \"deps\"\nversion = \"0.1.0\"\n\n[[package]]\nname = \"syn\"\nversion = \"3.0.7\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\nchecksum = \"00\"\n",
        ),
    ];

    for (path, text) in files {
        fs::write(root.join(path), text).expect("a file");
    }
    let pins = outdated::read_pins(&root);

    assert_eq!(
        pins,
        Pins {
            npm: vec![
                ("oxlint".to_owned(), "1.87.0".to_owned()),
                ("vitepress".to_owned(), "2.0.0-alpha.20".to_owned()),
            ],
            actions: vec![
                ActionUse {
                    repository: "actions/checkout".to_owned(),
                    major: 7
                },
                ActionUse {
                    repository: "actions/setup-node".to_owned(),
                    major: 7
                },
            ],
            node: Some(24),
            crates: vec![
                ("regex".to_owned(), "1.13.1".to_owned()),
                ("rmcp".to_owned(), "3.5.1".to_owned()),
            ],
            locked: vec![("syn".to_owned(), "3.0.7".to_owned())],
        }
    );
    fs::remove_dir_all(&root).expect("removed");
}

#[test]
fn an_empty_checkout_pins_nothing() {
    assert_eq!(
        outdated::read_pins(Path::new("/nonexistent/checkout")),
        Pins::default()
    );
}

// --- The commands that fetch ---

#[test]
fn reads_curl_s_write_out_for_each_address() {
    let write_out =
        "https://a/x\t200\t\nhttps://a/y\t404\t\nhttps://a/z\t000\tCould not resolve host: a\n";

    assert_eq!(
        fetch::answer_of("https://a/x", write_out, Some("{}".to_owned())),
        Ok("{}".to_owned())
    );
    assert_eq!(
        fetch::answer_of("https://a/x", write_out, None),
        Err("no body written".to_owned())
    );
    assert_eq!(
        fetch::answer_of("https://a/y", write_out, None),
        Err("HTTP 404".to_owned())
    );
    assert_eq!(
        fetch::answer_of("https://a/z", write_out, None),
        Err("Could not resolve host: a".to_owned())
    );
    assert_eq!(
        fetch::answer_of("https://a/w", write_out, None),
        Err("no answer".to_owned())
    );
}

#[test]
fn a_command_that_fails_or_is_missing_answers_nothing() {
    let addresses = ["https://registry.npmjs.org/oxlint".to_owned()];

    assert_eq!(
        fetch::urls("false", &addresses),
        [Err("no answer".to_owned())]
    );
    assert!(
        fetch::urls("/nonexistent/curl", &addresses)[0]
            .as_ref()
            .is_err_and(|error| error.starts_with("/nonexistent/curl: "))
    );
    assert_eq!(fetch::urls("false", &[]), Vec::<Answer>::new());
    assert_eq!(
        fetch::github("false", &["repos/a/b".to_owned()]),
        [Err("no answer".to_owned())]
    );
    assert!(
        fetch::github("/nonexistent/gh", &["repos/a/b".to_owned()])[0]
            .as_ref()
            .is_err_and(|error| error.starts_with("/nonexistent/gh: "))
    );
    assert_eq!(
        fetch::github("true", &["repos/a/b".to_owned()]),
        [Ok(String::new())]
    );
}

// --- The binary ---

fn run(folder: &Path, arguments: &[&str]) -> std::process::Output {
    test_support::command(env!("CARGO_BIN_EXE_dependencies"))
        .args(arguments)
        .current_dir(folder)
        .env("PATH", "/nonexistent")
        .output()
        .expect("the binary runs")
}

#[test]
fn f21_ac4_the_binary_exits_2_naming_the_source_without_a_network() {
    let root = checkout("binary");

    fs::write(root.join(".nvmrc"), "24\n").expect("a file");
    let output = run(&root, &["outdated"]);

    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("no answer from Node.js index: curl: ")
    );
    fs::remove_dir_all(&root).expect("removed");
}

#[test]
fn the_binary_exits_0_with_nothing_pinned_and_2_without_a_command() {
    let root = checkout("empty");
    let output = run(&root, &["outdated"]);

    assert_eq!(output.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&output.stdout).starts_with("Every npm package"));
    assert_eq!(run(&root, &[]).status.code(), Some(2));
    fs::remove_dir_all(&root).expect("removed");
}

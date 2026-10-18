---
id: EN-112
epic: E17
feature: F21
title: The release made by our own tool, in place of release-please
status: done
points: 8
---

# E17 · F21 · EN-112 — The release made by our own tool, in place of release-please

Decided on 2026-10-17 (F21's decisions): our release tool replaces release-please entirely, the GitHub release included; the version bump and the changelog are written inside the pull request `develop` → `main`, before it merges, so that no commit is made by the CI after the merge; the release notes are to be better than release-please's, from what the documentation on good release notes says; the tool may improve anything else it finds (F21.AC6). Today release-please opens a release pull request on `develop` that the Product Owner merges before the pull request into `main`, then tags and publishes from `main` (ADR-0067, `docs/tooling/releases.md`): two merges per release, a personal access token, and a third-party action.

## What good release notes are, from their sources

Read through the library on 2026-10-17:

| Rule | Source |
| --- | --- |
| Written for the people who use the library, not a copy of the commit log: "full of noise", each commit documents "a step in the evolution of the source code", a release entry "the noteworthy difference, often across multiple commits" | `REF-KEEP-A-CHANGELOG` (Commit log diffs), `REF-COMMON-CHANGELOG` (4.1) |
| One entry per version, the latest first, headed `## [x.y.z] - YYYY-MM-DD`, the date in ISO 8601, the version linked to its GitHub release, which holds the same content | `REF-KEEP-A-CHANGELOG` (Guiding principles, Confusing dates), `REF-COMMON-CHANGELOG` (2.2) |
| Changes grouped by their impact, in the order `Changed`, `Added`, `Removed`, `Fixed`; deprecations, removals and breaking changes always listed | `REF-COMMON-CHANGELOG` (2.4), `REF-KEEP-A-CHANGELOG` (Types of changes, Ignoring deprecations) |
| A breaking change first in its group, marked `**Breaking:**` | `REF-COMMON-CHANGELOG` (2.4.4) |
| Each change one line, in the imperative mood, self-describing without its heading, followed by a link to its pull request or commit | `REF-COMMON-CHANGELOG` (2.4.1, 2.4.2, 3.6) |
| Noise removed — dotfiles, development-only dependencies, formatting — but refactorings and new documentation kept | `REF-COMMON-CHANGELOG` (3.2) |
| Related changes merged into one line, changes that undo each other left out | `REF-COMMON-CHANGELOG` (3.4, 3.5) |
| At most one notice, a single sentence, for a first release or an upgrade guide | `REF-COMMON-CHANGELOG` (2.3) |
| Conventional Commits only draft the categories; the lines are then rephrased: a draft generated, then edited | `REF-COMMON-CHANGELOG` (3.1, 4.2) |
| The changelog says it follows Semantic Versioning; before 1.0.0 a release raises the minor version | `REF-KEEP-A-CHANGELOG` (Guiding principles), `REF-SEMVER` (Summary, FAQ) |

Read again by SP-023 on 2026-10-18 ([research note 0025](../../research/notes/0025-f21-agent-tools-in-rust.md), Good release notes, read again): every rule above is in its source as cited. Where the two sources differ, the tool follows Common Changelog and `docs/tooling/releases.md` says so: no `Unreleased` section, the entry written at release time with its references; no `Deprecated` nor `Security` group, a deprecation listed under `Changed` (`REF-COMMON-CHANGELOG`, 6.2; `REF-KEEP-A-CHANGELOG`). What release-please does here and the tool takes over — the version from the commits with `bump-minor-pre-major`, `package.json`, `package-lock.json` and the changelog written, the merge commit tagged `vX.Y.Z`, the release published with the entry — is listed in 0025 (The rewritten packages; `REF-RELEASE-PLEASE-DESIGN`, `REF-RELEASE-PLEASE-CLI`).

Where release-please's 0.2.0 falls short of them (`CHANGELOG.md`): one line per commit, 163 lines in all, 29 under Features and 96 under Documentation; the commits of a feature listed beside the feature's own merge commit (`F10 round shape…`, then `model: the round shape…`); the tooling's commits (`eslint`, `tooling`) among the library's; scopes in bold instead of words; a reference to a commit where a pull request exists; the breaking change in a block of its own, far from its group. GitHub's generated notes (`REF-GH-RELEASE-NOTES`) list merged pull requests by label, which here would be stories and features without their meaning for a user.

## Acceptance criteria

- Given the history since the last tag, when the release is prepared, then the version follows the table of `docs/tooling/releases.md` (`REF-SEMVER`, before 1.0.0 a breaking change or a `feat` raises the minor version, a `fix` the patch) and `package.json`, `package-lock.json` and `CHANGELOG.md` are written by one Rust command.
- Given the commits of the release, when the tool drafts the changelog's entry, then it follows every rule of the table above: one line per user-visible change of the library and the playground, grouped by feature, the tooling, tests, CI and backlog left out; the agent then rewrites each line in the imperative mood before committing; a check refuses an entry that breaks a rule it can check (heading, date, group order, a reference per line, the `**Breaking:**` mark first).
- Given the release asked, when the pull request into `main` is opened, then it holds the commit of the version and the changelog before it merges, its body is the changelog's entry, and the new ADR records how the commit reaches it (on `develop` before the pull request, or on a release branch from `develop`), superseding ADR-0067's decisions 1 to 3.
- Given the merge into `main`, when the release workflow runs, then a Rust command tags the merge commit `vX.Y.Z`, publishes the GitHub release with the changelog's entry as its notes, attaches the test report, and the site is published; no commit is made, and `develop` follows `main` by a fast-forward.
- Given the history from `v0.1.0` to `v0.2.0`, when the tool drafts its entry, then the draft and release-please's section are compared line for line in the story, with the count of lines and words of each.
- Given the story's end, when the repository is searched, then release-please's workflow steps, `release-please-config.json`, `.release-please-manifest.json`, the `release-please--` exceptions of the CI, the branch pattern and the commit-msg hook, and `RELEASE_PLEASE_TOKEN` in the docs are gone, and the docs ask the Product Owner to delete the token's secret.
- Given the release command, when it is timed, then its time to prepare and to publish is stated.

## Files

`tools/release/` (new crate: `prepare`, `check`, `publish`), `package.json` (`release:prepare`), `.github/workflows/release.yml`, `.github/workflows/ci.yml`, `release-please-config.json` and `.release-please-manifest.json` (removed), `CHANGELOG.md` (its header: the format and Semantic Versioning named), `tools/git-hooks/` (the release commit and branch), `docs/tooling/releases.md`, `docs/tooling/git-workflow.md`, `docs/adr/` (a new ADR superseding ADR-0067's decisions 1 to 3), `docs/references.md`.

## Measures (ADR-0069)

`release:prepare` and `release publish` timed on a copy of the repository at `v0.2.0`'s parent (10 runs, release build); the lines and words of the 0.2.0 entry, release-please's against ours.

Before, from GitHub's timings of the 0.2.0 release on 2026-10-14 (SP-023): the `release-please` job on `main` 16 s, the test report 5 s, the site 56 s; release-please's entry for 0.2.0 is 163 lines of changes, 211 lines and 2,018 words with its links.

After, measured by EN-112 on 2026-10-18 on this Mac, the release build, a timer around each process (Perl's `Time::HiRes`), the first run of each series left out, medians of 10:

| Command | History | Median | Runs (ms) |
| --- | --- | --- | --- |
| `release prepare` | `v0.1.0` → 0.2.0, on a clone at `d0f11bd9`, `develop` before 0.2.0 (v0.2.0's parent on `develop`): 23 lines from 186 commits | **103 ms** | 102 to 132 |
| `release prepare` | `v0.2.0` → 0.3.0, on a clone at `develop`'s head `ddef7032`: 57 lines from 753 commits | **150 ms** | 149 to 260 |
| `release publish` | the 0.3.0 merge made in that clone, `gh` played by a shell script (no network) | **40 ms** | 40 to 42 |

In the workflow, `release publish` adds GitHub's four calls (the release looked up, the run listed, the report downloaded, the release made) and the build of the crate on EN-111's cache; it replaces the `release-please` job (16 s on 2026-10-14) and the `test-report` job (5 s), whose own times can only be read at the first release, 0.3.0. The site's job is unchanged.

### 0.2.0 drafted again, against release-please's

The tool run on the history of 0.2.0 (`v0.1.0..d0f11bd9`), then the agent's rewrite, against release-please's section of `CHANGELOG.md`, counted by `wc` from the version's heading to the next one, links included:

| Entry | Lines of changes | Lines | Words |
| --- | --- | --- | --- |
| release-please's (`CHANGELOG.md`) | 163 (29 Features, 96 Documentation, 19 Bug fixes, 16 Refactoring, 1 Performance, 1 Build, 1 breaking block) | 192 | 1,855 |
| the tool's draft | 23 (7 Changed, 14 Added, 2 Fixed) | 34 | 302 |
| the draft rewritten by the agent | 14 (4 Changed, 8 Added, 2 Fixed) | 24 | 236 |

The story's own count, 211 lines and 2,018 words, took the section with the lines around it; counted alike, the ratios hold. Line for line, each line of the draft and the release-please lines it stands for, one per commit:

| Group | Story | Draft line | release-please lines |
| --- | --- | --- | --- |
| Changed | EN-021 | Shared test helpers, one path writer, one pass per corner | 1 |
| Changed | EN-030 | Pass 2: what pass 1 missed, and the duplicates | 1 |
| Changed | EN-029 | Pass 1: name the calculations of the model, io, render and playground | 2 |
| Changed | EN-028 | Pass 1: name the calculations of math and geometry | 1 |
| Changed | EN-026 | Keep only the ways and modes chosen at the validation | 2 |
| Changed | EN-023 | Folders by business domain, files and folders never side by side | 3 |
| Changed | EN-022 | Folders of the library and the playground | 2 |
| Added | VAL-011 | The guided test of F12, thirteen steps | 2 |
| Added | US-027 | A corner beside a curved side keeps its radius | 3 |
| Added | US-028 | Give handles to a point of a regular polygon | 2 |
| Added | US-026 | Smooth a point by a tension, its handles computed | 3 |
| Added | US-025 | Type a side's handles as shares of the side, or paste curve data | 3 |
| Added | US-024 | Type a handle as an angle and a length | 3 |
| Added | US-023 | Type a handle as dx and dy from its point | 2 |
| Added | US-022 | Switch a point to handles and type them as x and y | 3 |
| Added | EN-025 | Cubic sides in contours, path data and the straight box | 5 |
| Added | EN-024 | A point's handles in the free shape model | 2 |
| Added | VAL-012 | The guided test and demo page of F10 | 2 |
| Added | US-030 | Convert a round shape to a free shape, two ways | 1 |
| Added | US-029 | Draw a circle or an ellipse with the Round tool | 2 |
| Added | EN-027 | The round shape in the model, its curve and conversions | 1 |
| Fixed | AUD-012 | No fillet beside a side of length 0, the checks tested | 1 |
| Fixed | AUD-011 | Curves in their own folder, shared zero vector | 2 |

The other 114 lines of release-please's section are left out, by scope: `backlog` 52, `docs` 32, `tooling` 13, `adr` 7, `geometry` 3 (two derivations, and the breaking block, now the first line of its group), `eslint` 2, `model` 1, `deps` 1, and 3 without one (the merge commits of F10 and F12, listed beside their own commits, and the retrospective of E14). One of them, `docs(model): the audit of EN-027 applied, a box of size 0 not converted`, changed the code under a `docs` type: the draft trusts the type, the agent reads the hints. The rewrite merges the three naming passes of F13 into one line, the two folder stories of F11 into one, the three ways of typing a handle into one and the two guided tests into one, and drops the rounded rectangle that US-030 added and VAL-012 removed within the release (`REF-COMMON-CHANGELOG`, 3.4 and 3.5):

```markdown
## [0.2.0] - 2026-10-14

### Changed

- **Breaking:** remove `contourToPathData` from the public API; `filletArc`, `cornerSetbacks` and `turningAngles` take the turning angles or the triples ([`dc83b1b`](https://github.com/AlexandreGallais/synoptic-studio/commit/dc83b1b7110f8872d9beaf41a33677430464e0cc))
- Name every calculation of the library and the playground after what it computes ([#192](https://github.com/AlexandreGallais/synoptic-studio/pull/192))
- Keep only the ways of typing handles and the modes chosen at F12's validation ([#179](https://github.com/AlexandreGallais/synoptic-studio/pull/179))
- Arrange the library's and the playground's modules in folders by business domain ([`8582c6f`](https://github.com/AlexandreGallais/synoptic-studio/commit/8582c6f9486955ab7df61aa6206f79dd2280b666), [`fb07ae2`](https://github.com/AlexandreGallais/synoptic-studio/commit/fb07ae2cf9586a0586243a1581cf3baae78c2b63))

### Added

- Add the round shape, a circle or an ellipse drawn with the Round tool, and convert it to a free shape's points ([#192](https://github.com/AlexandreGallais/synoptic-studio/pull/192))
- Add handles to a free shape's points, drawn as cubic sides in contours, path data and boxes ([#179](https://github.com/AlexandreGallais/synoptic-studio/pull/179))
- Switch a point to handles in Points mode and type them as x and y, dx and dy, or an angle and a length ([#179](https://github.com/AlexandreGallais/synoptic-studio/pull/179))
- Type a side's handles as shares of the side, or paste curve data ([#179](https://github.com/AlexandreGallais/synoptic-studio/pull/179))
- Smooth a point by a tension, its handles computed from its neighbours ([#179](https://github.com/AlexandreGallais/synoptic-studio/pull/179))
- Give handles to a point of a regular polygon, converted to a free shape first ([#179](https://github.com/AlexandreGallais/synoptic-studio/pull/179))
- Keep a corner's radius beside a curved side ([#179](https://github.com/AlexandreGallais/synoptic-studio/pull/179))
- Add the guided tests of F10 and F12 to the playground ([#192](https://github.com/AlexandreGallais/synoptic-studio/pull/192), [#179](https://github.com/AlexandreGallais/synoptic-studio/pull/179))

### Fixed

- Draw no fillet beside a side of length 0 ([#179](https://github.com/AlexandreGallais/synoptic-studio/pull/179))
- Show the radii as drawn next to curves ([#179](https://github.com/AlexandreGallais/synoptic-studio/pull/179))
```

### 0.3.0, from the history since 0.2.0

The notes the tool drafts from `v0.2.0..ddef7032` (`develop` on 2026-10-18): 57 lines, 71 lines and 732 words in all; each line ends with its pull request, `#258` (E15: F05, F14 to F17) for 53 and `#291` (F18) for 4, left out here. F19 and F20 add none: their commits are the tooling's.

```markdown
## [0.3.0] - 2026-10-18

### Changed

- [draft] The HTML layer removed
- [draft] The balanced wrap removed, lines always wrapped automatically
- [draft] A word too long always stays whole
- [draft] The audit of both passes applied, NAM-001 done
- [draft] Name the calculations E15 wrote bare, first pass of NAM-001

### Added

- [draft] Installed fonts read whole before they are listed
- [draft] A short checklist, prepared by `val:prepare`
- [draft] The register of exceptions, and a byte buffer for font collections
- [draft] Only the fonts the editor can use are listed
- [draft] The box typed comes back when the text returns to Box
- [draft] A text put in a box keeps its fitted size
- [draft] X and Y of a turned text show its turned top-left corner
- [draft] Installed fonts read ahead, and only the weights the browser draws
- [draft] Validation on one checklist page
- [draft] A turned text's frames drawn as a turned shape's
- [draft] Values changed fast redrawn without lag
- [draft] Installed fonts read from collections, and faces that failed
- [draft] Fonts embedded only when a file is exported or saved
- [draft] Choose the underline's and the strike-through's colors
- [draft] Whole numbers in the color picker
- [draft] HSL and HSB as integers, one value edited with the exact others
- [draft] Add the guided test of F16, one step per option and decision
- [draft] Type or pick a color and its opacity
- [draft] HSL and HSB from a hexadecimal color, and back
- [draft] A color typed in hexadecimal, kept as six digits and an opacity
- [draft] Import or choose a font, embed it, and turn the HTML layer on
- [draft] The HTML layer of a text in a switch, over its positioned lines
- [draft] Fonts embedded once per document as @font-face rules
- [draft] A font file read, named and kept, and whether it may be embedded
- [draft] See the straight box of a turned text
- [draft] Add the guided test of F14, one step per option and decision
- [draft] A background color behind a text, as a highlighter
- [draft] Put a text in a box with the Text tool
- [draft] A balanced wrap that evens the lines' lengths
- [draft] Lines placed in the box: line height, cap-height trim, vertical alignment
- [draft] The lines a box shows, and the ellipsis
- [draft] A text in a box: its options and its lines wrapped at spaces
- [draft] A turned text grows along its direction and stays in place
- [draft] A turned text keeps its turned top-left corner when its block changes size
- [draft] The bold button gives back the weight before, and sentence case removed
- [draft] Add the guided test of F05, one step per option and decision
- [draft] Turn a text and see its measured block
- [draft] Place a text with the Text tool
- [draft] A text in the model: its characters, its lines and its measured block
- [draft] Font metrics read from a TrueType or OpenType file

### Removed

- [draft] Remove the HTML layer option and its guided steps
- [draft] Remove sentence case, the other cases kept

### Fixed

- [draft] Font list held while the default font is read again
- [draft] The font chosen automatically holds its files
- [draft] Only fonts read and approved, imported ones too
- [draft] The ellipsis on the last line shown holding text
- [draft] Read a large font collection without running out of memory
- [draft] A wrapped text's background band ends with its line
- [draft] A weight such as 450 drawn with the face it is measured with
- [draft] Keep the picker's hue only when it gives the color back
- [draft] Show every line at a zero line height, a space after a no-break space unmeasured
- [draft] Refuse fonts that cannot measure and bound crafted kerning tables
```

Rewritten by the agent: 13 lines, 20 lines and 290 words, `release check` passing. The HTML layer, the balanced wrap, the break of a word at a character and sentence case were added and removed within the release, so no line says either; every fix mends a feature new in 0.3.0 and joins its line; the history holds no breaking change; the version is 0.3.0 by its features:

```markdown
## [0.3.0] - 2026-10-18

### Changed

- Name the calculations of the text and color code after what they compute ([#258](https://github.com/AlexandreGallais/synoptic-studio/pull/258))

### Added

- Add text: place a text with the Text tool, measured from its font's metrics, Inter 4.1 shipped as the default font ([#258](https://github.com/AlexandreGallais/synoptic-studio/pull/258))
- Turn a text about its measured block; X and Y show its turned top-left corner, which stays in place when the text changes size ([#258](https://github.com/AlexandreGallais/synoptic-studio/pull/258))
- Give back the weight held before Bold was pressed ([#258](https://github.com/AlexandreGallais/synoptic-studio/pull/258))
- Put a text in a box: lines wrapped at spaces, a word too long kept whole, the line height, the cap-height trim, the vertical alignment and an ellipsis on the last line shown ([#258](https://github.com/AlexandreGallais/synoptic-studio/pull/258))
- Keep a text's fitted size when Box is chosen and the box typed when it comes back, and put it back with Reset ([#258](https://github.com/AlexandreGallais/synoptic-studio/pull/258))
- Draw a background behind each line of a text, as a highlighter ([#258](https://github.com/AlexandreGallais/synoptic-studio/pull/258))
- Show a turned text's straight box, and its frame as a turned shape's ([#258](https://github.com/AlexandreGallais/synoptic-studio/pull/258))
- Import a font file or read the installed fonts, collections included, list only those the editor can use with the weights the browser draws, and choose one ([#258](https://github.com/AlexandreGallais/synoptic-studio/pull/258), [#291](https://github.com/AlexandreGallais/synoptic-studio/pull/291))
- Embed the fonts a drawing uses in the file exported, once per face, when their licence allows it ([#258](https://github.com/AlexandreGallais/synoptic-studio/pull/258))
- Give a text, its underline and its strike-through colors and opacities, typed in hexadecimal or picked on a plane, shown as HSL and HSB in whole numbers ([#258](https://github.com/AlexandreGallais/synoptic-studio/pull/258))
- Redraw once per animation frame, a text's metrics kept while its inputs stay the same ([#258](https://github.com/AlexandreGallais/synoptic-studio/pull/258))
- Validate an epic on one checklist page of the playground, beside the guided tests of F05, F14 and F16 ([#258](https://github.com/AlexandreGallais/synoptic-studio/pull/258), [#291](https://github.com/AlexandreGallais/synoptic-studio/pull/291))
```

## Tasks

One task = one commit, referenced as `EN-112.Tn`.

- [ ] T1 — The ADR of the release flow, the rules of good notes in `docs/tooling/releases.md` from their sources (3 h)
- [ ] T2 — The version and the changelog's draft from the history, the entry's check; tested on recorded histories (6 h)
- [ ] T3 — The tag, the GitHub release and the test report from `main` by a Rust command; `release.yml` without release-please (4 h)
- [ ] T4 — 0.2.0 drafted again and compared; release-please's files and exceptions removed; docs; times measured (3 h)

## For the main thread

- **Workflows, kept to the release job** (EN-111 beside): in `release.yml`, release-please's `release-pr`, `release-please` and `test-report` jobs became the one `release` job, and the `docs` job's `needs` and `if` follow it; EN-111's `rust-cache` job and the `docs` job's caches are as they were. `ci.yml` and `codeql.yml` are not touched: their four `release-please--` conditions (`ci.yml` `quality`, `history`, `dependencies`; `codeql.yml`) stay, dead since no such branch is made, and `tests/ci-caches.test.ts` (EN-111) asserts two of them. They go together once both stories are merged; until then the criterion "the `release-please--` exceptions of the CI … gone" is met for the branch pattern and the commit-message hook only.
- **For the Product Owner**: delete the secret `RELEASE_PLEASE_TOKEN` (_Settings → Secrets and variables → Actions_) and the token in their account; _Allow GitHub Actions to create and approve pull requests_ is no longer needed (`docs/tooling/ci.md`). ADR-0079 chose option B, the release branch, over A, the commit on `develop`; it is theirs to confirm. F21.AC6 names the pull request "`develop` → `main`": under B its head is `release/x.y.z`, made from `develop`, holding `develop` and the commit of the version.
- **ADR number**: 0079 was free on the feature branch at T1; if EN-109 or EN-111 took it, renumber.
- **The first release, 0.3.0**, is the first real run of `release publish`: its job's time, the tag on the merge commit, the report attached and the site are read then.
- **Left as they were**: the accepted ADRs 0038, 0067 and 0072 (their release lines marked superseded in their status), F18's guide page and checklist lines (VAL-019's record; the checklist is a shared file), the logs and `docs/process/history.md`.
- `Rule-docs: none affected, no rule of docs/domain/ changes`: the story is tooling only.
- **F18.AC1** ("Release pull request on `develop`, only the release on `main`") is superseded by ADR-0079 in its first half; its test in `tests/release-flow.test.ts` keeps what stands, only the release on `main`, tagged `[F18.AC1] [F21.AC6]`. F18's feature file is a shared file: striking the first half, as F15.AC3 was withdrawn, is the main thread's.

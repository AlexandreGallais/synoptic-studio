# Versions and security

## Latest versions

Every package is on its **latest version published at least 7 days ago** (ADR-0072, `REF-NCU-COOLDOWN`). Latest, so that upgrades stay small and frequent; 7 days old, so that a release published minutes ago does not turn a check red and a compromised release has time to be found. `npm run deps:outdated` (`dependencies outdated` of `tools/deps/`, EN-109) checks every version the repository pins in one command, and runs in `check:all`, so at each push:

| Pin | Latest version at least 7 days old | Read from |
| --- | --- | --- |
| `package.json`, `dependencies` and `devDependencies` | the `latest` tag's version once 7 days old, else the greatest version 7 days old; a pre-release only for a pre-release pin, never a deprecated version, as npm-check-updates chose it with `target: "latest"` and `cooldown: 7` | the npm registry (`REF-NPM-PACKAGE-METADATA`) |
| each GitHub Action of `.github/workflows/` | the next major, once its first tag's commit is 7 days old (Product Owner, 2026-10-09) | GitHub's tags and commits (`REF-GH-MATCHING-REFS`, `REF-GH-COMMIT`) |
| `.nvmrc` | the major of the newest Node.js LTS release 7 days old | Node.js's index of releases (`REF-NODE-DIST-INDEX`) |
| `[workspace.dependencies]` of `Cargo.toml` | the greatest version, not yanked, 7 days old | the crates.io index (`REF-CARGO-REGISTRY-INDEX`) |
| each cargo tool of the Rust checks, `cargo-llvm-cov` and `cargo-deny`: the `CARGO_<TOOL>_VERSION` of the workflows, the `cargo install --locked <tool>@<version>` of `package.json`, the workflows and `docs/tooling/`, its row of [Rust tools](./dependencies.md#rust-tools) (EN-121) | the greatest version, not yanked, 7 days old, as a crate; and one version in every place, its row included | the crates.io index |
| each crates.io package of `Cargo.lock` | not newer than 7 days: Cargo picks the newest compatible version by itself, so a lock may hold one published that day; the line gives the `cargo update -p <crate>@<version> --precise <version>` that pins it back to the greatest compatible version 7 days old (`REF-CARGO-SEMVER`) | the crates.io index |

Each finding is one line with the version and its date, and the command exits 1; a source that does not answer is named and the command exits 2, never 0. `curl` reads the registries and Node.js's index, every address in one process, and `gh api` GitHub, with `gh auth`'s token or `GH_TOKEN`; requests carry no personal data, the `User-Agent` names the project's repository.

A pre-release is never adopted, except VitePress 2.0 alpha (ADR-0075, decision 1).

**Upgrading** (Product Owner, 2026-10-12) — an upgrade is read before it is installed, since a check passing says nothing of what a release added:

1. list the upgrades: `npm run deps:outdated`;
2. read what changed from the installed version to the new one: the changelog or the release notes (`npm view <pkg> repository.url`, then `CHANGELOG.md` or `gh release list -R <owner>/<repo>`);
3. bug fixes only: nothing more. New features: adopt what the project can use (a rule, an option) or record why not. A breaking change: read it closely and adapt the configuration in the same commit;
4. `npm install --save-exact <pkg>@<version>` (a crate: its `=x.y.z` in `Cargo.toml`, or `cargo update --precise`), then `npm run check:all`; a linter upgrade may bring rules to decide ([lint](../conventions/lint.md));
5. the commit body says what changed and what was done about it (`build(deps): …`), one commit per package with features or breaking changes.

## Vulnerabilities

- **Any advisory fails**, whatever its severity: `npm run deps:audit` (`npm audit --audit-level=low`) in `check` and `check:all`, before `npm run build` and in the CI; `npm run rust:advisories` for every crate of `Cargo.lock`, unmaintained and yanked ones included. A vulnerable tree can be neither checked nor built.
- **Fixing**: upgrade the direct dependency; otherwise force the transitive one through `overrides` in `package.json`, documented under [Overrides](./dependencies.md#overrides).
- Dependencies' install scripts stay blocked by npm unless approved; none is needed today.

## Packages blocked by an advisory

| Package | Blocked by | Since | Decision |
| --- | --- | --- | --- |
| `stylelint` (+ `stylelint-config-standard-scss`, `stylelint-order`, `stylelint-config-recess-order`) | `braces` ≤ 3.0.3, GHSA-vfj7-8cjw-p6xm (high), no patched version, through `micromatch` | 2026-10-08 | installed when SCSS starts, through a documented audit exception (dev tools only, with a review date) unless `braces` is fixed by then (Product Owner, 2026-10-08) |

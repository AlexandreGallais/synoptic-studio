# Versions and security

## Latest versions

- `npm run deps:outdated` runs npm-check-updates with `.ncurc.cjs`: every package must be on its **latest version published at least 7 days ago** (cooldown, ADR-0072, `REF-NCU-COOLDOWN`), `typescript` included: TypeScript 7 is the only TypeScript since EN-085 (ADR-0075), TypeScript 6's alias gone with typescript-eslint and TypeDoc. Any available upgrade makes the command fail. The cooldown keeps a version published minutes ago from turning a check red (release 0.10.0), and lowers the risk of installing a compromised version (supply-chain attacks, `REF-NCU-COOLDOWN`).
- It runs before every `git push` (`.husky/pre-push`) and in the CI of the pull requests into `main` (`check:all`).
- **Upgrading** (Product Owner, 2026-10-12):
  1. list the upgrades: `npx ncu`;
  2. **read what changed** for each package, from the installed version to the new one: its changelog or GitHub release notes (`npm view <pkg> repository.url`, then the repository's `CHANGELOG.md` or `gh release list -R <owner>/<repo>`);
  3. a release that only fixes bugs needs nothing more; a release with **new features** is read for what this project could use (a new rule, a new option): adopt it or record why not; a **breaking change** (major version, or a note saying so) is read closely and the configuration is adapted in the same commit;
  4. `npx ncu -u <pkg>`, `npm install`, `npm run check:all`; an oxlint upgrade may add rules: decide them in `.oxlintrc.json` (see [lint](../conventions/lint.md));
  5. the commit body says what changed and what was done about it (`build(deps): …`), one commit per package with features or breaking changes.
- `npm run deps:tools` (`scripts/tool-versions.ts`) does the same for the tools outside npm: every GitHub Action of `.github/workflows/` must be on its latest major version (a tag `vN+1` fails the check), and `.nvmrc` on the latest Node.js LTS major (official release index). It runs in `check:all`, with `GITHUB_TOKEN` in CI and `gh auth token` locally. The check fails as soon as a new major is out, and the agent upgrades it after reading its release notes (Product Owner decision, 2026-10-09: every tool, not only npm packages).
- Pre-releases are not proposed by npm-check-updates; VitePress 2.0 alpha is the documented exception (ADR-0009).

## Vulnerabilities

- `npm run deps:audit` (`npm audit --audit-level=low`): **any** advisory fails.
- It runs in `npm run check` (so before every push), before `npm run build`, and in the CI into `main`: a vulnerable dependency tree cannot be checked nor built.
- The Rust crates: `npm run rust:advisories` (`cargo deny --offline check advisories`, EN-069) fails on any RustSec advisory of a crate of the root `Cargo.lock`, an unmaintained or a yanked one included; it runs in `check:rust`, from the local copy of the database (`cargo deny fetch db`), and in the CI into `main` after fetching it. The workspace has no crates.io dependency today, so only a new one can bring an advisory.
- Fixing: upgrade the direct dependency; otherwise force the transitive one through `overrides` in `package.json` and document it in [dependencies](./dependencies.md#overrides).
- Install scripts of dependencies are blocked by npm unless approved (`npm install-scripts`); none is needed today.

## Packages blocked by an advisory

| Package | Blocked by | Since | Decision |
| --- | --- | --- | --- |
| `stylelint` (+ `stylelint-config-standard-scss`, `stylelint-order`, `stylelint-config-recess-order`) | `braces` ≤ 3.0.3, GHSA-vfj7-8cjw-p6xm (high, stack-exhaustion DoS), no patched version, pulled through `micromatch` | 2026-10-08 | not installed. Product Owner decision (2026-10-08): install it when SCSS starts, with `stylelint-order` and `stylelint-config-recess-order`, through a documented audit exception list (dev tools only, with a review date) unless `braces` is fixed by then |
| `markdownlint-cli2` | same advisory | 2026-10-08 | replaced by `@eslint/markdown`, itself replaced by the project's linter (EN-077) |

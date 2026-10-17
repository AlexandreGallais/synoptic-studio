# Versions and security

## Latest versions

Every package is on its **latest version published at least 7 days ago** (ADR-0072, `REF-NCU-COOLDOWN`). Latest, so that upgrades stay small and frequent; 7 days old, so that a release published minutes ago does not turn a check red and a compromised release has time to be found. `npm run deps:outdated` (npm-check-updates, `.ncurc.cjs`) fails on any available upgrade; it runs in `check:all`, so at each push and in the CI into `main`.

`npm run deps:tools` holds the tools outside npm to the same rule: every GitHub Action of `.github/workflows/` on its latest major, `.nvmrc` on the latest Node.js LTS major (Product Owner, 2026-10-09). The Rust tools of the checks are looked at by hand ([dependencies](./dependencies.md#rust-tools)).

A pre-release is never adopted, except VitePress 2.0 alpha (ADR-0075, decision 1).

**Upgrading** (Product Owner, 2026-10-12) — an upgrade is read before it is installed, since a check passing says nothing of what a release added:

1. list the upgrades: `npx ncu`;
2. read what changed from the installed version to the new one: the changelog or the release notes (`npm view <pkg> repository.url`, then `CHANGELOG.md` or `gh release list -R <owner>/<repo>`);
3. bug fixes only: nothing more. New features: adopt what the project can use (a rule, an option) or record why not. A breaking change: read it closely and adapt the configuration in the same commit;
4. `npx ncu -u <pkg>`, `npm install`, `npm run check:all`; a linter upgrade may bring rules to decide ([lint](../conventions/lint.md));
5. the commit body says what changed and what was done about it (`build(deps): …`), one commit per package with features or breaking changes.

## Vulnerabilities

- **Any advisory fails**, whatever its severity: `npm run deps:audit` (`npm audit --audit-level=low`) in `check` and `check:all`, before `npm run build` and in the CI; `npm run rust:advisories` for every crate of `Cargo.lock`, unmaintained and yanked ones included. A vulnerable tree can be neither checked nor built.
- **Fixing**: upgrade the direct dependency; otherwise force the transitive one through `overrides` in `package.json`, documented under [Overrides](./dependencies.md#overrides).
- Dependencies' install scripts stay blocked by npm unless approved; none is needed today.

## Packages blocked by an advisory

| Package | Blocked by | Since | Decision |
| --- | --- | --- | --- |
| `stylelint` (+ `stylelint-config-standard-scss`, `stylelint-order`, `stylelint-config-recess-order`) | `braces` ≤ 3.0.3, GHSA-vfj7-8cjw-p6xm (high), no patched version, through `micromatch` | 2026-10-08 | installed when SCSS starts, through a documented audit exception (dev tools only, with a review date) unless `braces` is fixed by then (Product Owner, 2026-10-08) |

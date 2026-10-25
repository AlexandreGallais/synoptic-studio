# ADR-0019 — Kebab-case file names, one export per file

**Status**: Accepted — supersedes ADR-0015, removed since (ADR-0101, which lists the rules of it this ADR keeps)

## Context

ADR-0015 named each module exactly like its export (`formatSvgNumber.ts`). The Product Owner asked for every file name in kebab-case, as in the Angular style guide (`REF-ANGULAR-STYLE`: `UserProfile` → `user-profile.ts`).

## Decision

Everything of ADR-0015 stays, except the file name:

- a module exports exactly one value (or one type), with `export <declaration>`;
- a type may sit next to the value only if it appears in its declaration;
- the file is named **the kebab-case of the export**: `formatSvgNumber` → `format-svg-number.ts`, `SVG_DECIMALS` → `svg-decimals.ts`, `Point` → `point.ts`;
- tests: `format-svg-number.test.ts`.

Enforced by `local/one-export-per-file` (name check) and `unicorn/filename-case` (kebab-case everywhere, folders included).

## References

ADR-0101 (ADR-0015, removed), `REF-ANGULAR-STYLE`, `REF-AIRBNB-STYLE`

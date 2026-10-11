---
paths:
  - "src/**"
  - "playground/**"
---

# Library and playground pitfalls

- Importing from another folder = importing **the folder** (`../geometry`); `npm run fix` corrects the path and updates the folder's `index.ts`.
- A new layer (`src/<layer>/`) exists only with a first module and its `index.ts`, re-exported by `src/index.ts`.
- Barrels: the autofix writes `export type *` for a folder holding only types and never switches back to `export *` once it holds values; `barrels.test.ts` then fails — replace the line by `export *`.
- `noUncheckedIndexedAccess`: an index read is `T | undefined`; an unreachable fallback branch breaks the 100 % coverage. Prefer an explicit, testable fallback (see `cyclicItem`).
- Module constants: `UPPER_CASE` and documented; type members documented too (TypeDoc fails otherwise).
- JSDoc description = sentences; `@param` / `@returns` = fragments without final period. `@kind` is declared in `tsdoc.json` (`jsdoc/check-values` off on purpose).
- Procedure verbs: `eslint/settings/verbs.ts` (add, sorted). Per-kind limits: `eslint/settings/kinds.ts` (change only with an ADR).
- Tests: examples with hand-computed values justified in a comment, then properties with `test.prop({ … })` from `@fast-check/vitest` (record form: `max-params` is 3). `toBe` compares with `Object.is`, so `-0` differs from `0`: use `toBeCloseTo` for computed numbers.
- DOM tests start with `// @vitest-environment happy-dom`.
- Run `npx tsc --noEmit` before each commit: Vitest strips types, and the unused-imports autofix drops a type import written before the code that uses it (EN-005).
- Zero vectors and signed zeros (ADR-0025): `atan2(+0, −0) = π`, `Math.sign(−0) = −0`, and `toEqual` tells `−0` from `0`. State the intended result for zero-length edges explicitly (see `turningAngle`, `unit`).
- More than three parameters: group them in a named type (`CornerPoints`); tests with many columns use `it.each` over objects.
- Playground visibility: page rules such as `label { display: block }` override the `hidden` attribute; keep the `[hidden] { display: none }` rule, and test what is displayed (`getComputedStyle`), the page head loaded (US-007).
- A test refusing a value outside a string union (data read from outside) needs a cast: `// eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- <reason>` (US-009). A complexity limit in a test callback counts `?.` and `??`: destructure with one fallback instead.
- Mutation checks by hand: save the file's content and copy it back; `git checkout` neither restores an untracked file nor keeps uncommitted work in a tracked one (F07).
- happy-dom: `:checked` follows the `checked` attribute, not the state, and the lint autofix wraps selector values in `CSS.escape` (a space becomes `\ `, which happy-dom does not match): find radio buttons by their `checked` and `value` properties (US-010).

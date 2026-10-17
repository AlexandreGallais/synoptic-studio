---
paths:
  - "src/**"
  - "playground/**"
---

# Library and playground pitfalls

Each is a trap met in this code, with the story that met it. The rules themselves: `docs/conventions/`.

## Modules and documentation

- Importing from another folder imports **the folder** (`../geometry`); `npm run fix` corrects the path and the folder's `index.ts`.
- A new module goes into the folder of its business domain (ADR-0043, `local/folder-layout`), and a new domain or layer (`src/<layer>/`) exists only with its first module and its `index.ts`, a layer re-exported by `src/index.ts`.
- The barrel autofix writes `export type *` for a folder holding only types and never switches back once it holds values: `barrels.test.ts` then fails; write `export *`.
- Module constants are `UPPER_CASE` and documented, and so are type members: the API pages' generator (`tools/api-pages`) fails otherwise. A JSDoc description is sentences, `@param` and `@returns` fragments without a final period; `@kind` is a custom tag, its own section of an API page.
- Procedure verbs and the limits per kind live in `tools/lint/src/settings.rs`: a verb is added sorted; a limit changes only with an ADR.
- More than three parameters are grouped in a named type (`CornerPoints`).
- The immutability rules read the written code (EN-076): in the core, a parameter or declared type from the standard library or a package is written `Readonly<DataView>`, while a type of the core needs nothing more; a module constant holding a function returned by a call has its function type written, or `naming-convention` asks for `UPPER_CASE`. `npm run fix` writes what is missing.
- A required field added to a model type (no optional field, ADR-0016): the type check lists every literal to complete; in tests add it beside the other fields, and let a property draw it (`fc.boolean()`) where both values matter (EN-017).
- Run `npm run fix` before each commit: it reports the type errors with the lint (EN-091), which Vitest does not, since it strips types. Write the code before its type import: the unused-imports autofix drops an import nothing uses yet (EN-005).

## Numbers

- An index read is `T | undefined` (`noUncheckedIndexedAccess`): an unreachable fallback breaks the 100 % coverage, so write an explicit, testable fallback (`cyclicItem`).
- Zero vectors and signed zeros (ADR-0025): `atan2(+0, −0) = π`, `Math.sign(−0) = −0`, and `toEqual` and `toBe` tell `−0` from `0`. State the intended result for a zero-length edge (`turningAngle`, `unit`); compare computed numbers with `toBeCloseTo`.
- Scale to an integer size by multiplying first, `o + size · (p − o) / span`: `(29 / 7) · 7` is 29.000000000000004, `29 · 7 / 7` is 29 (`DERIV-free-shape-stretch`).
- A bisection in doubles stops at one unit in the last place, before its nominal step count, so a bound it returns may sit that far on the wrong side of a `>=`: say "within a rounding error", never "at least". A clamp before a bisection on a monotone predicate is unobservable, so a mutation survives it: drop it or say so (EN-017).
- Never change the floats of a shared math function for a new caller: `npm run golden:check` guards the results of E01 and E14 bit for bit, and a 10⁻¹⁴ drift fails it; add a named function instead (`nonNegativeRemainder` beside `modulo`, EN-038). The golden file is written only by the main thread, before a refactor; `golden:write` refuses in a worktree (EN-052).

## Tests

- Examples with hand-computed values justified in a comment, then properties with `test.prop({ … })` of `@fast-check/vitest`, in record form since `max-params` is 3; many columns: `it.each` over objects.
- `expect.closeTo` inside `toMatchObject` or `toEqual` is an `any` for the lint: assert the numbers one by one with `toBeCloseTo` (EN-010).
- A test refusing a value outside a string union needs a cast, its directive on its own line above: `// oxlint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- <reason>` (US-009). In a test callback the complexity limit counts `?.` and `??`: destructure with one fallback.
- A mutation by hand is undone by copying the saved content back: `git checkout` neither restores an untracked file nor keeps the uncommitted work of a tracked one (F07).

## The DOM and happy-dom

- A DOM test starts with `// @vitest-environment happy-dom`.
- happy-dom differs from a browser: `:checked` follows the attribute, not the state, and the lint autofix escapes selector values (`CSS.escape`) in a way it does not match, so find radio buttons by their `checked` and `value` properties (US-010); a `<select>` moved from a `DOMParser` document loses its selected option, so choose it again (US-031); it keeps the line break after `<textarea>` that a browser drops, so the text opens with an empty line (US-051).
- The autofix turns `removeAttribute("data-x")` into `delete el.dataset.x`, which the type check refuses (TS4111): write `delete el.dataset["x"]` (US-011).

## The playground

- Page rules such as `label { display: block }`, and a rule by id with `display: grid` or `flex`, override the `hidden` attribute: keep the `[hidden] { display: none !important }` rule and test what is displayed (`getComputedStyle`), the page head loaded (US-007, US-031).
- A control acting on the shape is tested on a **refused** shape too: hide it or guard its handler, or a `NaN` is stored and crashes later redraws (US-014).
- A control that changes one value of a kept free shape writes only that value: `writeStoredFreeShape` also rewrites the size and radii from the stored shape, undoing what was typed (US-018).
- Frames and dashes are fills (a ring with `fill-rule: evenodd`, thin rectangles), never a native stroke; a box too small for its frame gets none (US-018).
- Its limits (`max-lines` 200, `import/max-dependencies` 10, procedure statements 15) are met by splitting by role into one-export files (`read-shape`, `show-shape`, `write-*`), and by passing `update` as a callback rather than importing the page's redraw (EN-010).
- A decision kept on a paused feature branch is not on `develop` yet: cite it only from that branch, or say it is paused (F08).

---
paths:
  - "src/**"
  - "playground/**"
---

# Library and playground

The traps of this code that a check does not catch by itself. The conventions: the records' `conventions` (areas architecture, files, functions, naming, imports, documentation, typescript, testing) and `lint_rules`.

## Modules

- Another folder is imported by **the folder** (`../geometry`); `npm run fix` corrects the path and the folder's `index.ts`. A barrel line `export type *` stays so once its module holds values: write `export *`.
- A new module goes into its business domain's folder (ADR-0043); a new domain or layer exists only with its first module and its `index.ts`, a layer re-exported by `src/index.ts`.
- Module constants are `UPPER_CASE` and documented, and so are type members (`tools/api-pages` fails otherwise). A description is sentences; `@param` and `@returns` fragments without a final period.
- Procedure verbs and the limits per kind live in `tools/lint/src/settings.rs`: a verb added sorted, a limit changed only with a decision. More than three parameters are grouped in a named type.
- In the core, a parameter or declared type of the standard library or a package is written `Readonly<…>`; `npm run fix` writes what is missing.
- A required field added to a model type (no optional field, ADR-0016): the type check lists every literal to complete; a property draws it where both values matter.
- `npm run fix` before each commit: it reports the type errors, which Vitest strips. Write the code before its type import, or the autofix drops the import.

## Numbers

- An index read is `T | undefined`: write an explicit, testable fallback, not an unreachable one.
- Zero vectors and signed zeros: `atan2(+0, −0) = π`, `Math.sign(−0) = −0`, `toBe` tells `−0` from `0`; state the result for a zero-length edge, compare computed numbers with `toBeCloseTo`.
- Scale to an integer by multiplying first, `o + size · (p − o) / span`: `29 · 7 / 7` is 29, `(29 / 7) · 7` is not.
- A bisection in doubles stops at one unit in the last place: say "within a rounding error", never "at least".
- Never change the floats of a shared math function for a new caller: the golden check (`target/release/heavy-lock npx vitest run tests/golden-results.test.ts --mode golden-check`, run by the pre-push hook once a golden file is written) guards the results bit for bit; add a named function instead. The golden file is written only by the main thread, before a refactor.

## Tests

- Examples with hand-computed values justified in a comment, then properties with `test.prop({ … })` of `@fast-check/vitest`; many columns: `it.each` over objects.
- `expect.closeTo` inside `toMatchObject` is an `any` for the lint: assert numbers one by one with `toBeCloseTo`.
- A test refusing a value outside a string union needs a cast, its directive on its own line above: `// oxlint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- <reason>`, and its row in the records' `lint_exceptions`, written into `tests/data/lint-exceptions.json` by `target/release/knowledge export-data`.
- A mutation by hand is undone by copying the saved content back, never by `git checkout`.
- A DOM test starts with `// @vitest-environment happy-dom`; happy-dom's `:checked` follows the attribute, so find radio buttons by their `checked` and `value` properties; a `<select>` moved from a `DOMParser` document loses its selected option.
- The autofix turns `removeAttribute("data-x")` into `delete el.dataset.x`, which the type check refuses: write `delete el.dataset["x"]`.

## The playground

- Its design follows the Figma design system named in E49's decisions (`knowledge show E49 Decisions`), read at each playground feature; a remark on it is a Figma comment, never an edit. It becomes an Angular app in E50, following Angular's usage there.
- In the drawing's SVG, frames and dashes are fills, never a native stroke; only the helps of the layer above (`#overlay`) are a native stroke, divided by the view's scale (ADR-0099).
- A control acting on the shape is tested on a **refused** shape too, or a `NaN` is stored.
- A rule with `display` overrides the `hidden` attribute: keep `[hidden] { display: none !important }` and test what is displayed.
- A control that changes one value of a kept free shape writes only that value: `writeStoredFreeShape` rewrites the size and radii too.
- Its limits (`max-lines` 200, `import/max-dependencies` 10, procedure statements 15) are met by splitting by role into one-export files, and by passing `update` as a callback rather than importing the page's redraw.

# Tests

## Tool

Vitest, without globals (`import { describe, expect, it } from "vitest"`). Files `<module>.test.ts` **next to** the tested module; repository-wide tests in `tests/`.

## Rules

- `math` / `geometry`: **tests first** (`AGENTS.md`, work) — nominal case, degenerate cases, reference values.
- Every non-obvious expected value is **computed by hand** and justified in a comment above the assertion:

```ts
it("returns the height of an equilateral triangle", () => {
  // 100 × √3 / 2 (DERIV-regular-polygon-fit, n = 3).
  expect(equilateralHeight(100)).toBeCloseTo(86.602_540_378, 9);
});
```

- When the expected value cannot be computed by hand: **stop** (research request or derivation to validate).
- Float comparisons: `toBeCloseTo` with a justified number of decimals, or the project's `EPSILON = 1e-9` (Q10, `src/math/precision/epsilon.ts`).
- Shared test data of `src/` lives in `tests/geometry-fixtures.ts`, imported from `tests` (EN-021): the rectangle of US-001 as points or corners, a free shape from points and radii side by side, the shoelace area, `expectPoints` with its number of decimals, integer points and triangles for properties. A test needing one of these never writes its own copy; test data never enters `src/`, whose barrels are the public API.
- A test font file is read once per test run (EN-089): `tests/font-files-setup.ts`, Vitest's global setup, reads `tests/inter-regular.ttf` and hands its bytes to every test file; `interRegularFile()` gives a fresh copy of them at each call, and one test, `tests/font-files.test.ts`, proves the reading by its length and digest. Tables built from it (`sharedSansTables()`) are built at their first call (`builtOnce`), frozen, and shared by the files of a worker ([shared fixtures](#shared-fixtures)). A test of a parser or a builder still calls it on those bytes: only what was read is shared, never a result of the function under test.
- DOM tests (`src/render/`, `src/interaction/` when it exists, `playground/`): start the file with `// @vitest-environment happy-dom`; the shell stays thin, the logic lives in the core.
- Playground tests (`playground/<panel>/*.test.ts`, happy-dom, in the folder of the panel they exercise, such as `points/points-list`, `shape/radius-field`):
  - they load the page as served (`index.html?raw`), mount it, act on it and read what it shows, through the actions of `playground/user-actions/` only; an element the test names and the page lacks fails it at once;
  - the actions select elements on their own (`selectPageElement`), never through the playground's `selectElement`: a test does not lean on the code it checks;
  - they set state through the page as a user does (a radius per point typed in Points mode), never in the page's attributes;
  - they carry the criteria only the playground can show (`[F01.AC1]`, the display part of `[F01.AC2]`, `[F02.AC4]`, the playground side of `[F02.AC1]`–`[F02.AC3]`);
  - the playground has a coverage floor of its own ([Coverage](#coverage)).

## One claim, one place (ADR-0070)

Test, never over-test: each claim is proven once, at the lowest level that can show it, and what is slow to prepare is prepared once.

| Claim | Proven in | Not proven again in |
| --- | --- | --- |
| what a `math` or `geometry` function gives | its own test file: hand-computed examples, then properties (ADR-0022) | the tests of the functions that call it |
| what a composing function adds (placement, anchor, order, a guard) | its own test file, the functions it calls serving as reference, their values proven in their own files | the tests of its callers |
| what a parser reads from bytes | its own test file, on shared bytes (EN-089) | the tests of the builders that use its result |
| what the page does after an action | the test of its panel (`playground/<panel>/`) | another panel's test, a walk through a whole test card |
| what a guided step of a checklist states | its epic's steps test: the state the step opens on and the result it states | — |
| a rule of the repository | `tests/`, or the tool's own tests in Rust | — |

- **Two criteria proven by one test**: two tags in one title (`[F02.AC1] [F07.AC1] …`, ADR-0027), never two copies of the test.
- **An example is a value computed by hand.** A test that compares a function with the same oracle as a property beside it is merged into the property, its generator widened to cover the example's input (an anchor, an angle); the bounds of a range stay as examples.
- **A wider property contains a narrower one** (equal radii inside any radii): one property, its generator drawing both cases (`fc.boolean()` choosing equal radii).
- **A Product Owner test card** is walked by the guided steps of its feature's `VAL`, not by a test repeating the tests of its panel one after the other; a walk stays only for the steps no other test proves.
- **A removed or merged test** is checked by `npm run mutate` on the code it covered (`-- npx vitest run {test}` for the file's own tests) and by the coverage gates.

### Shared fixtures

- A **file** is read once per run: the global setup reads it and `provide`s its bytes; `inject` gives a fresh copy to each call (`tests/font-files-setup.ts`, EN-089).
- A **value slow to build** — tables built from a font, a sample of 731 000 colors, the backlog's files or the source files read by a repository test — is built at its first use and shared after it: `builtOnce` (`tests/built-once.ts`), the value frozen (`Object.freeze`), so that no test changes it for the next. Never at a module's import: the barrel `tests` is imported by most test files, which would all pay for it.
- A fixture holds **inputs**, never a result of the function under test: a test of a parser still calls the parser.
- One test proves the preparation (`tests/font-files.test.ts` for the font file); the others use it.

### Test helpers

A helper of `tests/` or `playground/user-actions/` may build an input, act on the page as a user, read what the page shows, or compare (`expectPoints`, `partsDigest`). It never computes an expected value with the code under test, never holds a branch of that code's logic, and never asserts a claim a test does not name.

### Time limits

Vitest's limit of 5 s per test guards against a hang, never measures speed. A test known to be long gets a longer limit with its reason in a comment (`{ timeout: 20_000 }`, the work it does alone, four times over for a loaded `check:all`); a test that is merely slow is made faster, not given a longer limit.

### Isolation

Two Vitest projects (`vite.config.ts`): `shared`, where the files of a worker share the modules they load (`isolate: false`) — `src/`, `tests/`, `scripts/`, `eslint/`, the hooks and the docs, which keep no state between files — and `isolated`, the playground, whose page keeps its state in its modules and whose tests mock the library. A test of `shared` restores what it changes (`process.env`, a global stub) and declares no module mock (`vi.mock`); one that needs either moves to `isolated`.

## Properties (ADR-0022)

After the hand-computed examples, `math` and `geometry` functions with a stateable invariant get **properties** (`fast-check`, `test.prop` from `@fast-check/vitest`):

```ts
/** Vector with safe-integer coordinates. */
const VECTOR = fc.record({ x: fc.integer(), y: fc.integer() });

test.prop({ a: VECTOR, b: VECTOR })("is symmetric", ({ a, b }) => {
  expect(dot(a, b)).toBe(dot(b, a));
});
```

- Examples carry the sourced values; properties widen the input space. Neither replaces the other.
- Typical properties: symmetry, antisymmetry, bounds, invariance (rotation, translation, direction), round trip, agreement with a simpler formula on a sub-domain.
- A failure prints its seed and a shrunk counterexample: replay with `test.prop(…, { seed })`, then add the counterexample as an example test.
- `toBe` compares with `Object.is` (`-0` differs from `0`): computed floats use `toBeCloseTo` with a justified precision.

## Tests that prove a requirement (ADR-0027)

A test proving a feature's acceptance criterion carries its identifier in its title, so that the criterion → test link is searchable and checked:

```ts
it("[F01.AC3] clamps a 100 × 25 rectangle with radius 100 to an effective radius of 12.5", () => {
```

`tests/backlog.test.ts` fails when a `done` feature has a criterion no test names. The `VAL` story tags or writes these tests.

## Test strategy

| Item | Rule |
| --- | --- |
| Levels | examples computed by hand; properties (fast-check); criterion tests `[F01.ACn]`; the epic's checklist page in the playground (`VAL`, `REV`); regression and end-to-end suites with the applications (E12) |
| Environments | the core in Node.js without a DOM; `render` with happy-dom; browsers for end-to-end (later) |
| Independence | the `auditor`, `verifier` and `math-expert` agents re-derive, recompute and try mutations; the Product Owner tries the checklists |
| Success | every test green, 100 % coverage of `src/`, no `.skip` / `.only`, on every push and every release |
| Regression | every test runs before every push and on the pull request into `main`; a done feature's criterion tests are never deleted, only changed by a story citing the criterion |
| Evidence | each release carries its JUnit report (`test-report.xml`) as a release asset |

## Coverage

`npm run test` measures the coverage of `src/` and `playground/` (`@vitest/coverage-v8`). It fails below **100 %** of lines, branches, functions and statements in `src/`, and below the playground's floor (lines 98, statements 98, functions 100, branches 89 on 2026-10-13). Barrels, tests, the playground's entry and its test actions (`playground/user-actions/`) are excluded. An uncovered branch is either a missing test or dead code.

**Fitness functions** (`tests/fitness.test.ts`, `REF-FITNESS-FUNCTIONS`): every function exported by `src/math/` and `src/geometry/` has a property (`test.prop`) in its own test file, the one beside it; a property written for another function, even one calling it, does not count (EN-020). Repository-wide tests: [files.md](./files.md#names).

## Checking the tests themselves

- A test that reads files (backlog, bibliography, barrels) is **mutation-checked** once: break the input on purpose and see it fail — a test that cannot fail checks nothing.
- Feature audits mutate each function once (flip a sign, swap an operator) and check that a test fails ([audit.md](./audit.md) A4). The mutation tool StrykerJS is deferred: with Vitest 5 it runs no test against mutants (ADR-0022).

## Later: regression and end-to-end tests

Not now (ADR-0020): while the library is being built, unit tests and feature audits are the safety net. Regression suites and end-to-end tests of the applications come when the applications exist.

## ESLint relaxations in tests (`eslint/scopes/tests.ts`)

Literal numbers allowed, free function length, callback nesting up to 4, dev dependencies allowed, no `@kind`. Every other rule applies, including naming and JSDoc of module constants.

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
- Float comparisons: `toBeCloseTo` with a justified number of decimals, or the project's `EPSILON = 1e-9` (Q10, `src/math/epsilon.ts`).
- Shared test data of `src/` lives in `tests/geometry-fixtures.ts`, imported from `tests` (EN-021): the rectangle of US-001 as points or corners, a free shape from points and radii side by side, the shoelace area, `expectPoints` with its number of decimals, integer points and triangles for properties. A test needing one of these never writes its own copy; test data never enters `src/`, whose barrels are the public API.
- DOM tests (`src/render/`, `src/interaction/` when it exists, `playground/`): start the file with `// @vitest-environment happy-dom`; the shell stays thin, the logic lives in the core.
- The playground has its own tests (`playground/*.test.ts`, happy-dom: `mount-playground`, then one file per area such as `points-list`, `radius-field`): it loads the page as served (`index.html?raw`), mounts it, types values and reads what the page shows. It carries the criteria that only the playground can show (`[F01.AC1]`, the display part of `[F01.AC2]`, `[F02.AC4]`, and the playground side of `[F02.AC1]`–`[F02.AC3]`). The playground has a coverage floor of its own, below 100 %: `PLAYGROUND_COVERAGE` in `vite.config.ts`, which only rises. They drive the page through `playground/user-actions/` (EN-021), never a copy of their own: `openPage`, `openOnStep`, `typeValue`, `typeAndEnter`, `chooseOption`, `clickElement`, `clickWithKeys`, `clickPoint`, `readText`, `readValue`, `readPointRows`; an element the test names and the page lacks fails it at once. State is set through the page as a user sets it (a radius per point typed in Points mode), never written into the page's attributes.

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

| Item         | Rule                                                                                                                                                                                         |
| ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Levels       | examples computed by hand; properties (fast-check); criterion tests `[F01.ACn]`; guided tests in the playground (`VAL`, `REV`); regression and end-to-end suites with the applications (E12) |
| Environments | the core in Node.js without a DOM; `render` with happy-dom; browsers for end-to-end (later)                                                                                                  |
| Independence | the `auditor`, `verifier` and `math-expert` agents re-derive, recompute and try mutations; the Product Owner runs the guided tests                                                           |
| Success      | every test green, 100 % coverage of `src/`, no `.skip` / `.only`, on every push and every release                                                                                            |
| Regression   | every test runs before every push and on the pull request into `main`; a done feature's criterion tests are never deleted, only changed by a story citing the criterion                      |
| Evidence     | each release carries its JUnit report (`test-report.xml`) as a release asset                                                                                                                 |

## Coverage

`npm run test` measures the coverage of `src/` and `playground/` (`@vitest/coverage-v8`). It fails below **100 %** of lines, branches, functions and statements in `src/`, and below the playground's floor (lines 98, statements 98, functions 100, branches 89 on 2026-10-13). Barrels, tests, the playground's entry and its test actions (`playground/user-actions/`) are excluded. An uncovered branch is either a missing test or dead code.

**Fitness functions** (`tests/fitness.test.ts`, `REF-FITNESS-FUNCTIONS`): every function exported by `src/math/` and `src/geometry/` has a property (`test.prop`) in its own test file, the one beside it; a property written for another function, even one calling it, does not count (EN-020). Repository-wide tests live in `tests/`: backlog, bibliography, barrels, dependencies, fitness, and the shared fixtures with their own test.

## Checking the tests themselves

- A test that reads files (backlog, bibliography, barrels) is **mutation-checked** once: break the input on purpose and see it fail — a test that cannot fail checks nothing.
- Feature audits mutate each function once (flip a sign, swap an operator) and check that a test fails ([audit.md](./audit.md) A4). The mutation tool StrykerJS is deferred: with Vitest 5 it runs no test against mutants (ADR-0022).

## Later: regression and end-to-end tests

Not now (ADR-0020): while the library is being built, unit tests and feature audits are the safety net. Regression suites and end-to-end tests of the applications come when the applications exist.

## ESLint relaxations in tests (`eslint/scopes/tests.ts`)

Literal numbers allowed, free function length, callback nesting up to 4, dev dependencies allowed, no `@kind`. Every other rule applies, including naming and JSDoc of module constants.

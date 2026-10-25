# Tests

A test proves a claim with a value someone can check without the code. Each claim is proven once, at the lowest level that can show it.

## Tool and place

Tests run on Vitest, without globals (`import { describe, expect, it } from "vitest"`). A test file `<module>.test.ts` sits beside its module, and the tests of the repository as a whole sit in `tests/`.

## Expected values

- Write `math` and `geometry` tests first (ADR-0022): the nominal case, the degenerate cases, then the reference values.
- Compute every non-obvious expected value by hand, and show the computation in a comment above the assertion. A value copied from the code's output proves nothing.

```ts
it("returns the height of an equilateral triangle", () => {
  // 100 × √3 / 2 (DERIV-regular-polygon-fit, n = 3).
  expect(equilateralHeight(100)).toBeCloseTo(86.602_540_378, 9);
});
```

- When an expected value cannot be computed by hand, stop: propose a derivation to validate, or a research request.
- Compare computed floats with `toBeCloseTo` and a justified number of decimals, or with the project's `EPSILON = 1e-9` (`src/math/precision/epsilon.ts`). `toBe` compares with `Object.is`, so `-0` differs from `0`.

## Properties (ADR-0022)

After the hand-computed examples, give every `math` and `geometry` function that has a stateable invariant its properties (`fast-check`, `test.prop` from `@fast-check/vitest`). Examples carry the sourced values and properties widen the input space: neither replaces the other.

```ts
/** Vector with safe-integer coordinates. */
const VECTOR = fc.record({ x: fc.integer(), y: fc.integer() });

test.prop({ a: VECTOR, b: VECTOR })("is symmetric", ({ a, b }) => {
  expect(dot(a, b)).toBe(dot(b, a));
});
```

- Typical properties: symmetry, antisymmetry, bounds, invariance under rotation, translation or direction, a round trip, agreement with a simpler formula on a sub-domain.
- A failure prints its seed and a shrunk counterexample. Replay it with `test.prop(…, { seed })`, then add the counterexample as an example test.
- `tests/fitness.test.ts` checks that every function exported by `src/math/` and `src/geometry/` has a property in its own test file. A property written for a caller does not count (EN-020, `REF-FITNESS-FUNCTIONS`).

## One claim, one place (ADR-0070)

Test, never over-test: a claim proven twice doubles the cost of every change to it and proves nothing more.

| Claim | Proven in | Not proven again in |
| --- | --- | --- |
| what a `math` or `geometry` function gives | its own test file: hand-computed examples, then properties | the tests of the functions that call it |
| what a composing function adds (placement, anchor, order, a guard) | its own test file, the functions it calls serving as reference | the tests of its callers |
| what a parser reads from bytes | its own test file, on shared bytes | the tests of the builders that use its result |
| what the page does after an action | the test of its panel (`playground/<panel>/`) | another panel's test, a walk through a whole test card |
| what a guided step of a checklist states | its epic's steps test: the state the step opens on and the result it states | — |
| a rule of the repository | `tests/`, or the tool's own tests in Rust | — |

- **Two criteria proven by one test**: put both tags in one title (`[F02.AC1] [F07.AC1] …`), never two copies of the test.
- **An example is a value computed by hand.** An example that checks the same oracle as a property beside it is merged into the property, whose generator is widened to cover the example's input. The bounds of a range stay as examples.
- **A wider property contains a narrower one** (equal radii inside any radii): keep one property, its generator drawing both cases (`fc.boolean()` choosing equal radii).
- **A Product Owner test card** is walked by the Product Owner on `develop`, not by a test repeating the panel tests one after the other. A walk kept in a test keeps only the steps no other test proves.
- **A removed or merged test** is checked by `npm run mutate` on the code it covered (`-- npx vitest run {test}` for the file's own tests) and by the coverage gates.

## Shared fixtures and helpers

What is slow to prepare is prepared once; what a test proves is never prepared for it.

- Shared test data of `src/` lives in `tests/geometry-fixtures.ts`, imported from `tests`: the rectangle of US-001, free shapes, the shoelace area, `expectPoints`, integer points and triangles for properties. A test never writes its own copy of them. Test data never enters `src/`, whose barrels are the public API.
- A **file** is read once per run: Vitest's global setup (`tests/font-files-setup.ts`) reads it and `provide`s its bytes, and each call (`interRegularFile()`) gets a fresh copy (EN-089). One test proves the reading (`tests/font-files.test.ts`); the others use it.
- A **value slow to build**, such as tables built from a font or the source files a repository test reads, is built at its first use by `builtOnce` (`tests/built-once.ts`) and frozen, so that no test changes it for the next. Never build it when a module is imported: most test files import the barrel `tests` and would all pay for it.
- A fixture holds inputs, never a result of the function under test: a parser's test still calls the parser.
- A **helper** of `tests/` or `playground/user-actions/` may build an input, act on the page as a user, read what the page shows, or compare (`expectPoints`, `partsDigest`). It never computes an expected value with the code under test, never holds a branch of that code's logic, and never asserts a claim the test does not name.

## DOM and playground tests

- A DOM test (`src/render/`, `src/interaction/`, `playground/`) starts with `// @vitest-environment happy-dom`. The shell stays thin, so its logic is tested in the core.
- A playground test (`playground/<panel>/*.test.ts`) loads the page as served (`index.html?raw`), mounts it, acts on it and reads what it shows, through the actions of `playground/user-actions/` only. An element the test names and the page lacks fails it at once.
- The actions find elements on their own (`selectPageElement`), never through the playground's `selectElement`, so a test does not lean on the code it checks.
- A playground test sets state through the page as a user does, such as a radius typed per point, never through the page's attributes.
- The criteria that only the playground can show are proven there (`[F01.AC1]`, the display part of `[F01.AC2]`).

## Isolation and time

- Two Vitest projects (`vite.config.ts`, ADR-0070): `shared`, where the files of a worker share the modules they load (`isolate: false`), holds `src/`, `tests/`, `scripts/`, the hooks and the docs; `isolated` holds the playground, whose page keeps state in its modules and whose tests mock the library. A test of `shared` restores what it changes (`process.env`, a global stub) and declares no module mock; a test that needs either moves to `isolated`.
- Vitest's limit of 5 s per test guards against a hang; it does not measure speed. A test known to be long gets a longer limit, with its reason and its duration alone in a comment (`alone: 753 ms`, then `{ timeout: 20_000 }`), the limit at least four times that duration (`.claude/rules/tooling.md`, EN-129). A test that is merely slow is made faster instead.
- A limit under Vitest's 5 s measures a speed: the test is a timing test, tagged `timing` (`{ tags: ["timing"], timeout: 960 }`), left out of `npm run test` and run alone by `test:timings` once `check:all`'s lanes have ended, its file named by that script; `tests/fitness.test.ts` fails otherwise (REV-008, [commands](../tooling/commands.md#the-lanes-of-the-full-check)).
- A playground file mounts the page again in each test's `beforeEach`, in the same document. A listener the page puts on the document or its window takes the mount's signal (`readListenerSignal`), so that the next mount removes it (`removeListeners`, EN-150). A listener without it is heard once per test before, and the tests at the end of a file get slower.

## Tests that prove a requirement (ADR-0027)

A test that proves a feature's acceptance criterion carries the criterion's id in its title, so that the link from criterion to test can be searched and checked. `tests/backlog.test.ts` fails when a `done` feature has a criterion no test names. The `VAL` story tags or writes these tests.

```ts
it("[F01.AC3] clamps a 100 × 25 rectangle with radius 100 to an effective radius of 12.5", () => {
```

## Coverage

`npm run test` measures the coverage of `src/` and `playground/` (`@vitest/coverage-v8`). It fails below **95 %** of lines, branches, functions and statements in `src/` while the library is developed, raised again at a later refinement (Product Owner, 2026-10-25), and below the playground's floor (lines 98, statements 98, functions 100, branches 89 on 2026-10-13). Barrels, tests, the playground's entry and `playground/user-actions/` are excluded. An uncovered branch is either a missing test or dead code.

## Checking the tests themselves

- A test that reads files (the backlog, the bibliography, the barrels) is made to fail once on purpose: break its input and see it fail. A test that cannot fail checks nothing.
- Feature audits break each function once (flip a sign, swap an operator) and check that a test fails ([audit](./audit.md)). StrykerJS is deferred: with Vitest 5 it runs no test against mutants (ADR-0022).

## Test strategy

| Item | Rule |
| --- | --- |
| Levels | examples computed by hand; properties; criterion tests `[F01.ACn]`; the test cards of the user stories, tried by the Product Owner on `develop` at the review (`REV`, ADR-0100); regression and end-to-end suites once the applications exist (E48, ADR-0020) |
| Environments | the core in Node.js without a DOM; `render` and the playground with happy-dom; browsers for end-to-end, later |
| Independence | the `auditor` and `math-expert` agents re-derive, recompute and try mutations; the Product Owner tries the test cards |
| Success | every test green, 95 % coverage of `src/`, no `.skip` nor `.only`, on every push and every release |
| Regression | every test runs before every push and on the pull request into `main`; a done feature's criterion tests are never deleted, only changed by a story citing the criterion |
| Evidence | each release carries its JUnit report (`test-report.xml`) as a release asset |

Test files have looser lint limits: literal numbers, longer functions, callbacks nested up to 4, development packages, no `@kind` ([lint zones](./lint.md#zones)).

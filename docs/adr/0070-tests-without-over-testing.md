# ADR-0070 — Tests without over-testing

**Status**: Accepted (2026-10-17, decided by the Product Owner during F19's run; EN-090).

## Context

- About 3 500 tests in 490 files on 2026-10-17, `npm run test` taking 104 s on this Mac: 35 % of it the setup of each file, 28 % its imports, 24 % the tests themselves (EN-089, EN-090).
- The Product Owner asked to test, never to over-test: a behaviour proven once in one place is not proven again elsewhere, and what is slow to prepare is prepared once (2026-10-17). ADR-0069 makes speed a requirement of every tool.
- ADR-0022 asks for examples and properties side by side; ADR-0027 ties each criterion to the tests that name it. Neither says where a claim is proven, nor how often.
- EN-090's inventory found few claims proven twice — tests identical but for their criterion tag, an example checked against the same oracle as a property beside it, a property contained in a wider one, a Product Owner test card repeating the tests of its own panel — and much work repeated: the files of the repository read again by each test, a sample of 731 000 colors built in every file that imports the fixtures, each file loading the library and its setup again.

## Decision

1. **One claim, one place.** A claim — what a function gives for a class of inputs, what the page does after an action, a rule of the repository — is proven by one test, at the lowest level that can show it. Another test may run the same code, but asserts only what it adds. Two criteria proven by the same test are two tags in one title (`[F02.AC1] [F07.AC1] …`), never two copies.
2. **The level of each claim** (`docs/conventions/testing.md`): a `math` or `geometry` function in its own test file, examples then properties (ADR-0022); a function that composes others tests what it adds (placement, order, a guard), never the values of the functions it calls; a page's behaviour in the test of its panel; a guided step in its epic's steps test, only the state it opens on and the result it states; a repository rule in `tests/`.
3. **An example is a hand-computed value.** An example compared with the same oracle as a property beside it is merged into the property, whose generator is widened to cover it; the bounds of a range stay as examples.
4. **Slow preparation done once.** A file read once per run (a global setup, EN-089); a value built from it, a sample or a set of files read by a repository test built at its first use and shared, frozen (`tests/built-once.ts`); never at a module's import when only some files need it. A fixture holds inputs, never a result of the function under test, and one test proves its preparation.
5. **Modules shared within a worker** (`isolate: false`) for the tests of `src/`, `tests/`, `scripts/`, `eslint/`, the hooks and the docs, which keep no state between files: no module mock, no module-level variable, the environment restored. The playground's tests stay isolated: the page keeps its state in its modules, and three of them mock the library.
6. **A time limit guards against a hang**, never measures speed: Vitest's 5 s by default, a longer one with its reason in a comment where the work is known to be long.
7. **Each epic's end** holds a pass over the whole repository's tooling and tests — duplicates, slow steps, waits — with the naming pass (`NAM`, ADR-0049, ADR-0069 decision 5).

## Reasons

| Option | For | Against |
| --- | --- | --- |
| One claim, one place, slow work shared (chosen) | a failure points at one test; the suite stays fast as it grows | finding the level of a claim takes a moment at each test |
| Every level re-checks what it uses | more paths through the code | the same failure reported many times; minutes added to every check, for the agent most of all (ADR-0069) |
| Modules shared everywhere, the playground included | 104 s down to 33 s | a mock of one playground file would reach the next: tests no longer independent |

## Consequences

- `docs/conventions/testing.md` holds the rules; the `verifier` agent and the audit (A4) look for a claim proven twice as for a claim not proven.
- A new repository test that reads many files reads them once (`builtOnce`).
- A test of the shared project that needs a module mock or keeps state moves to the isolated one.
- A removed or merged test is checked by `mutate` on the code it covered and by the coverage gates.

## References

ADR-0022, ADR-0027, ADR-0049, ADR-0069, EN-089, EN-090

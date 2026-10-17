import type { ChecklistLine } from "./checklist-line";

/**
 * The lines of E17's checklist for F19, Checks on Oxc, TypeScript 7 and Rust.
 *
 * Prepared by `val:prepare` from its acceptance criteria (EN-056), then written as what to
 * try and what to see, at most 12: commands run in a terminal at the repository's root, each
 * tried by VAL-020 on 2026-10-17. A probe file planted to see a finding is deleted afterwards.
 */
export const EPIC_17_F19_LINES: readonly ChecklistLine[] = [
  {
    feature: "F19",
    todo: "Terminal: npm run lint",
    see: "oxlint prints nothing, then “lint: 0 finding(s) in <n> file(s)” from the project's Rust linter, in about 2 s (ESLint took 45 to 62 s)",
  },
  {
    feature: "F19",
    todo: 'Terminal: a file src/probe.ts holding export const wrongCount: number = "1";, then npm run lint',
    see: "“src/probe.ts:1:14: error typescript(TS2322): Type 'string' is not assignable to type 'number'.”: a type error comes with the lint",
  },
  {
    feature: "F19",
    todo: "Terminal: src/probe.ts holding // oxlint-disable-next-line no-console above export const quietCount = 1;, then npm run lint",
    see: "“error: Unused oxlint-disable directive (no problems were reported).”",
  },
  {
    feature: "F19",
    todo: "Terminal: a page docs/probe.md with a table row of three cells under a header of two, then npm run lint",
    see: "“markdown/table-column-count Table column count mismatch (Expected: 2, Actual: 3)…” at that row",
  },
  {
    feature: "F19",
    todo: "Terminal: npm run format:check; then a padded table in docs/probe.md and npm run format",
    see: "“Markdown: 0 of <n> files not formatted.” in under 1 s; the table rewritten “| A | B |”, its separator “| --- | --- |”",
  },
  {
    feature: "F19",
    todo: "Terminal: npx tsc --version, then grep -c typescript6 package-lock.json",
    see: "“Version 7.0.2”, then “0”: TypeScript 7 is the only TypeScript",
  },
  {
    feature: "F19",
    todo: "Terminal: npm run build, then find dist/types -name '*.d.ts' | wc -l",
    see: "the library built in about 2 s, then 589: the declarations written by Oxc, without tsc",
  },
  {
    feature: "F19",
    todo: "Terminal: npm run docs:api",
    see: "“api-pages: 537 pages written to docs/api” in under 3 s, from the Rust generator instead of TypeDoc",
  },
  {
    feature: "F19",
    todo: "Terminal: npm run check:all on a branch that changes nothing in docs/, src/ nor playground/",
    see: "“docs:build, playground:build skipped: nothing changed in their inputs against …”, each lane's time, “check:all passed in” about 40 s",
  },
  {
    feature: "F19",
    todo: "Terminal: npm run check:all on a branch that changes a page of docs/",
    see: "docs:build and playground:build run too, about 50 to 55 s for the docs build; check:all ends in about 55 to 60 s",
  },
  {
    feature: "F19",
    todo: "Terminal: npm run check:rust",
    see: "“advisories ok”, the Rust tests, a coverage TOTAL above the 93 % floor, then the check for Windows; about 1 min alone",
  },
  {
    feature: "F19",
    todo: "Terminal: npx vitest run tests/eslint-rules-accounted.test.ts, then cargo test --release -q -p lint",
    see: "every test passed: the 854 rules ESLint had, 374 in oxlint, 205 in the project's linter, 181 dropped, 94 already off; 652 Rust tests",
  },
];

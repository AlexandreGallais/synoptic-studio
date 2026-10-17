import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/**
 * The correctness rules of oxlint that no ESLint plugin had, so that no triage decided them,
 * turned on by EN-093: each reports code that is wrong whatever its context, none reported on the
 * tree, and none reported by the compiler or by a rule of the project's linter too. Each comes with a planted case.
 */
const ADDED_RULES = [
  {
    code: "const f = (a: number, b: number, c: boolean) => a === b === c;",
    rule: "oxc/bad-comparison-sequence",
  },
  { code: "const f = (s: string) => [...s.matchAll(/a/u)];", rule: "oxc/bad-match-all-arg" },
  { code: "const f = (n: number) => Math.min(Math.max(n, 10), 0);", rule: "oxc/bad-min-max-func" },
  { code: 'const f = (s: string) => s.replaceAll(/a/u, "b");', rule: "oxc/bad-replace-all-arg" },

  { code: "const f = (n: number) => n > 5 && n < 3;", rule: "oxc/const-comparisons" },
  { code: 'function f() { new Error("x"); }', rule: "oxc/missing-throw" },
  { code: "const f = (n: number) => n.toString(40);", rule: "oxc/number-arg-out-of-range" },
  {
    code: "function f(n: number, m: number): number { return n === 0 ? 0 : f(n - 1, m); }",
    rule: "oxc/only-used-in-recursion",
  },
  {
    code: "const f = (xs: number[]) => xs.slice(0, xs.length);",
    rule: "unicorn/no-length-as-slice-end",
  },
] as const;

/** The repository's settings of the rules above, read from `.oxlintrc.json`'s first override. */
const SETTINGS: unknown = JSON.parse(readFileSync(join(ROOT, ".oxlintrc.json"), "utf8"));

/**
 * A property of a value when it is an object, `undefined` otherwise.
 *
 * @param value - any value
 * @param key - the property's name
 * @returns the property's value, untyped
 */
function propertyOf(value: unknown, key: string): unknown {
  return typeof value === "object" && value !== null ? Reflect.get(value, key) : undefined;
}

/**
 * The setting `.oxlintrc.json` gives a rule on every TypeScript file.
 *
 * @param rule - the rule's name in oxlint
 * @returns its setting, `undefined` when the configuration does not name it
 */
function settingOf(rule: string): unknown {
  const overrides = propertyOf(SETTINGS, "overrides");
  const first: unknown = Array.isArray(overrides) ? overrides[0] : undefined;
  const rules = propertyOf(first, "rules");

  return propertyOf(rules, rule);
}

/** Each added rule with its setting in the repository. */
const SETTINGS_OF_ADDED: readonly (readonly [string, unknown])[] = ADDED_RULES.map(({ rule }) => [
  rule,
  settingOf(rule),
]);

/** A folder holding a configuration of the added rules alone, set as the repository sets them. */
const FOLDER = mkdtempSync(join(tmpdir(), "oxlint-correctness-"));

writeFileSync(
  join(FOLDER, "config.json"),
  JSON.stringify({
    categories: { correctness: "off" },
    plugins: ["oxc", "unicorn"],
    rules: Object.fromEntries(SETTINGS_OF_ADDED),
  }),
);

/**
 * The rules oxlint reports on a piece of code, with the added rules alone.
 *
 * @param code - TypeScript source
 * @returns the name of each rule reported
 */
function reportedRules(code: string): readonly string[] {
  const file = join(FOLDER, "planted.ts");

  writeFileSync(file, code);

  // oxlint exits 1 on a finding: its report is read either way.
  const { stdout: output } = spawnSync(
    join(ROOT, "node_modules", ".bin", "oxlint"),
    ["-c", join(FOLDER, "config.json"), "--format", "unix", file],
    { encoding: "utf8" },
  );

  return output
    .matchAll(/\[Error\/(?<plugin>[a-z]+)\((?<rule>[a-z-]+)\)\]$/gmu)
    .map((match) => `${match.groups?.["plugin"] ?? ""}/${match.groups?.["rule"] ?? ""}`)
    .toArray();
}

describe("oxlint's correctness rules added by EN-093", () => {
  it.each(ADDED_RULES)(
    "runs $rule on every TypeScript file, and it reports its planted case",
    ({ code, rule }) => {
      expect(settingOf(rule)).toBe("error");
      expect(reportedRules(code)).toEqual([rule]);
    },
  );
});

/** A folder holding the repository's whole configuration, so that its overrides match by name. */
const WHOLE_FOLDER = mkdtempSync(join(tmpdir(), "oxlint-whole-"));

writeFileSync(
  join(WHOLE_FOLDER, ".oxlintrc.json"),
  readFileSync(join(ROOT, ".oxlintrc.json"), "utf8"),
);

/** A helper with its description and no tag: what a test file may hold since EN-093. */
const UNTAGGED_HELPER = [
  "/**",
  " * Twice a number.",
  " */",
  "export function twice(value: number): number {",
  "  return value * 2;",
  "}",
  "",
].join("\n");

/**
 * The JSDoc rules the repository's configuration reports on a file.
 *
 * @param name - the file's name, which picks its overrides
 * @returns the name of each JSDoc rule reported
 */
function jsdocRulesOn(name: string): readonly string[] {
  const file = join(WHOLE_FOLDER, name);

  writeFileSync(file, UNTAGGED_HELPER);

  // oxlint exits 1 on a finding: its report is read either way.
  const { stdout: output } = spawnSync(
    join(ROOT, "node_modules", ".bin", "oxlint"),
    ["-c", join(WHOLE_FOLDER, ".oxlintrc.json"), "--format", "unix", file],
    { encoding: "utf8" },
  );

  return output
    .matchAll(/\[Error\/jsdoc\((?<rule>[a-z-]+)\)\]$/gmu)
    .map((match) => match.groups?.["rule"] ?? "")
    .toArray()
    .toSorted((left, right) => left.localeCompare(right));
}

describe("a test file's helpers described without their tags (EN-093)", () => {
  it("asks no `@param` nor `@returns` in a test file", () => {
    expect(jsdocRulesOn("helper.test.ts")).toEqual([]);
  });

  it("asks both everywhere else", () => {
    expect(jsdocRulesOn("helper.ts")).toEqual(["require-param", "require-returns"]);
  });
});

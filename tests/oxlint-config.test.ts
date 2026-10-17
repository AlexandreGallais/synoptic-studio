import { execFileSync, spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/** A rule's setting, as both linters write it: a severity, alone or followed by its options. */
type RuleSetting = unknown;

/** An entry of `overrides` in `.oxlintrc.json`: its files and its rules. */
type Override = {
  readonly files: readonly unknown[];
  readonly rules: Readonly<Record<string, RuleSetting>>;
};

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
 * A value when it is an array, an empty array otherwise.
 *
 * @param value - any value
 * @returns its items, untyped
 */
function itemsOf(value: unknown): readonly unknown[] {
  return Array.isArray(value) ? value : [];
}

/**
 * A value's own properties when it is an object, none otherwise.
 *
 * @param value - any value
 * @returns its properties, untyped
 */
function entriesOf(value: unknown): Readonly<Record<string, unknown>> {
  return typeof value === "object" && value !== null
    ? Object.fromEntries(Object.entries(value))
    : {};
}

/**
 * A JSON file of the repository, parsed.
 *
 * @param text - the file's content
 * @returns its value, untyped
 */
function parsed(text: string): unknown {
  return JSON.parse(text);
}

/** The configuration oxlint reads. */
const CONFIG = parsed(readFileSync(join(ROOT, ".oxlintrc.json"), "utf8"));

/** Its overrides: the first holds every rule, the others the scopes. */
const OVERRIDES: readonly Override[] = itemsOf(propertyOf(CONFIG, "overrides")).map((override) => ({
  files: itemsOf(propertyOf(override, "files")),
  rules: entriesOf(propertyOf(override, "rules")),
}));

/** The rules oxlint lists (`oxlint --rules`), as JSON. */
const LISTED_TEXT = execFileSync(
  join(ROOT, "node_modules", ".bin", "oxlint"),
  ["--rules", "--format", "json"],
  { encoding: "utf8" },
);

/** The rules oxlint lists. */
const LISTED_RULES = itemsOf(parsed(LISTED_TEXT));

/** Every rule oxlint has, by the name `.oxlintrc.json` gives it (`eslint` rules bare). */
const OXLINT_RULES: ReadonlySet<string> = new Set(
  LISTED_RULES.map((rule) => {
    const scope = String(propertyOf(rule, "scope"));
    const name = String(propertyOf(rule, "value"));

    return scope === "eslint" ? name : `${scope}/${name}`;
  }),
);

/** Every rule `.oxlintrc.json` names, in any override, with its settings there. */
const CONFIGURED = OVERRIDES.flatMap((override) => Object.entries(override.rules));

/** The rules `.oxlintrc.json` names. */
const CONFIGURED_NAMES: ReadonlySet<string> = new Set(CONFIGURED.map(([name]) => name));

describe(".oxlintrc.json", () => {
  it("[F19.AC1] names only rules oxlint has", () => {
    expect([...CONFIGURED_NAMES].filter((name) => !OXLINT_RULES.has(name))).toEqual([]);
  });

  it("[F19.AC2] runs `prefer-destructuring` natively with typescript-eslint's options", () => {
    // The options `@typescript-eslint/prefer-destructuring` had in `eslint/rules/` (EN-074).
    expect(OVERRIDES[0]?.rules["prefer-destructuring"]).toEqual([
      "error",
      { array: false, object: true },
      { enforceForRenamedProperties: false },
    ]);
  });
});

describe("the other plugins' rules (EN-084)", () => {
  it("[F19.AC11] runs `no-unused-vars` in oxlint with the options `unused-imports` had", () => {
    expect(OVERRIDES[0]?.rules["no-unused-vars"]).toEqual([
      "error",
      {
        args: "after-used",
        caughtErrors: "all",
        fix: { imports: "safe-fix" },
        ignoreRestSiblings: false,
        reportUsedIgnorePattern: true,
        vars: "all",
      },
    ]);
  });
});

describe("the last Unicorn rules (EN-095)", () => {
  it("[F19.AC11] runs the three oxlint carries natively, on every TypeScript file", () => {
    const rules = OVERRIDES[0]?.rules ?? {};

    expect([
      rules["oxc/erasing-op"],
      rules["oxc/misrefactored-assign-op"],
      rules["no-useless-concat"],
    ]).toEqual(["error", "error", "error"]);
  });
});

/** The files that ran ESLint or configured it, read as text. */
const LINT_RUNNERS: readonly string[] = ["package.json", ".lintstagedrc.json"].map((path) =>
  readFileSync(join(ROOT, path), "utf8"),
);

describe("ESLint out (EN-085)", () => {
  it("[F19.AC1] leaves no ESLint package, configuration nor command", () => {
    const manifest = parsed(LINT_RUNNERS[0] ?? "");
    const packages = Object.keys(entriesOf(propertyOf(manifest, "devDependencies")));

    expect(packages.filter((name) => name.includes("eslint"))).toEqual([]);
    expect(LINT_RUNNERS.filter((text) => /\beslint\s/u.test(text))).toEqual([]);

    expect(["eslint.config.ts", "eslint"].filter((path) => existsSync(join(ROOT, path)))).toEqual(
      [],
    );
  });
});

/** A folder holding a configuration of `oxc/no-accumulating-spread` alone. */
const FOLD_FOLDER = mkdtempSync(join(tmpdir(), "fold-spread-"));

writeFileSync(
  join(FOLD_FOLDER, "config.json"),
  JSON.stringify({
    categories: { correctness: "off" },
    rules: { "oxc/no-accumulating-spread": "error" },
  }),
);

/**
 * Lines that `oxc/no-accumulating-spread` reports in a piece of code.
 *
 * @param code - TypeScript source
 * @returns the line of each report
 */
function foldSpreadLines(code: string): readonly number[] {
  const file = join(FOLD_FOLDER, "fold.ts");

  writeFileSync(file, code);

  // oxlint exits 1 on a finding: its report is read either way.
  const { stdout: output } = spawnSync(
    join(ROOT, "node_modules", ".bin", "oxlint"),
    ["-c", join(FOLD_FOLDER, "config.json"), "--format", "unix", file],
    { encoding: "utf8" },
  );

  return output
    .matchAll(/^.+?:(?<line>\d+):\d+: /gmu)
    .map((match) => Number(match.groups?.["line"]))
    .toArray();
}

describe("no spread in a fold (EN-061), oxlint's `oxc/no-accumulating-spread` since EN-075", () => {
  it("[F18.AC18] runs on every TypeScript file, its plugin on", () => {
    expect(OVERRIDES[0]?.rules["oxc/no-accumulating-spread"]).toBe("error");
    expect(itemsOf(propertyOf(CONFIG, "plugins"))).toContain("oxc");
  });

  it.each([
    {
      code: "[1].reduce((all, x) => [...all, x], []);",
      title: "an array spread as the arrow's body",
    },
    {
      code: "[1].reduce((all, x) => ({ ...all, [x]: x }), {});",
      title: "an object spread as the arrow's body",
    },
    {
      code: "[1].reduce(function (all, x) { return [...all, x]; }, []);",
      title: "a spread returned from a block",
    },
    { code: "[1].reduceRight((all, x) => [...all, x], []);", title: "a spread in reduceRight" },
    {
      code: "[1].reduce((all, x) => (x > 0 ? [...all, x] : all), []);",
      title: "a spread in a conditional's branch",
    },
    {
      code: "[1].reduce(function (all, x) { return x > 0 ? all : { ...all, x }; }, {});",
      title: "a spread in a conditional returned from a block",
    },
    {
      // ESLint's selector missed it: oxlint follows the accumulator's name.
      code: "[1].reduce((all, x) => { const next = [...all, x]; return next; }, []);",
      title: "a spread held in a variable, then returned",
    },
  ])("[F18.AC18] reports $title", ({ code }) => {
    expect(foldSpreadLines(code)).toEqual([1]);
  });

  it.each([
    { code: "[1].reduce((sum, x) => sum + x, 0);", title: "a sum" },
    { code: "[[1]].map((row) => [...row, 0]);", title: "a spread in a map" },
    {
      code: "[[1]].reduce((top, row) => Math.max(top, ...row), 0);",
      title: "a spread passed to a call",
    },
    {
      code: "[1].reduce((sum, x) => sum + x, [...[0]].length);",
      title: "a spread in the initial value",
    },
  ])("lets $title pass", ({ code }) => {
    expect(foldSpreadLines(code)).toEqual([]);
  });
});

/** The scripts of `package.json`. */
const SCRIPTS = propertyOf(parsed(LINT_RUNNERS[0] ?? ""), "scripts");

/** The commands lint-staged runs on a TypeScript file. */
const STAGED_TYPESCRIPT = itemsOf(propertyOf(parsed(LINT_RUNNERS[1] ?? ""), "*.ts"));

/** Every oxlint command the repository runs: `lint`, `fix` and lint-staged's. */
const OXLINT_COMMANDS: readonly string[] = [
  String(propertyOf(SCRIPTS, "lint")),
  String(propertyOf(SCRIPTS, "fix")),
  ...STAGED_TYPESCRIPT.map(String),
].flatMap((command) => command.split(" && ").filter((part) => part.startsWith("oxlint ")));

/**
 * The places oxlint reports in a piece of code, run with the options of the `lint` script.
 *
 * @param code - TypeScript source
 * @returns `line:column message` for each report
 */
function lintScriptReports(code: string): readonly string[] {
  const file = join(FOLD_FOLDER, "directive.ts");
  const options = (OXLINT_COMMANDS[0] ?? "")
    .split(" ")
    .filter((word) => word.startsWith("--") && word !== "--type-check");

  writeFileSync(file, code);

  // oxlint exits 1 on a finding: its report is read either way.
  const { stdout: output } = spawnSync(
    join(ROOT, "node_modules", ".bin", "oxlint"),
    ["-c", join(FOLD_FOLDER, "config.json"), ...options, "--format", "unix", file],
    { encoding: "utf8" },
  );

  return output
    .matchAll(/^.+?:(?<place>\d+:\d+): (?<message>.*)$/gmu)
    .map((match) => `${match.groups?.["place"] ?? ""} ${match.groups?.["message"] ?? ""}`)
    .toArray();
}

describe("a disable directive that silences nothing (ESLint's `reportUnusedDisableDirectives`)", () => {
  it("[F19.AC1] is an error in every oxlint command", () => {
    expect(OXLINT_COMMANDS).toHaveLength(3);

    expect(
      OXLINT_COMMANDS.filter(
        (command) => !command.includes("--report-unused-disable-directives-severity=error"),
      ),
    ).toEqual([]);
  });

  it("[F19.AC1] is reported, with the `lint` script's options", () => {
    const code = [
      "// oxlint-disable-next-line oxc/no-accumulating-spread -- nothing to silence",
      "export const sum = [1].reduce((total, x) => total + x, 0);",
      "",
    ].join("\n");

    expect(lintScriptReports(code)).toEqual([
      "1:1 Unused oxlint-disable directive (no problems were reported). [Error]",
    ]);
  });
});

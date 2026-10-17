import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { ESLint } from "eslint";
import { describe, expect, it } from "vitest";

import { DROPPED_RULES, RULE_THEMES, SCOPES } from "../eslint";

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

/** Plugins whose name differs between ESLint and oxlint. */
const PLUGIN_NAMES: Readonly<Record<string, string>> = {
  "@typescript-eslint": "typescript",
  "import-x": "import",
};

/**
 * The name oxlint gives an ESLint rule: `import-x` is `import`, and typescript-eslint's
 * extension rules are ESLint's own in oxlint unless oxlint has a `typescript/` one.
 *
 * @param name - the rule's name in ESLint's configuration
 * @returns its name in `.oxlintrc.json`
 */
function oxlintNameOf(name: string): string {
  const slash = name.lastIndexOf("/");
  const plugin = name.slice(0, Math.max(slash, 0));
  const renamed = `${PLUGIN_NAMES[plugin] ?? plugin}/${name.slice(slash + 1)}`;

  return OXLINT_RULES.has(renamed) ? renamed : name.slice(slash + 1);
}

/**
 * A glob of ESLint's flat configuration as oxlint reads it: a pattern without a slash matches the
 * root only in ESLint, at any depth in oxlint unless it starts with `./`.
 *
 * @param pattern - the glob in ESLint's configuration
 * @returns the same glob for `.oxlintrc.json`
 */
function oxlintGlobOf(pattern: string): string {
  return pattern.includes("/") ? pattern : `./${pattern}`;
}

/** The scopes of `eslint/scopes/` as oxlint would read them: their files and native rules. */
const CARRIED_SCOPES = SCOPES.map((scope) => {
  const rules = Object.entries(scope.rules ?? {}).map(
    ([name, setting]): readonly [string, RuleSetting] => [oxlintNameOf(name), setting],
  );

  return {
    files: (scope.files ?? []).flat().map((pattern) => oxlintGlobOf(pattern)),
    rules: Object.fromEntries(rules.filter(([name]) => OXLINT_RULES.has(name))),
  };
});

/** Every rule `.oxlintrc.json` names, in any override, with its settings there. */
const CONFIGURED = OVERRIDES.flatMap((override) => Object.entries(override.rules));

/** The rules `.oxlintrc.json` names. */
const CONFIGURED_NAMES: ReadonlySet<string> = new Set(CONFIGURED.map(([name]) => name));

/** One file of each scope of ESLint's configuration, the library's layers and the tools'. */
const SAMPLE_FILES = [
  "src/math/sequence/scan.ts",
  "src/math/sequence/scan.test.ts",
  "src/index.ts",
  "playground/index.ts",
  "playground/checklist/read-ticked-lines.ts",
  "eslint/config.ts",
  "eslint/rules/core/all.ts",
  "scripts/check-all.ts",
  "tests/package.test.ts",
  "vite.config.ts",
];

/**
 * Whether a rule's setting runs it: a severity other than off, alone or before its options.
 *
 * @param setting - the setting, as either linter writes it
 * @returns `false` for `off` or `0`
 */
function isOn(setting: RuleSetting): boolean {
  const [severity] = [setting].flat();

  return severity !== "off" && severity !== 0;
}

/** ESLint, configured as `npm run lint` runs it. */
const LINT = new ESLint({ cwd: ROOT });

/**
 * The rules ESLint runs on a file: those its configuration sets to a severity other than off.
 *
 * @param path - the file, from the root
 * @returns the rules' names
 */
async function eslintRulesOf(path: string): Promise<readonly string[]> {
  const config: unknown = await LINT.calculateConfigForFile(join(ROOT, path));

  return Object.entries(entriesOf(propertyOf(config, "rules")))
    .filter(([, setting]) => isOn(setting))
    .map(([name]) => name);
}

/** The setting of a rule in the first theme of `eslint/rules/` that decides it. */
const THEME_SETTINGS: Readonly<Record<string, RuleSetting>> = Object.fromEntries(
  RULE_THEMES.flatMap((theme) => Object.entries(theme.rules ?? {})),
);

describe(".oxlintrc.json", () => {
  it("[F19.AC1] carries each scope of `eslint/scopes/` with native rules as an override of the same files and settings", () => {
    const carried = CARRIED_SCOPES.filter((scope) => Object.keys(scope.rules).length > 0);

    expect(OVERRIDES.slice(1)).toEqual(carried);
  });

  it("[F19.AC1] names only rules oxlint has", () => {
    expect([...CONFIGURED_NAMES].filter((name) => !OXLINT_RULES.has(name))).toEqual([]);
  });

  it(
    "[F19.AC1] runs each rule in one linter: ESLint runs none that oxlint runs or the triage dropped",
    { timeout: 30_000 },
    async () => {
      const dropped = new Set(DROPPED_RULES);
      const doubled = await Promise.all(
        SAMPLE_FILES.map(async (path) => {
          const rules = await eslintRulesOf(path);

          return rules
            .filter((name) => CONFIGURED_NAMES.has(oxlintNameOf(name)) || dropped.has(name))
            .map((name) => `${path}: ${name}`);
        }),
      );

      expect(doubled.flat()).toEqual([]);
    },
  );

  it("[F19.AC11] runs none of the rules the triage dropped", () => {
    const enabled = new Set(
      CONFIGURED.filter(([, setting]) => isOn(setting)).map(([name]) => name),
    );

    expect(
      DROPPED_RULES.map((name) => oxlintNameOf(name)).filter((name) => enabled.has(name)),
    ).toEqual([]);
  });

  it("[F19.AC2] runs `prefer-destructuring` natively with typescript-eslint's options", () => {
    expect(OVERRIDES[0]?.rules["prefer-destructuring"]).toEqual(
      THEME_SETTINGS["@typescript-eslint/prefer-destructuring"],
    );
  });
});

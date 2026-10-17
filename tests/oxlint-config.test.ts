import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

import { SCOPES } from "../eslint";

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

describe(".oxlintrc.json", () => {
  it("[F19.AC1] carries each scope of `eslint/scopes/` with native rules as an override of the same files and settings", () => {
    const carried = CARRIED_SCOPES.filter((scope) => Object.keys(scope.rules).length > 0);

    expect(OVERRIDES.slice(1)).toEqual(carried);
  });
});

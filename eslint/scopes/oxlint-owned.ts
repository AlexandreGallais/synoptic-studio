import { readFileSync } from "node:fs";
import { join } from "node:path";

import js from "@eslint/js";

import { DROPPED_RULES, PROJECT_ROOT, TS_FILES } from "../settings";

import { LANGUAGE } from "./language";

import type { TSESLint } from "@typescript-eslint/utils";

/** ESLint's plugin prefixes for the plugins oxlint names otherwise. */
const ESLINT_PLUGINS: Readonly<Record<string, string>> = {
  import: "import-x",
  typescript: "@typescript-eslint",
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
 * The rules `.oxlintrc.json` names, in any of its overrides, by oxlint's names.
 *
 * @returns each name once
 */
function oxlintRules(): readonly string[] {
  const parsed: unknown = JSON.parse(readFileSync(join(PROJECT_ROOT, ".oxlintrc.json"), "utf8"));
  const overrides = propertyOf(parsed, "overrides");
  const names = (Array.isArray(overrides) ? overrides : []).flatMap((override: unknown) =>
    Object.keys(propertyOf(override, "rules") ?? {}),
  );

  return [...new Set(names)];
}

/**
 * The names ESLint may give a rule oxlint runs: a bare name is ESLint's own rule or
 * typescript-eslint's extension of it; `typescript/` and `import/` are prefixed as ESLint's
 * plugins are.
 *
 * @param name - the rule's name in `.oxlintrc.json`
 * @returns its possible names in ESLint
 */
function eslintNamesOf(name: string): readonly string[] {
  const slash = name.lastIndexOf("/");
  const plugin = name.slice(0, Math.max(slash, 0));
  const rule = name.slice(slash + 1);

  return slash === -1
    ? [rule, `@typescript-eslint/${rule}`]
    : [`${ESLINT_PLUGINS[plugin] ?? plugin}/${rule}`];
}

/**
 * Whether ESLint knows a rule: a live core rule, or a rule of a registered plugin.
 *
 * @param name - `plugin/rule`, or a core rule alone, as ESLint writes it
 * @returns `true` when ESLint can configure it
 */
function isEslintRule(name: string): boolean {
  const slash = name.lastIndexOf("/");

  return slash === -1
    ? Object.hasOwn(js.configs.all.rules, name)
    : propertyOf(
        propertyOf(LANGUAGE.plugins?.[name.slice(0, slash)], "rules"),
        name.slice(slash + 1),
      ) !== undefined;
}

/**
 * The rules ESLint leaves to oxlint and those no linter runs (EN-074): every rule `.oxlintrc.json`
 * names and every dropped rule, off for the TypeScript files, after the scopes, so that each rule
 * runs in one linter only (docs/conventions/lint.md).
 */
export const OXLINT_OWNED: TSESLint.FlatConfig.Config = {
  files: TS_FILES,
  name: "scopes/oxlint-owned",
  rules: Object.fromEntries(
    [...oxlintRules().flatMap((name) => eslintNamesOf(name)), ...DROPPED_RULES]
      .filter((name) => isEslintRule(name))
      .map((name) => [name, "off"]),
  ),
};

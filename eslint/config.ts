import prettierConfig from "eslint-config-prettier";

import {
  CORE_RULES,
  FUNCTIONAL_RULES,
  IMPORTS_RULES,
  JSDOC_RULES,
  LOCAL_RULES,
  MARKDOWN_RULES,
  STYLISTIC_RULES,
  TYPESCRIPT_RULES,
  UNICORN_RULES,
  UNUSED_RULES,
} from "./rules";
import { LANGUAGE, MARKDOWN_LANGUAGE, SCOPES } from "./scopes";
import { IGNORED_FILES, MARKDOWN_FILES, TS_FILES } from "./settings";

import type { TSESLint } from "@typescript-eslint/utils";
import type { Linter } from "eslint";

/** Formatting rules owned by Prettier, switched off (docs/conventions/formatting.md). */
export const PRETTIER_OWNED: TSESLint.FlatConfig.Config = {
  ...prettierConfig,
  files: TS_FILES,
  name: "prettier/owned",
};

/** Markdown rule themes, applied to Markdown pages only. */
export const MARKDOWN_THEMES: readonly TSESLint.FlatConfig.Config[] = MARKDOWN_RULES.map(
  (theme) => ({ ...theme, files: MARKDOWN_FILES }),
);

/** Rule themes, one entry per file of `eslint/rules/`; every rule of every plugin is decided once. */
export const RULE_THEMES: readonly TSESLint.FlatConfig.Config[] = [
  ...CORE_RULES,
  ...TYPESCRIPT_RULES,
  ...JSDOC_RULES,
  ...IMPORTS_RULES,
  ...UNUSED_RULES,
  ...FUNCTIONAL_RULES,
  ...STYLISTIC_RULES,
  ...UNICORN_RULES,
  ...LOCAL_RULES,
].map((theme) => ({ ...theme, files: TS_FILES }));

/**
 * Complete flat configuration: ignores, language, Prettier-owned rules, themes, then scopes.
 *
 * Prettier comes before the themes so that the only formatting rule re-enabled on purpose
 * (`curly: all`, Prettier-compatible) wins. Scopes come last: they relax or tighten by path.
 */
export const CONFIG: readonly (Linter.Config | TSESLint.FlatConfig.Config)[] = [
  { ignores: IGNORED_FILES, name: "ignores" },
  LANGUAGE,
  PRETTIER_OWNED,
  ...RULE_THEMES,
  ...SCOPES,
  MARKDOWN_LANGUAGE,
  ...MARKDOWN_THEMES,
];

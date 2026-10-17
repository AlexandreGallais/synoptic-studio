import type { TSESLint } from "@typescript-eslint/utils";

/** Identifier casing, prefixes and suffixes, by syntactic role. */
export const TYPESCRIPT_NAMING: TSESLint.FlatConfig.Config = {
  name: "typescript/naming",
  rules: {
    // Run by the project's linter, its types read from the written code (tools/lint, EN-076).
    "@typescript-eslint/naming-convention": "off",
  },
};

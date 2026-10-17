import type { TSESLint } from "@typescript-eslint/utils";

/** Vertical rhythm Prettier does not decide: blank lines between statements. */
export const STYLISTIC_SPACING: TSESLint.FlatConfig.Config = {
  name: "stylistic/spacing",
  rules: {
    "@stylistic/lines-between-class-members": ["error", "always", { exceptAfterSingleLine: false }],
    // Run by the project's linter (tools/lint, EN-084).
    "@stylistic/padding-line-between-statements": "off",
  },
};

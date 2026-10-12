import { PADDING_LINES } from "../../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Vertical rhythm Prettier does not decide: blank lines between statements. */
export const STYLISTIC_SPACING: TSESLint.FlatConfig.Config = {
  name: "stylistic/spacing",
  rules: {
    "@stylistic/lines-between-class-members": ["error", "always", { exceptAfterSingleLine: false }],
    // Blank line after declarations, around blocks and before `return`: code reads in paragraphs.
    "@stylistic/padding-line-between-statements": ["error", ...PADDING_LINES],
  },
};

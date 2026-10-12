import { STYLISTIC_COMMENTS } from "./comments";
import { STYLISTIC_PRETTIER_OWNED } from "./prettier-owned";
import { STYLISTIC_SPACING } from "./spacing";

import type { TSESLint } from "@typescript-eslint/utils";

/** Every `stylistic` rule theme, in file order. */
export const STYLISTIC_RULES: readonly TSESLint.FlatConfig.Config[] = [
  STYLISTIC_COMMENTS,
  STYLISTIC_PRETTIER_OWNED,
  STYLISTIC_SPACING,
];

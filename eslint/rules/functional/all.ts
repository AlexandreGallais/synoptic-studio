import { FUNCTIONAL_IMMUTABILITY } from "./immutability";
import { FUNCTIONAL_PARADIGM } from "./paradigm";
import { FUNCTIONAL_PURITY } from "./purity";

import type { TSESLint } from "@typescript-eslint/utils";

/** Every `functional` rule theme, in file order. */
export const FUNCTIONAL_RULES: readonly TSESLint.FlatConfig.Config[] = [
  FUNCTIONAL_IMMUTABILITY,
  FUNCTIONAL_PARADIGM,
  FUNCTIONAL_PURITY,
];

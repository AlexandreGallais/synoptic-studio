import { NAMING_CONVENTION } from "../../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Identifier casing, prefixes and suffixes, by syntactic role. */
export const TYPESCRIPT_NAMING: TSESLint.FlatConfig.Config = {
  name: "typescript/naming",
  rules: {
    "@typescript-eslint/naming-convention": ["error", ...NAMING_CONVENTION],
  },
};

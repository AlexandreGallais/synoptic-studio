import { DENIED_IDENTIFIERS, MAX_IDENTIFIER_LENGTH, SHORT_MATH_IDENTIFIERS } from "../../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Identifiers (the main policy is `@typescript-eslint/naming-convention` and `local/kind-naming`). */
export const CORE_NAMING: TSESLint.FlatConfig.Config = {
  name: "core/naming",
  rules: {
    // Superseded by @typescript-eslint/naming-convention.
    camelcase: "off",
    "id-denylist": ["error", ...DENIED_IDENTIFIERS],
    // Single letters only for standard mathematical variables.
    "id-length": [
      "error",
      {
        exceptions: SHORT_MATH_IDENTIFIERS,
        max: MAX_IDENTIFIER_LENGTH,
        min: 2,
        properties: "never",
      },
    ],
    // Superseded by @typescript-eslint/naming-convention.
    "id-match": "off",
    "no-underscore-dangle": ["error", { allowFunctionParams: false }],
  },
};

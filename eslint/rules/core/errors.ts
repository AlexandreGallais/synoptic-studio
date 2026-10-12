import type { TSESLint } from "@typescript-eslint/utils";

/** Throwing and catching. */
export const CORE_ERRORS: TSESLint.FlatConfig.Config = {
  name: "core/errors",
  rules: {
    "no-ex-assign": "error",
    // Superseded by the type-aware @typescript-eslint/only-throw-error.
    "no-throw-literal": "off",
    "no-useless-catch": "error",
    "preserve-caught-error": ["error", { requireCatchParameter: true }],
  },
};

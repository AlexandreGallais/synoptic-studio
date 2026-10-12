import type { TSESLint } from "@typescript-eslint/utils";

/** Promises. */
export const TYPESCRIPT_PROMISES: TSESLint.FlatConfig.Config = {
  name: "typescript/promises",
  rules: {
    "@typescript-eslint/await-thenable": "error",
    "@typescript-eslint/no-floating-promises": [
      "error",
      { checkThenables: true, ignoreIIFE: false, ignoreVoid: false },
    ],
    "@typescript-eslint/no-misused-promises": "error",
    "@typescript-eslint/only-throw-error": "error",
    "@typescript-eslint/prefer-promise-reject-errors": "error",
    "@typescript-eslint/require-await": "error",
    "@typescript-eslint/return-await": ["error", "always"],
  },
};

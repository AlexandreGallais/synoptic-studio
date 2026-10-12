import type { TSESLint } from "@typescript-eslint/utils";

/** Errors: creation, naming and handling. */
export const UNICORN_ERRORS: TSESLint.FlatConfig.Config = {
  name: "unicorn/errors",
  rules: {
    "unicorn/catch-error-name": "error",
    "unicorn/custom-error-definition": "error",
    "unicorn/error-message": "error",
    "unicorn/no-error-property-assignment": "error",
    "unicorn/no-useless-error-capture-stack-trace": "error",
    "unicorn/prefer-aggregate-error": "error",
    "unicorn/prefer-error-is-error": "error",
    "unicorn/prefer-optional-catch-binding": "error",
    "unicorn/prefer-type-error": "error",
    "unicorn/throw-new-error": "error",
    "unicorn/try-complexity": "error",
  },
};

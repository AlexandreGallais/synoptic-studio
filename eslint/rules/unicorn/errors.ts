import type { TSESLint } from "@typescript-eslint/utils";

/** Errors: creation, naming and handling. */
export const UNICORN_ERRORS: TSESLint.FlatConfig.Config = {
  name: "unicorn/errors",
  rules: {
    "unicorn/catch-error-name": "error",
    "unicorn/custom-error-definition": "error",
    "unicorn/error-message": "error",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/no-error-property-assignment": "off",
    "unicorn/no-useless-error-capture-stack-trace": "error",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/prefer-aggregate-error": "off",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/prefer-error-is-error": "off",
    "unicorn/prefer-optional-catch-binding": "error",
    "unicorn/prefer-type-error": "error",
    "unicorn/throw-new-error": "error",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/try-complexity": "off",
  },
};

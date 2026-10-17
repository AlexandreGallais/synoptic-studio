import type { TSESLint } from "@typescript-eslint/utils";

/** Functions: scope, style, parameters and calls. */
export const UNICORN_FUNCTIONS: TSESLint.FlatConfig.Config = {
  name: "unicorn/functions",
  rules: {
    // Contradicts the core arrow-body-style rule (as-needed), kept as the single source.
    "unicorn/consistent-arrow-return-style": "off",
    "unicorn/consistent-function-scoping": "error",
    "unicorn/consistent-function-style": "error",
    "unicorn/isolated-functions": "error",
    "unicorn/max-nested-calls": "error",
    "unicorn/no-invalid-argument-count": "error",
    // Run by the project's linter, its types read from the written code (tools/lint, EN-076).
    "unicorn/no-non-function-verb-prefix": "off",
    "unicorn/no-object-as-default-parameter": "error",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/no-repeated-call-wrappers": "off",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/no-uncalled-method": "off",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/no-unnecessary-default-arguments": "off",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/no-unnecessary-parameters": "off",
    "unicorn/no-unreadable-iife": "error",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/no-useless-recursion": "off",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/prefer-block-statement-over-iife": "off",
    "unicorn/prefer-default-parameters": "error",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/prefer-short-arrow-method": "off",
  },
};

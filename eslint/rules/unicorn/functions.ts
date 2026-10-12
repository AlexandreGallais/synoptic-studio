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
    "unicorn/no-non-function-verb-prefix": "error",
    "unicorn/no-object-as-default-parameter": "error",
    "unicorn/no-repeated-call-wrappers": "error",
    "unicorn/no-uncalled-method": "error",
    "unicorn/no-unnecessary-default-arguments": "error",
    "unicorn/no-unnecessary-parameters": "error",
    "unicorn/no-unreadable-iife": "error",
    "unicorn/no-useless-recursion": "error",
    "unicorn/prefer-block-statement-over-iife": "error",
    "unicorn/prefer-default-parameters": "error",
    "unicorn/prefer-short-arrow-method": "error",
  },
};

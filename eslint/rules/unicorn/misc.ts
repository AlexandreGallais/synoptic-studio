import type { TSESLint } from "@typescript-eslint/utils";

/** Miscellaneous language features. */
export const UNICORN_MISC: TSESLint.FlatConfig.Config = {
  name: "unicorn/misc",
  rules: {
    // JSON and CSS are formatted by Prettier, not linted.
    "unicorn/comma-spacing": "off",
    "unicorn/consistent-assert": "error",
    "unicorn/consistent-optional-chaining": "error",
    // JSON and CSS are formatted by Prettier, not linted.
    "unicorn/indent": "off",
    // Every folder exposes a barrel by design (ADR-0012).
    "unicorn/no-barrel-files": "off",
    "unicorn/no-console-spaces": "error",
    "unicorn/no-immediate-mutation": "error",
    // Duplicate of the core rule of the same name (rules/core/operators.ts).
    "unicorn/no-loss-of-precision": "off",
    "unicorn/no-misrefactored-assignment": "error",
    // DOM APIs return `null`; comparing with it stays allowed.
    "unicorn/no-null": ["error", { checkStrictEquality: false }],
    "unicorn/no-optional-chaining-on-undeclared-variable": "error",
    "unicorn/no-unnecessary-slice-end": "error",
    "unicorn/no-useless-compound-assignment": "error",
    "unicorn/no-useless-delete-check": "error",
    // Duplicate of the core rule of the same name (rules/core/operators.ts).
    "unicorn/operator-assignment": "off",
    "unicorn/prefer-https": "error",
  },
};

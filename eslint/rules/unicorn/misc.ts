import type { TSESLint } from "@typescript-eslint/utils";

/** Miscellaneous language features. */
export const UNICORN_MISC: TSESLint.FlatConfig.Config = {
  name: "unicorn/misc",
  rules: {
    // JSON and CSS are formatted by Prettier, not linted.
    "unicorn/comma-spacing": "off",
    "unicorn/consistent-assert": "error",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/consistent-optional-chaining": "off",
    // JSON and CSS are formatted by Prettier, not linted.
    "unicorn/indent": "off",
    // Every folder exposes a barrel by design (ADR-0012).
    "unicorn/no-barrel-files": "off",
    "unicorn/no-console-spaces": "error",
    "unicorn/no-immediate-mutation": "error",
    // Duplicate of the core rule of the same name (rules/core/operators.ts).
    "unicorn/no-loss-of-precision": "off",
    // Run by oxlint as oxc/misrefactored-assign-op (EN-095).
    "unicorn/no-misrefactored-assignment": "off",
    // DOM APIs return `null`; comparing with it stays allowed.
    "unicorn/no-null": ["error", { checkStrictEquality: false }],
    "unicorn/no-optional-chaining-on-undeclared-variable": "error",
    "unicorn/no-unnecessary-slice-end": "error",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/no-useless-compound-assignment": "off",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/no-useless-delete-check": "off",
    // Duplicate of the core rule of the same name (rules/core/operators.ts).
    "unicorn/operator-assignment": "off",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/prefer-https": "off",
  },
};

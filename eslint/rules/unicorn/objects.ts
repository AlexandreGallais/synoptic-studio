import type { TSESLint } from "@typescript-eslint/utils";

/** Objects, classes and properties. */
export const UNICORN_OBJECTS: TSESLint.FlatConfig.Config = {
  name: "unicorn/objects",
  rules: {
    "unicorn/class-reference-in-static-methods": "error",
    "unicorn/consistent-class-member-order": "error",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/consistent-destructuring": "off",
    "unicorn/new-for-builtins": "error",
    "unicorn/no-accessor-recursion": "error",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/no-computed-property-existence-check": "off",
    "unicorn/no-incomplete-accessor-override": "error",
    "unicorn/no-instanceof-builtins": "error",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/no-invalid-property-descriptor": "off",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/no-invalid-well-known-symbol-methods": "off",
    "unicorn/no-nonstandard-builtin-properties": "error",
    "unicorn/no-static-only-class": "error",
    "unicorn/no-this-assignment": "error",
    "unicorn/no-this-outside-of-class": "error",
    "unicorn/no-undeclared-class-members": "error",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/no-unreadable-new-expression": "off",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/no-unreadable-object-destructuring": "off",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/no-unsafe-property-key": "off",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/no-unused-builtin-method-return": "off",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/no-unused-properties": "off",
    "unicorn/no-useless-override": "error",
    "unicorn/prefer-class-fields": "error",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/prefer-object-define-properties": "off",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/prefer-object-destructuring-defaults": "off",
    "unicorn/prefer-object-from-entries": "error",
    "unicorn/prefer-private-class-fields": "error",
    "unicorn/prefer-prototype-methods": "error",
    "unicorn/prefer-reflect-apply": "error",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/prefer-single-object-destructuring": "off",
    "unicorn/require-proxy-trap-boolean-return": "error",
  },
};

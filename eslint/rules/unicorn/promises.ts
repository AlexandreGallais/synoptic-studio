import type { TSESLint } from "@typescript-eslint/utils";

/** Promises and asynchronous code. */
export const UNICORN_PROMISES: TSESLint.FlatConfig.Config = {
  name: "unicorn/promises",
  rules: {
    // Run by the project's linter (tools/lint, EN-081).
    "unicorn/no-async-iterator-callback": "off",
    // Run by the project's linter (tools/lint, EN-081).
    "unicorn/no-async-promise-finally": "off",
    "unicorn/no-await-expression-member": "error",
    "unicorn/no-await-in-promise-methods": "error",
    // Run by the project's linter (tools/lint, EN-081).
    "unicorn/no-multiple-promise-resolver-calls": "off",
    "unicorn/no-single-promise-in-promise-methods": "error",
    "unicorn/no-thenable": "error",
    "unicorn/no-unnecessary-await": "error",
    "unicorn/no-unsafe-promise-all-settled-values": "error",
    "unicorn/no-useless-promise-resolve-reject": "error",
    "unicorn/prefer-array-from-async": "error",
    // Run by the project's linter (tools/lint, EN-081).
    "unicorn/prefer-await": "off",
    // Run by the project's linter (tools/lint, EN-081).
    "unicorn/prefer-promise-static-methods": "off",
    // Run by the project's linter (tools/lint, EN-081).
    "unicorn/prefer-promise-try": "off",
    "unicorn/prefer-promise-with-resolvers": "error",
    "unicorn/prefer-queue-microtask": "error",
    // Run by the project's linter (tools/lint, EN-081).
    "unicorn/prefer-then-catch": "off",
  },
};

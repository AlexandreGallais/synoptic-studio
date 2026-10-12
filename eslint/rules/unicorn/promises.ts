import type { TSESLint } from "@typescript-eslint/utils";

/** Promises and asynchronous code. */
export const UNICORN_PROMISES: TSESLint.FlatConfig.Config = {
  name: "unicorn/promises",
  rules: {
    "unicorn/no-async-iterator-callback": "error",
    "unicorn/no-async-promise-finally": "error",
    "unicorn/no-await-expression-member": "error",
    "unicorn/no-await-in-promise-methods": "error",
    "unicorn/no-multiple-promise-resolver-calls": "error",
    "unicorn/no-single-promise-in-promise-methods": "error",
    "unicorn/no-thenable": "error",
    "unicorn/no-unnecessary-await": "error",
    "unicorn/no-unsafe-promise-all-settled-values": "error",
    "unicorn/no-useless-promise-resolve-reject": "error",
    "unicorn/prefer-array-from-async": "error",
    "unicorn/prefer-await": "error",
    "unicorn/prefer-promise-static-methods": "error",
    "unicorn/prefer-promise-try": "error",
    "unicorn/prefer-promise-with-resolvers": "error",
    "unicorn/prefer-queue-microtask": "error",
    "unicorn/prefer-then-catch": "error",
  },
};

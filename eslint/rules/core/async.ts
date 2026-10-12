import type { TSESLint } from "@typescript-eslint/utils";

/** Promises and asynchronous code. */
export const CORE_ASYNC: TSESLint.FlatConfig.Config = {
  name: "core/async",
  rules: {
    "no-async-promise-executor": "error",
    "no-await-in-loop": "error",
    "no-promise-executor-return": ["error", { allowVoid: false }],
    // Superseded by the type-aware @typescript-eslint/prefer-promise-reject-errors.
    "prefer-promise-reject-errors": "off",
    "require-atomic-updates": ["error", { allowProperties: false }],
    // Superseded by the type-aware @typescript-eslint/require-await.
    "require-await": "off",
  },
};

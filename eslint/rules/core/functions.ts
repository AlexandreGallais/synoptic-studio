import type { TSESLint } from "@typescript-eslint/utils";

/** Function declarations, parameters and calls. */
export const CORE_FUNCTIONS: TSESLint.FlatConfig.Config = {
  name: "core/functions",
  rules: {
    "arrow-body-style": ["error", "as-needed"],
    // Superseded by TypeScript's noImplicitReturns (see @typescript-eslint/consistent-return).
    "consistent-return": "off",
    // Superseded by the type-aware @typescript-eslint/default-param-last.
    "default-param-last": "off",
    "func-name-matching": ["error", "always"],
    "func-names": ["error", "always"],
    // Named concepts are function declarations (hoisted, readable top-down); arrows stay inline callbacks.
    "func-style": ["error", "declaration"],
    "getter-return": "error",
    "no-dupe-args": "error",
    // Superseded by the type-aware @typescript-eslint/no-empty-function.
    "no-empty-function": "off",
    "no-extra-bind": "error",
    "no-func-assign": "error",
    "no-inner-declarations": ["error", "both"],
    "no-loop-func": "error",
    // A function never modifies its arguments (docs/conventions/functions.md).
    "no-param-reassign": ["error", { props: true }],
    "no-setter-return": "error",
    "no-useless-call": "error",
    "no-useless-return": "error",
    "prefer-arrow-callback": ["error", { allowNamedFunctions: false }],
    "prefer-rest-params": "error",
    "prefer-spread": "error",
    "require-yield": "error",
  },
};

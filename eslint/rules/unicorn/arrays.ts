import type { TSESLint } from "@typescript-eslint/utils";

/** Arrays and collections. */
export const UNICORN_ARRAYS: TSESLint.FlatConfig.Config = {
  name: "unicorn/arrays",
  rules: {
    "unicorn/consistent-existence-index-check": "error",
    "unicorn/explicit-length-check": "error",
    "unicorn/no-array-callback-reference": "error",
    "unicorn/no-array-fill-with-reference-type": "error",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/no-array-from-fill": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/no-array-front-mutation": "off",
    "unicorn/no-array-method-this-argument": "error",
    // `reduce` is the functional fold, required by functional/no-loop-statements.
    "unicorn/no-array-reduce": "off",
    "unicorn/no-array-reverse": "error",
    "unicorn/no-array-sort": "error",
    "unicorn/no-array-splice": "error",
    "unicorn/no-boolean-sort-comparator": "error",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/no-confusing-array-splice": "off",
    "unicorn/no-confusing-array-with": "error",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/no-impossible-length-comparison": "off",
    "unicorn/no-magic-array-flat-depth": "error",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/no-negated-array-predicate": "off",
    "unicorn/no-new-array": "error",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/no-return-array-push": "off",
    "unicorn/no-unnecessary-array-flat-depth": "error",
    "unicorn/no-unnecessary-array-splice-count": "error",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/no-unnecessary-splice": "off",
    "unicorn/no-unreadable-array-destructuring": "error",
    "unicorn/no-useless-length-check": "error",
    "unicorn/prefer-array-find": "error",
    "unicorn/prefer-array-flat": "error",
    "unicorn/prefer-array-from-range": "error",
    "unicorn/prefer-array-index-of": "error",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-array-last-methods": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-array-slice": "off",
    "unicorn/prefer-array-some": "error",
    "unicorn/prefer-at": "error",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-flat-math-min-max": "off",
    "unicorn/prefer-negative-index": "error",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-simple-sort-comparator": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-single-array-predicate": "off",
    "unicorn/prefer-single-call": "error",
    "unicorn/require-array-join-separator": "error",
    // Duplicate of the type-aware @typescript-eslint/require-array-sort-compare.
    "unicorn/require-array-sort-compare": "off",
  },
};

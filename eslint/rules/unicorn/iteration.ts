import type { TSESLint } from "@typescript-eslint/utils";

/** Loops and iterators. */
export const UNICORN_ITERATION: TSESLint.FlatConfig.Config = {
  name: "unicorn/iteration",
  rules: {
    "unicorn/iteration-fallback-style": "error",
    "unicorn/no-array-concat-in-loop": "error",
    "unicorn/no-array-sort-for-min-max": "error",
    "unicorn/no-break-in-nested-loop": "error",
    "unicorn/no-duplicate-loops": "error",
    "unicorn/no-for-each": "error",
    "unicorn/no-for-loop": "error",
    "unicorn/no-loop-iterable-mutation": "error",
    "unicorn/no-unreadable-for-of-expression": "error",
    "unicorn/no-unused-iterator-helper": "error",
    "unicorn/no-useless-continue": "error",
    "unicorn/no-useless-iterator-to-array": "error",
    "unicorn/prefer-array-iterable-methods": "error",
    // Contradicts the core no-continue rule: loops filter first, then act.
    "unicorn/prefer-continue": "off",
    "unicorn/prefer-direct-iteration": "error",
    "unicorn/prefer-iterable-in-constructor": "error",
    // Its autofix writes Iterator.concat, which Node.js 24 lacks: code passing tsc crashes at run
    // time (VAL-010, RET-002). Back on once the Node.js LTS ships it.
    "unicorn/prefer-iterator-concat": "off",
    "unicorn/prefer-iterator-helpers": "error",
    "unicorn/prefer-iterator-to-array": "error",
    "unicorn/prefer-iterator-to-array-at-end": "error",
    // Same reason: Iterator.zip is missing from Node.js 24.
    "unicorn/prefer-iterator-zip": "off",
    "unicorn/prefer-object-iterable-methods": "error",
    "unicorn/prefer-while-loop-condition": "error",
  },
};

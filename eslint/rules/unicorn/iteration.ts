import type { TSESLint } from "@typescript-eslint/utils";

/** Loops and iterators. */
export const UNICORN_ITERATION: TSESLint.FlatConfig.Config = {
  name: "unicorn/iteration",
  rules: {
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/iteration-fallback-style": "off",
    "unicorn/no-array-concat-in-loop": "error",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/no-array-sort-for-min-max": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/no-break-in-nested-loop": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/no-duplicate-loops": "off",
    // Run by oxlint as unicorn/no-array-for-each, its former name (EN-078).
    "unicorn/no-for-each": "off",
    "unicorn/no-for-loop": "error",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/no-loop-iterable-mutation": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/no-unreadable-for-of-expression": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/no-unused-iterator-helper": "off",
    "unicorn/no-useless-continue": "error",
    "unicorn/no-useless-iterator-to-array": "error",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-array-iterable-methods": "off",
    // Contradicts the core no-continue rule: loops filter first, then act.
    "unicorn/prefer-continue": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-direct-iteration": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-iterable-in-constructor": "off",
    // Its autofix writes Iterator.concat, which Node.js 24 lacks: code passing tsc crashes at run
    // time (VAL-010, RET-002). Back on once the Node.js LTS ships it.
    "unicorn/prefer-iterator-concat": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-iterator-helpers": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-iterator-to-array": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-iterator-to-array-at-end": "off",
    // Same reason: Iterator.zip is missing from Node.js 24.
    "unicorn/prefer-iterator-zip": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-object-iterable-methods": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-while-loop-condition": "off",
  },
};

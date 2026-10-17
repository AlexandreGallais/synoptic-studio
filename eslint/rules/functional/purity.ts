import type { TSESLint } from "@typescript-eslint/utils";

/** Functional core: expressions, no statements with side effects (relaxed by path in tools/lint/src/scopes.rs). */
export const FUNCTIONAL_PURITY: TSESLint.FlatConfig.Config = {
  name: "functional/purity",
  rules: {
    // Run by the project's linter, its `arguments` part by oxlint's `prefer-rest-params` (tools/lint, EN-084).
    "functional/functional-parameters": "off",
    // Run by the project's linter, its branches read from the tree (tools/lint, EN-076).
    "functional/no-conditional-statements": "off",
    // Run by the project's linter (tools/lint, EN-084).
    "functional/no-expression-statements": "off",
    // Run by the project's linter (tools/lint, EN-084).
    "functional/no-loop-statements": "off",
    // Run by the project's linter (tools/lint, EN-084).
    "functional/no-promise-reject": "off",
    // Run by the project's linter (tools/lint, EN-075).
    "functional/no-return-void": "off",
    // Run by the project's linter (tools/lint, EN-084).
    "functional/no-throw-statements": "off",
    // Run by the project's linter (tools/lint, EN-084).
    "functional/no-try-statements": "off",
    // Point-free callbacks pass extra arguments (index, array): unicorn/no-array-callback-reference forbids them.
    "functional/prefer-tacit": "off",
  },
};

import type { TSESLint } from "@typescript-eslint/utils";

/** DOM nodes, selectors and events. */
export const UNICORN_DOM: TSESLint.FlatConfig.Config = {
  name: "unicorn/dom",
  rules: {
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/better-dom-traversing": "off",
    // Run by oxlint as unicorn/prefer-dom-node-dataset, its former name (EN-082).
    "unicorn/dom-node-dataset": "off",
    "unicorn/no-canvas-to-image": "error",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/no-incorrect-query-selector": "off",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/no-invalid-dom-token": "off",
    "unicorn/no-invalid-remove-event-listener": "error",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/no-invalid-style-set-property": "off",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/no-late-current-target-access": "off",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/no-late-event-control": "off",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/no-prevent-default-in-passive-listener": "off",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/no-selector-as-dom-name": "off",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/no-unsafe-dom-html": "off",
    "unicorn/prefer-add-event-listener": "error",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/prefer-add-event-listener-options": "off",
    "unicorn/prefer-classlist-toggle": "error",
    "unicorn/prefer-dom-node-append": "error",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/prefer-dom-node-html-methods": "off",
    "unicorn/prefer-dom-node-remove": "error",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/prefer-dom-node-replace-children": "off",
    "unicorn/prefer-dom-node-text-content": "error",
    "unicorn/prefer-event-target": "error",
    "unicorn/prefer-keyboard-event-key": "error",
    "unicorn/prefer-modern-dom-apis": "error",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/prefer-observer-apis": "off",
    "unicorn/prefer-path2d": "error",
    "unicorn/prefer-query-selector": "error",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/prefer-scoped-selector": "off",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/prefer-toggle-attribute": "off",
    // Run by the project's linter (tools/lint, EN-082).
    "unicorn/require-passive-events": "off",
  },
};

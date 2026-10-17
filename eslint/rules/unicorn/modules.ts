import { NODE_IMPORT_STYLES } from "../../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Modules, imports and the global scope. */
export const UNICORN_MODULES: TSESLint.FlatConfig.Config = {
  name: "unicorn/modules",
  rules: {
    "unicorn/consistent-export-decorator-position": "error",
    "unicorn/default-export-style": "error",
    // Node.js built-ins are imported by name.
    "unicorn/import-style": ["error", { extendDefaultStyles: false, styles: NODE_IMPORT_STYLES }],
    // Duplicate of import-x/no-anonymous-default-export.
    "unicorn/no-anonymous-default-export": "off",
    "unicorn/no-exports-in-scripts": "error",
    // Run by the project's linter (tools/lint, EN-081).
    "unicorn/no-global-object-property-assignment": "off",
    "unicorn/no-top-level-assignment-in-function": "error",
    // Run by the project's linter (tools/lint, EN-081).
    "unicorn/no-top-level-side-effects": "off",
    // Run by the project's linter (tools/lint, EN-081).
    "unicorn/no-unnecessary-global-this": "off",
    "unicorn/no-unnecessary-polyfills": "error",
    // Run by the project's linter (tools/lint, EN-081).
    "unicorn/no-useless-re-export": "off",
    "unicorn/prefer-export-from": "error",
    // Run by the project's linter (tools/lint, EN-081).
    "unicorn/prefer-global-number-constants": "off",
    "unicorn/prefer-global-this": "error",
    // Run by the project's linter (tools/lint, EN-081).
    "unicorn/prefer-identifier-import-export-specifiers": "off",
    "unicorn/prefer-import-meta-properties": "error",
    // Run by the project's linter (tools/lint, EN-081).
    "unicorn/prefer-json-import": "off",
    "unicorn/prefer-module": "error",
    "unicorn/prefer-top-level-await": "error",
    "unicorn/require-module-attributes": "error",
    "unicorn/require-module-specifiers": "error",
  },
};

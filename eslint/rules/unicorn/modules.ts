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
    "unicorn/no-global-object-property-assignment": "error",
    "unicorn/no-top-level-assignment-in-function": "error",
    "unicorn/no-top-level-side-effects": "error",
    "unicorn/no-unnecessary-global-this": "error",
    "unicorn/no-unnecessary-polyfills": "error",
    "unicorn/no-useless-re-export": "error",
    "unicorn/prefer-export-from": "error",
    "unicorn/prefer-global-number-constants": "error",
    "unicorn/prefer-global-this": "error",
    "unicorn/prefer-identifier-import-export-specifiers": "error",
    "unicorn/prefer-import-meta-properties": "error",
    "unicorn/prefer-json-import": "error",
    "unicorn/prefer-module": "error",
    "unicorn/prefer-top-level-await": "error",
    "unicorn/require-module-attributes": "error",
    "unicorn/require-module-specifiers": "error",
  },
};

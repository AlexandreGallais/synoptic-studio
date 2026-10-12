import { IMPORT_ORDER } from "../../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Shape and order of import and export statements. */
export const IMPORTS_STYLE: TSESLint.FlatConfig.Config = {
  name: "imports/style",
  rules: {
    "import-x/consistent-type-specifier-style": ["error", "prefer-top-level"],
    // Webpack-specific.
    "import-x/dynamic-import-chunkname": "off",
    // A module reads top-down; exports and their helpers interleave.
    "import-x/exports-last": "off",
    "import-x/first": "error",
    // Each export stays next to its declaration and JSDoc.
    "import-x/group-exports": "off",
    // Comments between imports (eslint-disable directives) belong to the next import.
    "import-x/newline-after-import": ["error", { considerComments: false, count: 1 }],
    "import-x/no-duplicates": ["error", { "prefer-inline": false }],
    "import-x/no-empty-named-blocks": "error",
    "import-x/no-mutable-exports": "error",
    "import-x/no-namespace": "error",
    "import-x/no-unassigned-import": ["error", { allow: ["**/*.css"] }],
    "import-x/order": ["error", IMPORT_ORDER],
    // Namespace imports are banned (import-x/no-namespace).
    "import-x/prefer-namespace-import": "off",
    // Every file is a module: "type": "module" and TypeScript's moduleDetection "force".
    "import-x/unambiguous": "off",
  },
};

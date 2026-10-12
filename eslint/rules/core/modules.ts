import type { TSESLint } from "@typescript-eslint/utils";

/** Imports and exports (most module rules come from import-x and the local plugin). */
export const CORE_MODULES: TSESLint.FlatConfig.Config = {
  name: "core/modules",
  rules: {
    // Superseded by import-x/no-duplicates, which understands `import type`.
    "no-duplicate-imports": "off",
    "no-import-assign": "error",
    "no-restricted-exports": [
      "error",
      {
        restrictDefaultExports: {
          direct: true,
          named: true,
          defaultFrom: true,
          namedFrom: true,
          namespaceFrom: true,
        },
      },
    ],
    // Layer boundaries: import-x/no-restricted-paths; path shape: local/canonical-import-path.
    "no-restricted-imports": "off",
    // Members only: declaration order belongs to import-x/order.
    "sort-imports": [
      "error",
      { ignoreCase: false, ignoreDeclarationSort: true, ignoreMemberSort: false },
    ],
    strict: ["error", "safe"],
    "unicode-bom": ["error", "never"],
  },
};

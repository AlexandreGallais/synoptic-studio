import type { TSESLint } from "@typescript-eslint/utils";

/** Named exports only (root tool configurations excepted, scopes/tooling.ts). */
export const IMPORTS_DEFAULT_EXPORTS: TSESLint.FlatConfig.Config = {
  name: "imports/default-exports",
  rules: {
    "import-x/no-anonymous-default-export": "error",
    "import-x/no-default-export": "error",
    "import-x/no-named-as-default": "error",
    "import-x/no-named-as-default-member": "error",
    "import-x/no-named-default": "error",
    // Contradicts the named-exports policy.
    "import-x/no-named-export": "off",
    // Third-party default exports are anonymous (`index`): the importer names them.
    "import-x/no-rename-default": "off",
    // Contradicts no-default-export.
    "import-x/prefer-default-export": "off",
  },
};

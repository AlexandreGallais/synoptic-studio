import type { TSESLint } from "@typescript-eslint/utils";

/** ECMAScript modules only. */
export const IMPORTS_MODULE_SYSTEMS: TSESLint.FlatConfig.Config = {
  name: "imports/module-systems",
  rules: {
    "import-x/no-amd": "error",
    "import-x/no-commonjs": "error",
    "import-x/no-dynamic-require": "error",
    "import-x/no-import-module-exports": "error",
  },
};

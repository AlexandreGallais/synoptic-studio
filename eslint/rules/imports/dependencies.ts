import { MAX_MODULE_DEPENDENCIES } from "../../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Dependency graph: cycles, boundaries and packages. */
export const IMPORTS_DEPENDENCIES: TSESLint.FlatConfig.Config = {
  name: "imports/dependencies",
  rules: {
    "import-x/max-dependencies": [
      "error",
      { ignoreTypeImports: false, max: MAX_MODULE_DEPENDENCIES },
    ],
    "import-x/no-cycle": [
      "error",
      { allowUnsafeDynamicCyclicDependency: false, ignoreExternal: true },
    ],
    // Run by the project's linter, development packages allowed in the tools and the tests (tools/lint, EN-084).
    "import-x/no-extraneous-dependencies": "off",
    // Superseded by local/canonical-import-path, which encodes the barrel policy.
    "import-x/no-internal-modules": "off",
    "import-x/no-nodejs-modules": "error",
    // Cross-layer imports go up (`../geometry`); boundaries are enforced by no-restricted-paths.
    "import-x/no-relative-parent-imports": "off",
    // Run by the project's linter, its zones built from tools/lint/src/settings.rs (EN-084).
    "import-x/no-restricted-paths": "off",
    // Not compatible with ESLint 10 flat config file enumeration; unused exports are reviewed manually.
    "import-x/no-unused-modules": "off",
  },
};

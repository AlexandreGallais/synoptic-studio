import { LAYER_ZONES, MAX_MODULE_DEPENDENCIES } from "../../settings";

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
    // Zero runtime dependency (CLAUDE.md); dev dependencies are allowed in scopes/tooling.ts and scopes/tests.ts.
    "import-x/no-extraneous-dependencies": [
      "error",
      { devDependencies: false, optionalDependencies: false, peerDependencies: false },
    ],
    // Superseded by local/canonical-import-path, which encodes the barrel policy.
    "import-x/no-internal-modules": "off",
    "import-x/no-nodejs-modules": "error",
    // Cross-layer imports go up (`../geometry`); boundaries are enforced by no-restricted-paths.
    "import-x/no-relative-parent-imports": "off",
    // Layer boundaries generated from eslint/settings/layers.ts.
    "import-x/no-restricted-paths": ["error", { zones: LAYER_ZONES }],
    // Not compatible with ESLint 10 flat config file enumeration; unused exports are reviewed manually.
    "import-x/no-unused-modules": "off",
  },
};

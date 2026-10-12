import type { TSESLint } from "@typescript-eslint/utils";

/** Imports resolve to existing modules and existing exports. */
export const IMPORTS_RESOLUTION: TSESLint.FlatConfig.Config = {
  name: "imports/resolution",
  rules: {
    "import-x/default": "error",
    "import-x/export": "error",
    // Superseded by local/canonical-import-path (one autofix source for path shape).
    "import-x/extensions": "off",
    "import-x/named": "error",
    "import-x/namespace": "error",
    "import-x/no-absolute-path": "error",
    // Superseded by the type-aware @typescript-eslint/no-deprecated.
    "import-x/no-deprecated": "off",
    "import-x/no-relative-packages": "error",
    "import-x/no-self-import": "error",
    "import-x/no-unresolved": "error",
    // Superseded by local/canonical-import-path (one autofix source for path shape).
    "import-x/no-useless-path-segments": "off",
    "import-x/no-webpack-loader-syntax": "error",
  },
};

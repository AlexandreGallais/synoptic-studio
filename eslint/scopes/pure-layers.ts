import { DOM_GLOBALS, PURE_LAYER_FILES, RESTRICTED_GLOBALS } from "../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Functional core (math → io): no DOM, no host, no hidden input. */
export const PURE_LAYERS: TSESLint.FlatConfig.Config = {
  files: PURE_LAYER_FILES,
  name: "scopes/pure-layers",
  rules: {
    "no-restricted-globals": ["error", ...RESTRICTED_GLOBALS, ...DOM_GLOBALS],
  },
};

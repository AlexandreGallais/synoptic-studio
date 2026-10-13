import { PROJECT_ROOT } from "../../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Folder barrels and canonical import paths (ADR-0012). */
export const LOCAL_MODULES: TSESLint.FlatConfig.Config = {
  name: "local/modules",
  rules: {
    // Enabled on barrel files only (scopes/barrels.ts).
    "local/barrel-exports": "off",
    "local/canonical-import-path": "error",
    "local/folder-has-index": [
      "error",
      { exemptDirectories: [".", "docs/.vitepress", "tests"], root: PROJECT_ROOT },
    ],
    // Enabled on library and playground files only (scopes/modules.ts).
    "local/folder-layout": "off",
    // Enabled on library and playground modules only (scopes/modules.ts).
    "local/one-export-per-file": "off",
  },
};

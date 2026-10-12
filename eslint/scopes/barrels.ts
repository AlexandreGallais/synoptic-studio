import { AGGREGATOR_FILES, BARREL_FILES } from "../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Folder barrels: generated re-export lists (ADR-0012). */
export const BARRELS: TSESLint.FlatConfig.Config = {
  files: BARREL_FILES,
  name: "scopes/barrels",
  rules: {
    // A barrel's dependencies are its folder's modules, however many.
    "import-x/max-dependencies": "off",
    "local/barrel-exports": "error",
    // A folder without modules yet (bootstrap) has an empty barrel.
    "unicorn/no-empty-file": "off",
  },
};

/** Rule aggregators of `eslint/rules/<plugin>/all.ts`: one import per theme file. */
export const AGGREGATORS: TSESLint.FlatConfig.Config = {
  files: AGGREGATOR_FILES,
  name: "scopes/aggregators",
  rules: {
    "import-x/max-dependencies": "off",
  },
};

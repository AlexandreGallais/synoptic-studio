import { TEST_FILES } from "../settings";

import { KINDLESS, RELAXED_PURITY } from "./relaxed-purity";

import type { TSESLint } from "@typescript-eslint/utils";

/** Vitest files: literal expected values, long `describe` blocks, dev dependencies. */
export const TESTS: TSESLint.FlatConfig.Config = {
  files: TEST_FILES,
  name: "scopes/tests",
  rules: {
    ...RELAXED_PURITY,
    ...KINDLESS,
    "@typescript-eslint/no-magic-numbers": "off",
    "import-x/no-extraneous-dependencies": ["error", { devDependencies: true }],
    "max-lines": "off",
    "max-lines-per-function": "off",
    "max-nested-callbacks": ["error", { max: 4 }],
    "max-statements": "off",
  },
};

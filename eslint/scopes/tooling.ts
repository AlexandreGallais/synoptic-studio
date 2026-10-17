import { ROOT_CONFIG_FILES, TOOLING_FILES } from "../settings";

import { RELAXED_PURITY } from "./relaxed-purity";

import type { TSESLint } from "@typescript-eslint/utils";

/** Lint configuration, custom plugin and tool configurations: Node.js code, dev dependencies. */
export const TOOLING: TSESLint.FlatConfig.Config = {
  files: TOOLING_FILES,
  name: "scopes/tooling",
  rules: {
    ...RELAXED_PURITY,
    "import-x/no-extraneous-dependencies": ["error", { devDependencies: true }],
    "import-x/no-nodejs-modules": "off",
    // typescript-eslint's rule factory is a capitalised function, not a constructor.
    "new-cap": [
      "error",
      {
        capIsNew: true,
        capIsNewExceptions: ["ESLintUtils.RuleCreator"],
        newIsCap: true,
        properties: true,
      },
    ],
  },
};

/** Root configuration files: their tools load the default export. */
export const ROOT_CONFIGS: TSESLint.FlatConfig.Config = {
  files: ROOT_CONFIG_FILES,
  name: "scopes/root-configs",
  rules: {
    "import-x/no-default-export": "off",
    "no-restricted-exports": "off",
  },
};

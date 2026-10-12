import { NODE_RUN_FILES, ROOT_CONFIG_FILES, TOOLING_FILES } from "../settings";

import { KINDLESS, RELAXED_PURITY } from "./relaxed-purity";

import type { TSESLint } from "@typescript-eslint/utils";

/** Lint configuration, custom plugin and tool configurations: Node.js code, dev dependencies. */
export const TOOLING: TSESLint.FlatConfig.Config = {
  files: TOOLING_FILES,
  name: "scopes/tooling",
  rules: {
    ...RELAXED_PURITY,
    ...KINDLESS,
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

/**
 * Files Node.js runs without a build: "file extensions are mandatory" in their imports
 * (`REF-NODE-TS`), so the canonical path without extension (ADR-0012) cannot apply.
 */
export const NODE_RUN: TSESLint.FlatConfig.Config = {
  files: NODE_RUN_FILES,
  name: "scopes/node-run",
  rules: {
    "local/canonical-import-path": "off",
    // Nothing imports these folders as modules: a barrel would only be dead code (knip).
    "local/folder-has-index": "off",
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

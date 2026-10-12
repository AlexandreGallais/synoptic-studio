import stylistic from "@stylistic/eslint-plugin";
import functional from "eslint-plugin-functional";
import { createNodeResolver, importX } from "eslint-plugin-import-x";
// eslint-disable-next-line import-x/no-named-as-default -- the package's default export is the plugin.
import jsdoc from "eslint-plugin-jsdoc";
import unicorn from "eslint-plugin-unicorn";
import unusedImports from "eslint-plugin-unused-imports";
import tseslint from "typescript-eslint";

import { LOCAL_PLUGIN } from "../plugin";
import { PROJECT_ROOT, TS_FILES } from "../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Parser, type information, plugins and resolver shared by every linted file. */
export const LANGUAGE: TSESLint.FlatConfig.Config = {
  files: TS_FILES,
  languageOptions: {
    parser: tseslint.parser,
    parserOptions: { projectService: true, tsconfigRootDir: PROJECT_ROOT },
  },
  linterOptions: { reportUnusedDisableDirectives: "error", reportUnusedInlineConfigs: "error" },
  name: "scopes/language",
  plugins: {
    "@stylistic": stylistic,
    "@typescript-eslint": tseslint.plugin,
    functional,
    "import-x": importX,
    jsdoc,
    local: LOCAL_PLUGIN,
    unicorn,
    "unused-imports": unusedImports,
  },
  settings: {
    "import-x/resolver-next": [createNodeResolver({ extensions: [".ts", ".js", ".json"] })],
    jsdoc: { mode: "typescript" },
  },
};

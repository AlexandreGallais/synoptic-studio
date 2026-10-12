import { PLAYGROUND_ENTRY_FILES } from "../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Playground entry module: it boots the app at load time. */
export const PLAYGROUND_ENTRY: TSESLint.FlatConfig.Config = {
  files: PLAYGROUND_ENTRY_FILES,
  name: "scopes/playground-entry",
  rules: {
    "unicorn/no-top-level-side-effects": "off",
  },
};

import type { TSESLint } from "@typescript-eslint/utils";

/** Regular expressions. */
export const CORE_REGEX: TSESLint.FlatConfig.Config = {
  name: "core/regex",
  rules: {
    "no-control-regex": "error",
    "no-div-regex": "error",
    "no-empty-character-class": "error",
    "no-invalid-regexp": "error",
    "no-misleading-character-class": "error",
    "no-regex-spaces": "error",
    "no-useless-backreference": "error",
    "no-useless-escape": "error",
    "prefer-named-capture-group": "error",
    "prefer-regex-literals": ["error", { disallowRedundantWrapping: true }],
    "require-unicode-regexp": "error",
  },
};

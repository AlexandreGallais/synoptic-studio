import type { TSESLint } from "@typescript-eslint/utils";

/** Unused code, autofixed on save for imports. */
export const UNUSED_UNUSED: TSESLint.FlatConfig.Config = {
  name: "unused/unused",
  rules: {
    "unused-imports/no-unused-imports": "error",
    // Positional callbacks (`(entry, index) => …`) may skip leading parameters.
    "unused-imports/no-unused-vars": [
      "error",
      {
        args: "after-used",
        caughtErrors: "all",
        ignoreRestSiblings: false,
        reportUsedIgnorePattern: true,
        vars: "all",
      },
    ],
  },
};

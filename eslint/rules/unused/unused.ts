import type { TSESLint } from "@typescript-eslint/utils";

/** Unused code, autofixed on save for imports. */
export const UNUSED_UNUSED: TSESLint.FlatConfig.Config = {
  name: "unused/unused",
  rules: {
    // Run by oxlint's `no-unused-vars`, its unused imports autofixed (EN-084).
    "unused-imports/no-unused-imports": "off",
    // Run by oxlint's `no-unused-vars`, with the same options (EN-084).
    "unused-imports/no-unused-vars": "off",
  },
};

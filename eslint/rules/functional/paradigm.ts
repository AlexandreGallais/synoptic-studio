import type { TSESLint } from "@typescript-eslint/utils";

/** No object-oriented constructs. */
export const FUNCTIONAL_PARADIGM: TSESLint.FlatConfig.Config = {
  name: "functional/paradigm",
  rules: {
    "functional/no-class-inheritance": "error",
    "functional/no-classes": "error",
    // Run by the project's linter, its types read from the written code (tools/lint, EN-076).
    "functional/no-mixed-types": "off",
    "functional/no-this-expressions": "error",
  },
};

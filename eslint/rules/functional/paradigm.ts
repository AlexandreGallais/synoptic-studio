import type { TSESLint } from "@typescript-eslint/utils";

/** No object-oriented constructs. */
export const FUNCTIONAL_PARADIGM: TSESLint.FlatConfig.Config = {
  name: "functional/paradigm",
  rules: {
    "functional/no-class-inheritance": "error",
    "functional/no-classes": "error",
    "functional/no-mixed-types": "error",
    "functional/no-this-expressions": "error",
  },
};

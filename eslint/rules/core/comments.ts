import type { TSESLint } from "@typescript-eslint/utils";

/** Comments. */
export const CORE_COMMENTS: TSESLint.FlatConfig.Config = {
  name: "core/comments",
  rules: {
    // Comments often start with a formula or an identifier (`θ = …`); the autofix would corrupt them.
    "capitalized-comments": "off",
    "no-inline-comments": "error",
    // Open work is tracked in docs/ (questions, research requests), not in code.
    "no-warning-comments": [
      "error",
      { location: "anywhere", terms: ["todo", "fixme", "xxx", "hack"] },
    ],
  },
};

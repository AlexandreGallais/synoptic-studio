import type { TSESLint } from "@typescript-eslint/utils";

/** Comment style Prettier preserves as written. */
export const STYLISTIC_COMMENTS: TSESLint.FlatConfig.Config = {
  name: "stylistic/comments",
  rules: {
    "@stylistic/line-comment-position": ["error", { position: "above" }],
    // Run by the project's linter (tools/lint, EN-084).
    "@stylistic/multiline-comment-style": "off",
    // Run by the project's linter (tools/lint, EN-084).
    "@stylistic/spaced-comment": "off",
  },
};

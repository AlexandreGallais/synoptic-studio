import type { TSESLint } from "@typescript-eslint/utils";

/** Comment style Prettier preserves as written. */
export const STYLISTIC_COMMENTS: TSESLint.FlatConfig.Config = {
  name: "stylistic/comments",
  rules: {
    "@stylistic/line-comment-position": ["error", { position: "above" }],
    "@stylistic/multiline-comment-style": ["error", "separate-lines", { checkJSDoc: false }],
    "@stylistic/spaced-comment": [
      "error",
      "always",
      { block: { balanced: true }, line: { markers: ["/"] } },
    ],
  },
};

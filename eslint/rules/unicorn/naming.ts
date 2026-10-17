import type { TSESLint } from "@typescript-eslint/utils";

/** Names of identifiers and files, comments and file shape. */
export const UNICORN_NAMING: TSESLint.FlatConfig.Config = {
  name: "unicorn/naming",
  rules: {
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/comment-content": "off",
    // Boolean prefixes are enforced by @typescript-eslint/naming-convention.
    "unicorn/consistent-boolean-name": "off",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/consistent-compound-words": "off",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/consistent-tuple-labels": "off",
    // Task-marker comments are banned outright (no-warning-comments).
    "unicorn/expiring-todo-comments": "off",
    "unicorn/filename-case": ["error", { case: "kebabCase" }],
    // Superseded by @typescript-eslint/naming-convention.
    "unicorn/id-match": "off",
    // Targets JSON or Markdown, which ESLint does not lint here (Prettier formats them).
    "unicorn/key-name-casing": "off",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/name-replacements": "off",
    "unicorn/no-abusive-eslint-disable": "error",
    // Contradicts jsdoc/require-asterisk-prefix.
    "unicorn/no-asterisk-prefix-in-documentation-comments": "off",
    "unicorn/no-empty-file": "error",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/no-keyword-prefix": "off",
    "unicorn/no-leading-empty-lines": "error",
    // Prettier does not wrap comments: they are wrapped by hand at max_line_length.
    "unicorn/no-manually-wrapped-comments": "off",
    // Duplicate of import-x/no-named-default.
    "unicorn/no-named-default": "off",
    // Run by the project's linter (tools/lint, EN-080).
    "unicorn/prefer-type-literal-last": "off",
    // A one-line JSDoc stays on one line: `/** Doc. */`.
    "unicorn/single-line-block-comment-style": ["error", "single-line"],
  },
};

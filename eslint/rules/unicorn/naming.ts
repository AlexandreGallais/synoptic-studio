import type { TSESLint } from "@typescript-eslint/utils";

/** Names of identifiers and files, comments and file shape. */
export const UNICORN_NAMING: TSESLint.FlatConfig.Config = {
  name: "unicorn/naming",
  rules: {
    "unicorn/comment-content": ["error", { replacements: { application: false } }],
    // Boolean prefixes are enforced by @typescript-eslint/naming-convention.
    "unicorn/consistent-boolean-name": "off",
    "unicorn/consistent-compound-words": "error",
    "unicorn/consistent-tuple-labels": "error",
    // Task-marker comments are banned outright (no-warning-comments).
    "unicorn/expiring-todo-comments": "off",
    "unicorn/filename-case": ["error", { case: "kebabCase" }],
    // Superseded by @typescript-eslint/naming-convention.
    "unicorn/id-match": "off",
    // Targets JSON or Markdown, which ESLint does not lint here (Prettier formats them).
    "unicorn/key-name-casing": "off",
    "unicorn/name-replacements": "error",
    "unicorn/no-abusive-eslint-disable": "error",
    // Contradicts jsdoc/require-asterisk-prefix.
    "unicorn/no-asterisk-prefix-in-documentation-comments": "off",
    "unicorn/no-empty-file": "error",
    // Third-party option names (`newIsCap`) are not ours to rename.
    "unicorn/no-keyword-prefix": ["error", { checkProperties: false }],
    "unicorn/no-leading-empty-lines": "error",
    // Prettier does not wrap comments: they are wrapped by hand at max_line_length.
    "unicorn/no-manually-wrapped-comments": "off",
    // Duplicate of import-x/no-named-default.
    "unicorn/no-named-default": "off",
    "unicorn/prefer-type-literal-last": "error",
    // A one-line JSDoc stays on one line: `/** Doc. */`.
    "unicorn/single-line-block-comment-style": ["error", "single-line"],
  },
};

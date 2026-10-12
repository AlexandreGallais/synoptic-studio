import type { TSESLint } from "@typescript-eslint/utils";

/** Structure of Markdown pages (docs/conventions/writing.md); layout is Prettier's. */
export const MARKDOWN_STRUCTURE: TSESLint.FlatConfig.Config = {
  name: "markdown/structure",
  rules: {
    "markdown/fenced-code-language": "error",
    "markdown/fenced-code-meta": ["error", "never"],
    "markdown/heading-increment": ["error", { frontmatterTitle: "" }],
    "markdown/no-bare-urls": "error",
    "markdown/no-duplicate-definitions": "error",
    "markdown/no-duplicate-headings": ["error", { checkSiblingsOnly: true }],
    "markdown/no-empty-definitions": "error",
    "markdown/no-empty-images": "error",
    "markdown/no-empty-links": "error",
    // `<span v-pre>` is the documented way to show double curly braces on the site.
    "markdown/no-html": ["error", { allowed: ["span"] }],
    "markdown/no-invalid-label-refs": "error",
    "markdown/no-missing-atx-heading-space": "error",
    // `[verified]` / `[unverified]` are the bibliography status markers (docs/references.md).
    "markdown/no-missing-label-refs": [
      "error",
      // GitHub-flavored alerts (docs/conventions/writing.md) and the bibliography's status labels.
      {
        allowLabels: [
          "!CAUTION",
          "!IMPORTANT",
          "!NOTE",
          "!TIP",
          "!WARNING",
          "unverified",
          "verified",
        ],
      },
    ],
    "markdown/no-missing-link-fragments": "error",
    "markdown/no-multiple-h1": ["error", { frontmatterTitle: "" }],
    "markdown/no-reference-like-urls": "error",
    "markdown/no-reversed-media-syntax": "error",
    "markdown/no-space-in-emphasis": "error",
    "markdown/no-unused-definitions": "error",
    "markdown/require-alt-text": "error",
    "markdown/table-column-count": ["error", { checkMissingCells: true }],
  },
};

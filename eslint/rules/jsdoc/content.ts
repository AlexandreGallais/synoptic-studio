import type { TSESLint } from "@typescript-eslint/utils";

/** Wording and validity of descriptions, names and references. */
export const JSDOC_CONTENT: TSESLint.FlatConfig.Config = {
  name: "jsdoc/content",
  rules: {
    "jsdoc/check-access": "error",
    // Deprecated upstream in favour of a processor; no `@example` blocks are written.
    "jsdoc/check-examples": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/check-param-names": "off",
    "jsdoc/check-property-names": "error",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/check-syntax": "off",
    // `@kind` is the project's function classification tag (ADR-0011).
    "jsdoc/check-tag-names": ["error", { definedTags: ["kind"], typed: true }],
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/check-template-names": "off",
    // Validates JSDoc 3 `@kind` values (class, function…), which the project redefines (ADR-0011); its other tags are banned.
    "jsdoc/check-values": "off",
    "jsdoc/empty-tags": "error",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/escape-inline-tags": "off",
    "jsdoc/implements-on-classes": "error",
    "jsdoc/imports-as-dependencies": "error",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/informative-docs": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/match-description": "off",
    // No naming constraint is expressed through JSDoc names.
    "jsdoc/match-name": "off",
    "jsdoc/no-defaults": "error",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/no-restricted-syntax": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/normalize-see-links": "off",
    // Checks tag descriptions as sentences; the project writes them as lower-case fragments (match-description).
    "jsdoc/require-description-complete-sentence": "off",
    // Types live in TypeScript annotations, never in JSDoc (jsdoc/no-types).
    "jsdoc/require-next-type": "off",
    // Types live in TypeScript annotations, never in JSDoc (jsdoc/no-types).
    "jsdoc/require-param-type": "off",
    // Types live in TypeScript annotations, never in JSDoc (jsdoc/no-types).
    "jsdoc/require-property-type": "off",
    // Types live in TypeScript annotations, never in JSDoc (jsdoc/no-types).
    "jsdoc/require-returns-type": "off",
    // Types live in TypeScript annotations, never in JSDoc (jsdoc/no-types).
    "jsdoc/require-throws-type": "off",
    // Types live in TypeScript annotations, never in JSDoc (jsdoc/no-types).
    "jsdoc/require-yields-type": "off",
    // Formulas use `<` and `>` literally; HTML or Markdown escaping would make them unreadable.
    "jsdoc/text-escaping": "off",
  },
};

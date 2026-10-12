import { JSDOC_DESCRIPTION_PATTERNS } from "../../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Wording and validity of descriptions, names and references. */
export const JSDOC_CONTENT: TSESLint.FlatConfig.Config = {
  name: "jsdoc/content",
  rules: {
    "jsdoc/check-access": "error",
    // Deprecated upstream in favour of a processor; no `@example` blocks are written.
    "jsdoc/check-examples": "off",
    "jsdoc/check-param-names": [
      "error",
      { checkDestructured: true, disableExtraPropertyReporting: false, enableFixer: true },
    ],
    "jsdoc/check-property-names": "error",
    "jsdoc/check-syntax": "error",
    // `@kind` is the project's function classification tag (ADR-0011).
    "jsdoc/check-tag-names": ["error", { definedTags: ["kind"], typed: true }],
    "jsdoc/check-template-names": "error",
    // Validates JSDoc 3 `@kind` values (class, function…), which the project redefines (ADR-0011); its other tags are banned.
    "jsdoc/check-values": "off",
    "jsdoc/empty-tags": "error",
    "jsdoc/escape-inline-tags": "error",
    "jsdoc/implements-on-classes": "error",
    "jsdoc/imports-as-dependencies": "error",
    // `@kind` holds a classification (e.g. `format` on `formatSvgNumber`), not a description.
    "jsdoc/informative-docs": ["error", { excludedTags: ["kind"] }],
    // Main description: sentences. `@param` / `@returns`: lower-case fragments without final period.
    "jsdoc/match-description": ["error", JSDOC_DESCRIPTION_PATTERNS],
    // No naming constraint is expressed through JSDoc names.
    "jsdoc/match-name": "off",
    "jsdoc/no-defaults": "error",
    "jsdoc/no-restricted-syntax": [
      "error",
      {
        contexts: [
          {
            comment: "JsdocBlock:has(JsdocTag[tag=/^(todo|author|version|since)$/])",
            context: "any",
            message: "Track work and history in docs/ and git, not in JSDoc tags.",
          },
        ],
      },
    ],
    "jsdoc/normalize-see-links": "error",
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

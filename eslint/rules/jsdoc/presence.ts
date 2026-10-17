import type { TSESLint } from "@typescript-eslint/utils";

/** Which declarations must carry a JSDoc block, and which tags it must hold. */
export const JSDOC_PRESENCE: TSESLint.FlatConfig.Config = {
  name: "jsdoc/presence",
  rules: {
    // Required tags are checked by local/require-kind and local/see-references.
    "jsdoc/no-missing-syntax": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/require-description": "off",
    // Tests are the executable examples; `@example` blocks would duplicate them.
    "jsdoc/require-example": "off",
    // One concept per file, named after it: an overview would restate the file name.
    "jsdoc/require-file-overview": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/require-jsdoc": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/require-next-description": "off",
    "jsdoc/require-param": [
      "error",
      { checkDestructured: true, checkRestProperty: true, enableRestElementFixer: false },
    ],
    "jsdoc/require-param-description": "error",
    "jsdoc/require-param-name": "error",
    "jsdoc/require-property": "error",
    "jsdoc/require-property-description": "error",
    "jsdoc/require-property-name": "error",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/require-rejects": "off",
    "jsdoc/require-returns": ["error", { checkGetters: true, forceReturnsWithAsync: true }],
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/require-returns-check": "off",
    "jsdoc/require-returns-description": "error",
    // `@kind` presence and value are checked by local/require-kind.
    "jsdoc/require-tags": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/require-template": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/require-template-description": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/require-throws": "off",
    "jsdoc/require-throws-description": "error",
    "jsdoc/require-yields": "error",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/require-yields-check": "off",
    "jsdoc/require-yields-description": "error",
  },
};

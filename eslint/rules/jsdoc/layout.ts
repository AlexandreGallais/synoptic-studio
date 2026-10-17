import type { TSESLint } from "@typescript-eslint/utils";

/** Layout of JSDoc blocks (Prettier does not format comments). */
export const JSDOC_LAYOUT: TSESLint.FlatConfig.Config = {
  name: "jsdoc/layout",
  rules: {
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/check-alignment": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/check-indentation": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/check-line-alignment": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/convert-to-jsdoc-comments": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/lines-before-block": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/multiline-blocks": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/no-bad-blocks": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/no-blank-block-descriptions": "off",
    "jsdoc/no-blank-blocks": ["error", { enableFixer: true }],
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/no-multi-asterisks": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/require-asterisk-prefix": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/require-hyphen-before-param-description": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/sort-tags": "off",
    // Run by the project's linter (tools/lint, EN-083).
    "jsdoc/tag-lines": "off",
  },
};

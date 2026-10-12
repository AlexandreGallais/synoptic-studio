import { JSDOC_TAG_SEQUENCE } from "../../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Layout of JSDoc blocks (Prettier does not format comments). */
export const JSDOC_LAYOUT: TSESLint.FlatConfig.Config = {
  name: "jsdoc/layout",
  rules: {
    "jsdoc/check-alignment": "error",
    "jsdoc/check-indentation": "error",
    "jsdoc/check-line-alignment": ["error", "never"],
    "jsdoc/convert-to-jsdoc-comments": [
      "error",
      { enableFixer: true, enforceJsdocLineStyle: "multi", lineOrBlockStyle: "both" },
    ],
    "jsdoc/lines-before-block": "error",
    "jsdoc/multiline-blocks": "error",
    "jsdoc/no-bad-blocks": "error",
    "jsdoc/no-blank-block-descriptions": "error",
    "jsdoc/no-blank-blocks": ["error", { enableFixer: true }],
    "jsdoc/no-multi-asterisks": "error",
    "jsdoc/require-asterisk-prefix": "error",
    "jsdoc/require-hyphen-before-param-description": ["error", "always"],
    "jsdoc/sort-tags": [
      "error",
      {
        alphabetizeExtras: false,
        linesBetween: 0,
        reportTagGroupSpacing: false,
        tagSequence: JSDOC_TAG_SEQUENCE,
      },
    ],
    "jsdoc/tag-lines": ["error", "never", { startLines: 1 }],
  },
};

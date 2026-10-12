import type { TSESLint } from "@typescript-eslint/utils";

/** Type expressions inside JSDoc (forbidden: TypeScript holds the types). */
export const JSDOC_TYPES: TSESLint.FlatConfig.Config = {
  name: "jsdoc/types",
  rules: {
    "jsdoc/check-types": "error",
    "jsdoc/no-types": "error",
    "jsdoc/no-undefined-types": "error",
    "jsdoc/no-unnecessary-type-assertion": "error",
    "jsdoc/prefer-import-tag": "error",
    "jsdoc/reject-any-type": "error",
    "jsdoc/reject-function-type": "error",
    "jsdoc/ts-ban-ts-comment": "error",
    "jsdoc/ts-method-signature-style": "error",
    "jsdoc/ts-no-empty-object-type": "error",
    "jsdoc/ts-no-unnecessary-template-expression": "error",
    "jsdoc/ts-prefer-function-type": "error",
    "jsdoc/type-formatting": "error",
    "jsdoc/valid-types": "error",
  },
};

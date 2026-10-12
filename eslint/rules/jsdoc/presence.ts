import { DOCUMENTED_CONTEXTS } from "../../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Which declarations must carry a JSDoc block, and which tags it must hold. */
export const JSDOC_PRESENCE: TSESLint.FlatConfig.Config = {
  name: "jsdoc/presence",
  rules: {
    // Required tags are checked by local/require-kind and local/see-references.
    "jsdoc/no-missing-syntax": "off",
    "jsdoc/require-description": ["error", { descriptionStyle: "body" }],
    // Tests are the executable examples; `@example` blocks would duplicate them.
    "jsdoc/require-example": "off",
    // One concept per file, named after it: an overview would restate the file name.
    "jsdoc/require-file-overview": "off",
    "jsdoc/require-jsdoc": [
      "error",
      {
        contexts: DOCUMENTED_CONTEXTS,
        enableFixer: false,
        publicOnly: false,
        require: {
          ArrowFunctionExpression: false,
          ClassDeclaration: true,
          ClassExpression: true,
          FunctionDeclaration: true,
          FunctionExpression: false,
          MethodDefinition: true,
        },
      },
    ],
    "jsdoc/require-next-description": "error",
    "jsdoc/require-param": [
      "error",
      { checkDestructured: true, checkRestProperty: true, enableRestElementFixer: false },
    ],
    "jsdoc/require-param-description": "error",
    "jsdoc/require-param-name": "error",
    "jsdoc/require-property": "error",
    "jsdoc/require-property-description": "error",
    "jsdoc/require-property-name": "error",
    "jsdoc/require-rejects": "error",
    "jsdoc/require-returns": ["error", { checkGetters: true, forceReturnsWithAsync: true }],
    "jsdoc/require-returns-check": "error",
    "jsdoc/require-returns-description": "error",
    // `@kind` presence and value are checked by local/require-kind.
    "jsdoc/require-tags": "off",
    "jsdoc/require-template": "error",
    "jsdoc/require-template-description": "error",
    "jsdoc/require-throws": "error",
    "jsdoc/require-throws-description": "error",
    "jsdoc/require-yields": "error",
    "jsdoc/require-yields-check": "error",
    "jsdoc/require-yields-description": "error",
  },
};

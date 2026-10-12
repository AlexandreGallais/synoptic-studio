import { GLOBAL_FUNCTION_CEILING } from "../../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Function signatures: explicit types at every boundary. */
export const TYPESCRIPT_SIGNATURES: TSESLint.FlatConfig.Config = {
  name: "typescript/signatures",
  rules: {
    "@typescript-eslint/default-param-last": "error",
    "@typescript-eslint/explicit-function-return-type": [
      "error",
      {
        allowConciseArrowFunctionExpressionsStartingWithVoid: false,
        allowDirectConstAssertionInArrowFunctions: false,
        allowExpressions: false,
        allowFunctionsWithoutTypeParameters: false,
        allowHigherOrderFunctions: false,
        allowIIFEs: false,
        allowTypedFunctionExpressions: true,
      },
    ],
    "@typescript-eslint/explicit-module-boundary-types": [
      "error",
      {
        allowArgumentsExplicitlyTypedAsAny: false,
        allowDirectConstAssertionInArrowFunctions: false,
        allowHigherOrderFunctions: false,
        allowTypedFunctionExpressions: true,
      },
    ],
    // Beyond three parameters, pass one named object.
    "@typescript-eslint/max-params": [
      "error",
      { countVoidThis: true, max: GLOBAL_FUNCTION_CEILING.maxParams },
    ],
    "@typescript-eslint/no-empty-function": "error",
    // Superseded by functional/prefer-immutable-types, which is configurable per layer.
    "@typescript-eslint/prefer-readonly-parameter-types": "off",
    "@typescript-eslint/prefer-return-this-type": "error",
    "@typescript-eslint/promise-function-async": "error",
  },
};

import type { TSESLint } from "@typescript-eslint/utils";

/** Conditions and values the type system proves useless or ambiguous. */
export const TYPESCRIPT_STRICTNESS: TSESLint.FlatConfig.Config = {
  name: "typescript/strictness",
  rules: {
    "@typescript-eslint/no-array-delete": "error",
    "@typescript-eslint/no-confusing-void-expression": [
      "error",
      {
        ignoreArrowShorthand: false,
        ignoreVoidOperator: false,
        ignoreVoidReturningFunctions: false,
      },
    ],
    "@typescript-eslint/no-deprecated": "error",
    "@typescript-eslint/no-duplicate-type-constituents": "error",
    "@typescript-eslint/no-dynamic-delete": "error",
    "@typescript-eslint/no-for-in-array": "error",
    "@typescript-eslint/no-invalid-void-type": [
      "error",
      { allowAsThisParameter: false, allowInGenericTypeArguments: true },
    ],
    "@typescript-eslint/no-meaningless-void-operator": ["error", { checkNever: true }],
    "@typescript-eslint/no-misused-spread": "error",
    "@typescript-eslint/no-redundant-type-constituents": "error",
    "@typescript-eslint/no-unnecessary-boolean-literal-compare": "error",
    "@typescript-eslint/no-unnecessary-condition": [
      "error",
      { allowConstantLoopConditions: "never", checkTypePredicates: true },
    ],
    "@typescript-eslint/no-unnecessary-parameter-property-assignment": "error",
    "@typescript-eslint/no-unnecessary-qualifier": "error",
    "@typescript-eslint/no-unnecessary-template-expression": "error",
    "@typescript-eslint/no-unnecessary-type-arguments": "error",
    "@typescript-eslint/no-unnecessary-type-constraint": "error",
    "@typescript-eslint/no-unnecessary-type-conversion": "error",
    "@typescript-eslint/no-unnecessary-type-parameters": "error",
    "@typescript-eslint/no-useless-default-assignment": "error",
    "@typescript-eslint/no-useless-empty-export": "error",
    "@typescript-eslint/prefer-nullish-coalescing": [
      "error",
      {
        ignoreConditionalTests: false,
        ignoreMixedLogicalExpressions: false,
        ignoreTernaryTests: false,
      },
    ],
    "@typescript-eslint/prefer-optional-chain": "error",
    "@typescript-eslint/require-array-sort-compare": ["error", { ignoreStringArrays: false }],
    "@typescript-eslint/strict-boolean-expressions": [
      "error",
      {
        allowAny: false,
        allowNullableBoolean: false,
        allowNullableEnum: false,
        allowNullableNumber: false,
        allowNullableObject: false,
        allowNullableString: false,
        allowNumber: false,
        allowString: false,
      },
    ],
    "@typescript-eslint/strict-void-return": "error",
    "@typescript-eslint/switch-exhaustiveness-check": [
      "error",
      {
        allowDefaultCaseForExhaustiveSwitch: false,
        considerDefaultExhaustiveForUnions: false,
        requireDefaultForNonUnion: true,
      },
    ],
  },
};

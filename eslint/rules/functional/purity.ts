import type { TSESLint } from "@typescript-eslint/utils";

/** Functional core: expressions, no statements with side effects (relaxed in scopes/dom-layers.ts). */
export const FUNCTIONAL_PURITY: TSESLint.FlatConfig.Config = {
  name: "functional/purity",
  rules: {
    // A pure function without parameter is a constant.
    "functional/functional-parameters": [
      "error",
      {
        allowArgumentsKeyword: false,
        allowRestParameter: true,
        enforceParameterCount: "atLeastOne",
      },
    ],
    "functional/no-conditional-statements": ["error", { allowReturningBranches: true }],
    "functional/no-expression-statements": ["error", { ignoreVoid: false }],
    "functional/no-loop-statements": "error",
    "functional/no-promise-reject": "error",
    "functional/no-return-void": [
      "error",
      { allowNull: false, allowUndefined: false, ignoreInferredTypes: false },
    ],
    "functional/no-throw-statements": ["error", { allowToRejectPromises: false }],
    "functional/no-try-statements": ["error", { allowCatch: false, allowFinally: false }],
    // Point-free callbacks pass extra arguments (index, array): unicorn/no-array-callback-reference forbids them.
    "functional/prefer-tacit": "off",
  },
};

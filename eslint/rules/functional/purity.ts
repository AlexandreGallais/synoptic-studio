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
    // Run by the project's linter, its branches read from the tree (tools/lint, EN-076).
    "functional/no-conditional-statements": "off",
    "functional/no-expression-statements": ["error", { ignoreVoid: false }],
    "functional/no-loop-statements": "error",
    "functional/no-promise-reject": "error",
    // Run by the project's linter (tools/lint, EN-075).
    "functional/no-return-void": "off",
    "functional/no-throw-statements": ["error", { allowToRejectPromises: false }],
    "functional/no-try-statements": ["error", { allowCatch: false, allowFinally: false }],
    // Point-free callbacks pass extra arguments (index, array): unicorn/no-array-callback-reference forbids them.
    "functional/prefer-tacit": "off",
  },
};

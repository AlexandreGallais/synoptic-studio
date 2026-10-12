import { ALLOWED_NUMBERS } from "../../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** Type-aware versions of core rules (the core versions are off in rules/core). */
export const TYPESCRIPT_EXTENSIONS: TSESLint.FlatConfig.Config = {
  name: "typescript/extensions",
  rules: {
    // Conflicts with unicorn/no-useless-undefined on `T | undefined` returns; TypeScript's noImplicitReturns covers it.
    "@typescript-eslint/consistent-return": "off",
    "@typescript-eslint/dot-notation": [
      "error",
      {
        allowIndexSignaturePropertyAccess: false,
        allowKeywords: true,
        allowPrivateClassPropertyAccess: false,
        allowProtectedClassPropertyAccess: false,
      },
    ],
    "@typescript-eslint/init-declarations": ["error", "always"],
    "@typescript-eslint/no-array-constructor": "error",
    "@typescript-eslint/no-implied-eval": "error",
    "@typescript-eslint/no-invalid-this": "error",
    "@typescript-eslint/no-magic-numbers": [
      "error",
      {
        detectObjects: false,
        enforceConst: true,
        ignore: ALLOWED_NUMBERS,
        ignoreArrayIndexes: true,
        ignoreClassFieldInitialValues: false,
        ignoreDefaultValues: false,
        ignoreEnums: false,
        ignoreNumericLiteralTypes: true,
        ignoreReadonlyClassProperties: false,
        ignoreTypeIndexes: true,
      },
    ],
    // TypeScript reports redeclarations itself (2451) and the rule misreads overloads.
    "@typescript-eslint/no-redeclare": "off",
    "@typescript-eslint/no-shadow": [
      "error",
      {
        builtinGlobals: false,
        hoist: "all",
        ignoreFunctionTypeParameterNameValueShadow: false,
        ignoreTypeValueShadow: false,
      },
    ],
    "@typescript-eslint/no-unused-expressions": [
      "error",
      {
        allowShortCircuit: false,
        allowTaggedTemplates: false,
        allowTernary: false,
        enforceForJSX: true,
      },
    ],
    // Superseded by unused-imports/no-unused-vars (same engine, plus import autofix).
    "@typescript-eslint/no-unused-vars": "off",
    // Functions may be used before their declaration: a module reads top-down, from the formula to its steps.
    "@typescript-eslint/no-use-before-define": [
      "error",
      { classes: true, enums: true, functions: false, typedefs: false, variables: true },
    ],
    // Arrays: indexed access stays explicit (noUncheckedIndexedAccess).
    "@typescript-eslint/prefer-destructuring": [
      "error",
      { array: false, object: true },
      { enforceForRenamedProperties: false },
    ],
    "@typescript-eslint/prefer-find": "error",
    "@typescript-eslint/prefer-for-of": "error",
    // Superseded by unicorn/prefer-includes, which also covers `Array#some`.
    "@typescript-eslint/prefer-includes": "off",
    "@typescript-eslint/prefer-regexp-exec": "error",
    // Superseded by unicorn/prefer-string-starts-ends-with, which also covers regular expressions.
    "@typescript-eslint/prefer-string-starts-ends-with": "off",
  },
};

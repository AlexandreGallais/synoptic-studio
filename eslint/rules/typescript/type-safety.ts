import { RESTRICTED_TYPES } from "../../settings";

import type { TSESLint } from "@typescript-eslint/utils";

/** `any`, unsafe operations and type assertions: the type system is never bypassed. */
export const TYPESCRIPT_TYPE_SAFETY: TSESLint.FlatConfig.Config = {
  name: "typescript/type-safety",
  rules: {
    "@typescript-eslint/ban-ts-comment": [
      "error",
      {
        minimumDescriptionLength: 10,
        "ts-check": true,
        "ts-expect-error": "allow-with-description",
        "ts-ignore": true,
        "ts-nocheck": true,
      },
    ],
    "@typescript-eslint/ban-tslint-comment": "error",
    // No `as`: an exception needs an eslint-disable comment with its justification.
    "@typescript-eslint/consistent-type-assertions": ["error", { assertionStyle: "never" }],
    "@typescript-eslint/no-base-to-string": "error",
    "@typescript-eslint/no-confusing-non-null-assertion": "error",
    "@typescript-eslint/no-explicit-any": ["error", { fixToUnknown: true, ignoreRestArgs: false }],
    "@typescript-eslint/no-extra-non-null-assertion": "error",
    "@typescript-eslint/no-non-null-asserted-nullish-coalescing": "error",
    "@typescript-eslint/no-non-null-asserted-optional-chain": "error",
    "@typescript-eslint/no-non-null-assertion": "error",
    "@typescript-eslint/no-restricted-types": ["error", { types: RESTRICTED_TYPES }],
    "@typescript-eslint/no-unnecessary-type-assertion": "error",
    "@typescript-eslint/no-unsafe-argument": "error",
    "@typescript-eslint/no-unsafe-assignment": "error",
    "@typescript-eslint/no-unsafe-call": "error",
    "@typescript-eslint/no-unsafe-declaration-merging": "error",
    "@typescript-eslint/no-unsafe-enum-assignment": "error",
    "@typescript-eslint/no-unsafe-enum-comparison": "error",
    "@typescript-eslint/no-unsafe-function-type": "error",
    "@typescript-eslint/no-unsafe-member-access": "error",
    "@typescript-eslint/no-unsafe-return": "error",
    "@typescript-eslint/no-unsafe-type-assertion": "error",
    "@typescript-eslint/no-unsafe-unary-minus": "error",
    "@typescript-eslint/no-wrapper-object-types": "error",
    // Contradicts no-non-null-assertion: both `!` and `as` are banned.
    "@typescript-eslint/non-nullable-type-assertion-style": "off",
    "@typescript-eslint/prefer-reduce-type-parameter": "error",
    "@typescript-eslint/restrict-plus-operands": [
      "error",
      {
        allowAny: false,
        allowBoolean: false,
        allowNullish: false,
        allowNumberAndString: false,
        allowRegExp: false,
        skipCompoundAssignments: false,
      },
    ],
    // Numbers are interpolated in SVG path data; everything else is converted explicitly.
    "@typescript-eslint/restrict-template-expressions": [
      "error",
      {
        allow: [],
        allowAny: false,
        allowBoolean: false,
        allowNever: false,
        allowNullish: false,
        allowNumber: true,
        allowRegExp: false,
      },
    ],
    "@typescript-eslint/unbound-method": "error",
    "@typescript-eslint/use-unknown-in-catch-callback-variable": "error",
  },
};

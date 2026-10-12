import type { TSESLint } from "@typescript-eslint/utils";

/** Data is never mutated: values are `readonly` and functions return new values. */
export const FUNCTIONAL_IMMUTABILITY: TSESLint.FlatConfig.Config = {
  name: "functional/immutability",
  rules: {
    "functional/immutable-data": [
      "error",
      { ignoreClasses: false, ignoreImmediateMutation: false, ignoreNonConstDeclarations: false },
    ],
    "functional/no-let": ["error", { allowInForLoopInit: false, allowInFunctions: false }],
    "functional/prefer-immutable-types": [
      "error",
      {
        enforcement: "ReadonlyDeep",
        ignoreInferredTypes: true,
        parameters: { enforcement: "ReadonlyDeep" },
        returnTypes: { enforcement: "None" },
        variables: { enforcement: "None" },
      },
    ],
    "functional/prefer-property-signatures": "error",
    "functional/readonly-type": ["error", "keyword"],
    "functional/type-declaration-immutability": [
      "error",
      {
        rules: [
          {
            comparator: "AtLeast",
            fixer: false,
            identifiers: "^.+$",
            immutability: "ReadonlyDeep",
          },
        ],
      },
    ],
  },
};

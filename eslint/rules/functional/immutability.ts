import type { TSESLint } from "@typescript-eslint/utils";

/** Data is never mutated: values are `readonly` and functions return new values. */
export const FUNCTIONAL_IMMUTABILITY: TSESLint.FlatConfig.Config = {
  name: "functional/immutability",
  rules: {
    // Run by the project's linter, without types (tools/lint, EN-076).
    "functional/immutable-data": "off",
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

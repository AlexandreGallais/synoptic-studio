import type { TSESLint } from "@typescript-eslint/utils";

/** Data is never mutated: values are `readonly` and functions return new values. */
export const FUNCTIONAL_IMMUTABILITY: TSESLint.FlatConfig.Config = {
  name: "functional/immutability",
  rules: {
    // Run by the project's linter, without types and in their strict form (tools/lint, EN-076).
    "functional/immutable-data": "off",
    // Run by the project's linter (tools/lint, EN-084).
    "functional/no-let": "off",
    "functional/prefer-immutable-types": "off",
    "functional/prefer-property-signatures": "error",
    // Run by the project's linter (tools/lint, EN-084).
    "functional/readonly-type": "off",
    "functional/type-declaration-immutability": "off",
  },
};

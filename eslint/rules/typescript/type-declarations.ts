import type { TSESLint } from "@typescript-eslint/utils";

/** Shape of type declarations. */
export const TYPESCRIPT_TYPE_DECLARATIONS: TSESLint.FlatConfig.Config = {
  name: "typescript/type-declarations",
  rules: {
    "@typescript-eslint/adjacent-overload-signatures": "error",
    "@typescript-eslint/array-type": ["error", { default: "array", readonly: "array" }],
    "@typescript-eslint/consistent-generic-constructors": ["error", "constructor"],
    "@typescript-eslint/consistent-indexed-object-style": ["error", "record"],
    // Closed type aliases: no declaration merging.
    "@typescript-eslint/consistent-type-definitions": ["error", "type"],
    "@typescript-eslint/method-signature-style": ["error", "property"],
    "@typescript-eslint/no-duplicate-enum-values": "error",
    "@typescript-eslint/no-empty-object-type": [
      "error",
      { allowInterfaces: "never", allowObjectTypes: "never" },
    ],
    "@typescript-eslint/no-generated-empty-object-type": "error",
    "@typescript-eslint/no-inferrable-types": [
      "error",
      { ignoreParameters: false, ignoreProperties: false },
    ],
    "@typescript-eslint/no-misused-new": "error",
    "@typescript-eslint/no-mixed-enums": "error",
    "@typescript-eslint/no-namespace": [
      "error",
      { allowDeclarations: false, allowDefinitionFiles: false },
    ],
    "@typescript-eslint/prefer-as-const": "error",
    "@typescript-eslint/prefer-enum-initializers": "error",
    "@typescript-eslint/prefer-function-type": "error",
    "@typescript-eslint/prefer-literal-enum-member": ["error", { allowBitwiseExpressions: false }],
    "@typescript-eslint/prefer-namespace-keyword": "error",
    "@typescript-eslint/related-getter-setter-pairs": "error",
    "@typescript-eslint/triple-slash-reference": [
      "error",
      { lib: "never", path: "never", types: "never" },
    ],
    "@typescript-eslint/unified-signatures": "error",
  },
};

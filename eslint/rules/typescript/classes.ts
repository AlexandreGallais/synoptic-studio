import type { TSESLint } from "@typescript-eslint/utils";

/** Classes (banned by functional/no-classes; these rules guard the remainder). */
export const TYPESCRIPT_CLASSES: TSESLint.FlatConfig.Config = {
  name: "typescript/classes",
  rules: {
    "@typescript-eslint/class-literal-property-style": ["error", "fields"],
    "@typescript-eslint/class-methods-use-this": "error",
    "@typescript-eslint/explicit-member-accessibility": "error",
    "@typescript-eslint/member-ordering": "error",
    "@typescript-eslint/no-dupe-class-members": "error",
    "@typescript-eslint/no-extraneous-class": "error",
    "@typescript-eslint/no-this-alias": ["error", { allowDestructuring: false }],
    "@typescript-eslint/no-unused-private-class-members": "error",
    "@typescript-eslint/no-useless-constructor": "error",
    "@typescript-eslint/parameter-properties": "error",
    "@typescript-eslint/prefer-readonly": "error",
  },
};

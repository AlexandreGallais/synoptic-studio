import type { TSESLint } from "@typescript-eslint/utils";

/** Type-only imports and exports (`verbatimModuleSyntax`). */
export const TYPESCRIPT_TYPE_IMPORTS: TSESLint.FlatConfig.Config = {
  name: "typescript/type-imports",
  rules: {
    "@typescript-eslint/consistent-type-exports": [
      "error",
      { fixMixedExportsWithInlineTypeSpecifier: false },
    ],
    "@typescript-eslint/consistent-type-imports": [
      "error",
      { disallowTypeAnnotations: true, fixStyle: "separate-type-imports", prefer: "type-imports" },
    ],
    "@typescript-eslint/no-import-type-side-effects": "error",
    "@typescript-eslint/no-require-imports": "error",
  },
};

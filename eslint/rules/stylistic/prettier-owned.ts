import type { TSESLint } from "@typescript-eslint/utils";

/** Layout rules left to Prettier (not listed by eslint-config-prettier, but formatting nonetheless). */
export const STYLISTIC_PRETTIER_OWNED: TSESLint.FlatConfig.Config = {
  name: "stylistic/prettier-owned",
  rules: {
    // Brace line breaks are Prettier's.
    "@stylistic/curly-newline": "off",
    // No JSX in the project.
    "@stylistic/exp-jsx-props-style": "off",
    // Bracket line breaks are Prettier's (experimental rule).
    "@stylistic/exp-list-style": "off",
    // No JSX in the project.
    "@stylistic/jsx-curly-brace-presence": "off",
    // No JSX in the project.
    "@stylistic/jsx-function-call-newline": "off",
    // No JSX in the project.
    "@stylistic/jsx-pascal-case": "off",
    // No JSX in the project.
    "@stylistic/jsx-self-closing-comp": "off",
  },
};

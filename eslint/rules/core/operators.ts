import type { TSESLint } from "@typescript-eslint/utils";

/** Operators, comparisons, coercions and literals. */
export const CORE_OPERATORS: TSESLint.FlatConfig.Config = {
  name: "core/operators",
  rules: {
    eqeqeq: ["error", "always"],
    "logical-assignment-operators": ["error", "always", { enforceForIfStatements: true }],
    "no-bitwise": "error",
    "no-compare-neg-zero": "error",
    "no-cond-assign": ["error", "always"],
    "no-constant-binary-expression": "error",
    "no-eq-null": "error",
    "no-extra-boolean-cast": "error",
    "no-implicit-coercion": ["error", { allow: [], disallowTemplateShorthand: true }],
    "no-loss-of-precision": "error",
    // Superseded by the type-aware @typescript-eslint/no-magic-numbers.
    "no-magic-numbers": "off",
    "no-multi-assign": "error",
    "no-nonoctal-decimal-escape": "error",
    "no-octal": "error",
    "no-octal-escape": "error",
    "no-plusplus": "error",
    "no-return-assign": ["error", "always"],
    "no-self-assign": "error",
    "no-self-compare": "error",
    "no-sequences": ["error", { allowInParentheses: false }],
    "no-template-curly-in-string": "error",
    // The ternary is the expression form of a choice, required by functional/no-conditional-statements.
    "no-ternary": "off",
    "no-unneeded-ternary": ["error", { defaultAssignment: false }],
    "no-unsafe-negation": ["error", { enforceForOrderingRelations: true }],
    "no-unsafe-optional-chaining": ["error", { disallowArithmeticOperators: true }],
    // Superseded by the type-aware @typescript-eslint/no-unused-expressions.
    "no-unused-expressions": "off",
    // Superseded by unicorn/no-useless-concat (autofixable).
    "no-useless-concat": "off",
    "no-void": "error",
    "operator-assignment": ["error", "always"],
    "prefer-exponentiation-operator": "error",
    "prefer-numeric-literals": "error",
    "prefer-template": "error",
    radix: "error",
    "use-isnan": ["error", { enforceForIndexOf: true, enforceForSwitchCase: true }],
    "valid-typeof": ["error", { requireStringLiterals: true }],
    yoda: ["error", "never"],
  },
};

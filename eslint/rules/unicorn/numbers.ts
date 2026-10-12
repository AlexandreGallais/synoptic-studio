import type { TSESLint } from "@typescript-eslint/utils";

/** Numbers, arithmetic and comparisons. */
export const UNICORN_NUMBERS: TSESLint.FlatConfig.Config = {
  name: "unicorn/numbers",
  rules: {
    "unicorn/no-accidental-bitwise-operator": "error",
    "unicorn/no-chained-comparison": "error",
    "unicorn/no-constant-zero-expression": "error",
    "unicorn/no-double-comparison": "error",
    "unicorn/no-negated-comparison": "error",
    "unicorn/no-redundant-comparison": "error",
    "unicorn/no-subtraction-comparison": "error",
    // Duplicate of the type-aware @typescript-eslint/no-unnecessary-boolean-literal-compare.
    "unicorn/no-unnecessary-boolean-comparison": "off",
    "unicorn/no-useless-coercion": "error",
    "unicorn/no-xor-as-exponentiation": "error",
    "unicorn/no-zero-fractions": "error",
    "unicorn/numeric-separators-style": "error",
    "unicorn/prefer-bigint-literals": "error",
    "unicorn/prefer-math-abs": "error",
    "unicorn/prefer-math-constants": "error",
    "unicorn/prefer-math-min-max": "error",
    "unicorn/prefer-math-trunc": "error",
    "unicorn/prefer-modern-math-apis": "error",
    "unicorn/prefer-native-coercion-functions": "error",
    "unicorn/prefer-number-coercion": "error",
    "unicorn/prefer-number-is-safe-integer": "error",
    "unicorn/prefer-number-properties": "error",
    "unicorn/prefer-unary-minus": "error",
    "unicorn/require-number-to-fixed-digits-argument": "error",
  },
};

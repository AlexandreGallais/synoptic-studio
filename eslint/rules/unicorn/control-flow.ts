import type { TSESLint } from "@typescript-eslint/utils";

/** Conditions, branches and scopes. */
export const UNICORN_CONTROL_FLOW: TSESLint.FlatConfig.Config = {
  name: "unicorn/control-flow",
  rules: {
    // Duplicate of the core rule of the same name (rules/core/operators.ts).
    "unicorn/logical-assignment-operators": "off",
    "unicorn/no-declarations-before-early-exit": "error",
    "unicorn/no-duplicate-if-branches": "error",
    "unicorn/no-duplicate-logical-operands": "error",
    "unicorn/no-lonely-if": "error",
    "unicorn/no-negated-condition": "error",
    "unicorn/no-negation-in-equality-check": "error",
    "unicorn/no-typeof-undefined": "error",
    "unicorn/no-unnecessary-nested-ternary": "error",
    "unicorn/no-useless-boolean-cast": "error",
    "unicorn/no-useless-else": "error",
    "unicorn/no-useless-logical-operand": "error",
    "unicorn/no-useless-switch-case": "error",
    "unicorn/no-useless-undefined": "error",
    "unicorn/prefer-boolean-return": "error",
    "unicorn/prefer-combined-guards": "error",
    "unicorn/prefer-early-return": "error",
    "unicorn/prefer-else-if": "error",
    "unicorn/prefer-hoisting-branch-code": "error",
    "unicorn/prefer-logical-operator-over-ternary": "error",
    "unicorn/prefer-minimal-ternary": "error",
    "unicorn/prefer-simple-condition-first": "error",
    "unicorn/prefer-simplified-conditions": "error",
    "unicorn/prefer-smaller-scope": "error",
    "unicorn/prefer-switch": "error",
    "unicorn/prefer-ternary": "error",
    "unicorn/switch-case-braces": "error",
    "unicorn/switch-case-break-position": "error",
  },
};

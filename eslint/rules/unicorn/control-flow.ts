import type { TSESLint } from "@typescript-eslint/utils";

/** Conditions, branches and scopes. */
export const UNICORN_CONTROL_FLOW: TSESLint.FlatConfig.Config = {
  name: "unicorn/control-flow",
  rules: {
    // Duplicate of the core rule of the same name (rules/core/operators.ts).
    "unicorn/logical-assignment-operators": "off",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/no-declarations-before-early-exit": "off",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/no-duplicate-if-branches": "off",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/no-duplicate-logical-operands": "off",
    "unicorn/no-lonely-if": "error",
    "unicorn/no-negated-condition": "error",
    "unicorn/no-negation-in-equality-check": "error",
    "unicorn/no-typeof-undefined": "error",
    "unicorn/no-unnecessary-nested-ternary": "error",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/no-useless-boolean-cast": "off",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/no-useless-else": "off",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/no-useless-logical-operand": "off",
    "unicorn/no-useless-switch-case": "error",
    "unicorn/no-useless-undefined": "error",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/prefer-boolean-return": "off",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/prefer-combined-guards": "off",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/prefer-early-return": "off",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/prefer-else-if": "off",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/prefer-hoisting-branch-code": "off",
    "unicorn/prefer-logical-operator-over-ternary": "error",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/prefer-minimal-ternary": "off",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/prefer-simple-condition-first": "off",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/prefer-simplified-conditions": "off",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/prefer-smaller-scope": "off",
    // Run by the project's linter (tools/lint, EN-079).
    "unicorn/prefer-switch": "off",
    "unicorn/prefer-ternary": "error",
    "unicorn/switch-case-braces": "error",
    "unicorn/switch-case-break-position": "error",
  },
};

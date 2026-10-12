import type { TSESLint } from "@typescript-eslint/utils";

/** Branches, loops, switches and reachability. */
export const CORE_CONTROL_FLOW: TSESLint.FlatConfig.Config = {
  name: "core/control-flow",
  rules: {
    // Compatible with Prettier in the `all` mode (eslint-config-prettier, special rules).
    curly: ["error", "all"],
    // Exhaustiveness is checked on union types by @typescript-eslint/switch-exhaustiveness-check; a default case would hide a missing member.
    "default-case": "off",
    "default-case-last": "error",
    "for-direction": "error",
    "guard-for-in": "error",
    "no-case-declarations": "error",
    "no-constant-condition": "error",
    "no-continue": "error",
    "no-dupe-else-if": "error",
    "no-duplicate-case": "error",
    "no-else-return": ["error", { allowElseIf: false }],
    "no-empty": "error",
    "no-extra-label": "error",
    "no-fallthrough": "error",
    "no-labels": "error",
    "no-lone-blocks": "error",
    "no-lonely-if": "error",
    // Superseded by unicorn/no-negated-condition (autofixable).
    "no-negated-condition": "off",
    "no-unmodified-loop-condition": "error",
    "no-unreachable": "error",
    "no-unreachable-loop": "error",
    "no-unsafe-finally": "error",
    "no-unused-labels": "error",
  },
};

import type { TSESLint } from "@typescript-eslint/utils";

/** Maps, sets and spreads. */
export const UNICORN_COLLECTIONS: TSESLint.FlatConfig.Config = {
  name: "unicorn/collections",
  rules: {
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/consistent-conditional-object-spread": "off",
    "unicorn/consistent-empty-array-spread": "error",
    "unicorn/no-collection-bracket-access": "error",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/no-duplicate-set-values": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/no-mismatched-map-key": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/no-object-methods-with-collections": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/no-unnecessary-array-flat-map": "off",
    "unicorn/no-useless-collection-argument": "error",
    "unicorn/no-useless-fallback-in-spread": "error",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/no-useless-set-construction": "off",
    "unicorn/no-useless-spread": "error",
    "unicorn/prefer-array-flat-map": "error",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-array-from-map": "off",
    "unicorn/prefer-get-or-insert-computed": "error",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-group-by": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-has-check": "off",
    "unicorn/prefer-includes": "error",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-includes-over-repeated-comparisons": "off",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-map-from-entries": "off",
    "unicorn/prefer-set-has": "error",
    // Run by the project's linter (tools/lint, EN-078).
    "unicorn/prefer-set-methods": "off",
    "unicorn/prefer-set-size": "error",
    "unicorn/prefer-spread": "error",
  },
};

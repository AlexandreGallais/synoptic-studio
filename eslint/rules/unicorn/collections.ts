import type { TSESLint } from "@typescript-eslint/utils";

/** Maps, sets and spreads. */
export const UNICORN_COLLECTIONS: TSESLint.FlatConfig.Config = {
  name: "unicorn/collections",
  rules: {
    "unicorn/consistent-conditional-object-spread": "error",
    "unicorn/consistent-empty-array-spread": "error",
    "unicorn/no-collection-bracket-access": "error",
    "unicorn/no-duplicate-set-values": "error",
    "unicorn/no-mismatched-map-key": "error",
    "unicorn/no-object-methods-with-collections": "error",
    "unicorn/no-unnecessary-array-flat-map": "error",
    "unicorn/no-useless-collection-argument": "error",
    "unicorn/no-useless-fallback-in-spread": "error",
    "unicorn/no-useless-set-construction": "error",
    "unicorn/no-useless-spread": "error",
    "unicorn/prefer-array-flat-map": "error",
    "unicorn/prefer-array-from-map": "error",
    "unicorn/prefer-get-or-insert-computed": "error",
    "unicorn/prefer-group-by": "error",
    "unicorn/prefer-has-check": "error",
    "unicorn/prefer-includes": "error",
    "unicorn/prefer-includes-over-repeated-comparisons": "error",
    "unicorn/prefer-map-from-entries": "error",
    "unicorn/prefer-set-has": "error",
    "unicorn/prefer-set-methods": "error",
    "unicorn/prefer-set-size": "error",
    "unicorn/prefer-spread": "error",
  },
};

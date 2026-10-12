/**
 * Statement-level purity rules relaxed outside the functional core (tests and tooling).
 *
 * Kept in one place so that test and tooling scopes relax exactly the same set.
 */
export const RELAXED_PURITY = {
  "functional/functional-parameters": "off",
  "functional/immutable-data": "off",
  "functional/no-conditional-statements": "off",
  "functional/no-expression-statements": "off",
  "functional/no-loop-statements": "off",
  "functional/no-return-void": "off",
  "functional/no-throw-statements": "off",
  "functional/no-try-statements": "off",
  "functional/prefer-immutable-types": "off",
  "functional/type-declaration-immutability": "off",
  "unicorn/no-top-level-side-effects": "off",
} as const;

/** Rules about the library's function kinds, irrelevant outside `src/` and `playground/`. */
export const KINDLESS = {
  "local/kind-in-layer": "off",
  "local/kind-limits": "off",
  "local/kind-naming": "off",
  "local/require-kind": "off",
  "local/see-references": "off",
} as const;

/**
 * Rules relaxed outside the functional core (tests and tooling); the statement-level purity
 * rules moved to the project's linter with their scopes (tools/lint, EN-084).
 *
 * Kept in one place so that test and tooling scopes relax exactly the same set.
 */
export const RELAXED_PURITY = {
  "unicorn/no-top-level-side-effects": "off",
} as const;

/**
 * Global ceilings on functions for the core ESLint rules: the loosest of the per-kind limits of
 * ADR-0011, which the project's linter checks by kind (`tools/lint/src/settings.rs`).
 */
export const GLOBAL_FUNCTION_CEILING = {
  maxComplexity: 5,
  maxDepth: 2,
  maxLines: 40,
  maxNestedCallbacks: 2,
  maxParams: 3,
  maxStatements: 15,
};

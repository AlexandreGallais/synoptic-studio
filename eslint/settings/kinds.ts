/**
 * Function kinds (`@kind` JSDoc tag) and the limits attached to each kind.
 *
 * Rationale and sources: ADR-0011, docs/conventions/functions.md.
 * Lines exclude blank and comment lines; complexity is McCabe's (decision points + 1),
 * callbacks included; depth is block nesting.
 */
export const KIND_LIMITS = {
  math: { maxComplexity: 3, maxDepth: 1, maxLines: 10, maxStatements: 4 },
  geometry: { maxComplexity: 5, maxDepth: 2, maxLines: 20, maxStatements: 8 },
  domain: { maxComplexity: 5, maxDepth: 2, maxLines: 20, maxStatements: 8 },
  format: { maxComplexity: 4, maxDepth: 1, maxLines: 20, maxStatements: 8 },
  procedure: { maxComplexity: 3, maxDepth: 1, maxLines: 40, maxStatements: 15 },
};

/** Names of all function kinds, in pipeline order. */
export const KIND_NAMES = Object.keys(KIND_LIMITS);

/** Loosest limits over all kinds, used as global ceilings by the core ESLint rules. */
export const GLOBAL_FUNCTION_CEILING = {
  maxComplexity: 5,
  maxDepth: 2,
  maxLines: 40,
  maxNestedCallbacks: 2,
  maxParams: 3,
  maxStatements: 15,
};

/** Leading words reserved for boolean-returning functions (predicates). */
export const PREDICATE_PREFIXES = [
  "are",
  "can",
  "contains",
  "equals",
  "has",
  "includes",
  "intersects",
  "is",
  "should",
];

/** Vague leading verbs forbidden on pure kinds: the name must state the concept itself. */
export const VAGUE_PREFIXES = [
  "calc",
  "calculate",
  "compute",
  "do",
  "get",
  "handle",
  "make",
  "perform",
  "process",
];

/** Name patterns of `format` functions: conversions read as `xToY`, `toX`, `parseX`… */
export const FORMAT_NAME_PATTERN =
  "^(?:(?:to|from|parse|serialize|format|encode|decode)[A-Z]|[a-z][A-Za-z0-9]*To[A-Z])";

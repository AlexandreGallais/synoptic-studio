import { TOP_LEVEL_FUNCTION_SELECTOR, createRule, functionMetrics, kindOf } from "../utils";

import type { FunctionMetrics, TopLevelFunction } from "../utils";

/** Limits of one kind. */
type KindLimit = {
  readonly maxComplexity: number;
  readonly maxDepth: number;
  readonly maxLines: number;
  readonly maxStatements: number;
};

/** Options of `local/kind-limits`. */
type Options = [{ readonly limits: Readonly<Record<string, KindLimit>> }];

/** One metric exceeding its limit. */
type Excess = {
  readonly actual: number;
  readonly max: number;
  readonly metric: keyof FunctionMetrics;
};

/** Correspondence between a measured metric and the limit bounding it. */
const BOUNDS: readonly (readonly [keyof FunctionMetrics, keyof KindLimit])[] = [
  ["complexity", "maxComplexity"],
  ["depth", "maxDepth"],
  ["lines", "maxLines"],
  ["statements", "maxStatements"],
];

/**
 * Metrics of a function exceeding the limits of its kind.
 *
 * @param metrics - measured metrics
 * @param limit - limits of the function's kind, `undefined` for an unknown kind
 * @returns every exceeded metric, in `BOUNDS` order
 */
function excesses(metrics: FunctionMetrics, limit: KindLimit | undefined): Excess[] {
  return limit === undefined
    ? []
    : BOUNDS.map(([metric, bound]) => ({
        actual: metrics[metric],
        max: limit[bound],
        metric,
      })).filter((excess) => excess.actual > excess.max);
}

/**
 * `local/kind-limits`: size and complexity limits depending on the function's `@kind`.
 *
 * @see ADR-0011
 */
export const KIND_LIMITS_RULE = createRule<Options, "exceeded">({
  create: (context, [{ limits }]) => ({
    [TOP_LEVEL_FUNCTION_SELECTOR]: (node: TopLevelFunction): void => {
      const kind = kindOf(context.sourceCode, node) ?? "";
      const metrics = functionMetrics(context.sourceCode, node);

      const exceeded = excesses(metrics, limits[kind]);

      for (const excess of exceeded) {
        context.report({ data: { ...excess, kind }, messageId: "exceeded", node: node.id ?? node });
      }
    },
  }),
  defaultOptions: [{ limits: {} }],
  meta: {
    docs: { description: "Bound lines, statements, complexity and depth by function kind." },
    messages: {
      exceeded:
        "`{{kind}}` function {{metric}} is {{actual}} (max {{max}}). Extract named steps (Composed Method).",
    },
    schema: [
      {
        additionalProperties: false,
        properties: { limits: { additionalProperties: { type: "object" }, type: "object" } },
        required: ["limits"],
        type: "object",
      },
    ],
    type: "suggestion",
  },
  name: "kind-limits",
});

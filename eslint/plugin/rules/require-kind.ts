import { TOP_LEVEL_FUNCTION_SELECTOR, createRule, jsdocOf, tagValues } from "../utils";

import type { TopLevelFunction } from "../utils";

/** Options of `local/require-kind`. */
type Options = [{ readonly kinds: readonly string[] }];

/** Message identifiers of `local/require-kind`. */
type MessageIds = "missing" | "multiple" | "unknown";

/**
 * Problem with the `@kind` tags of a function, if any.
 *
 * @param values - values of every `@kind` tag of the function
 * @param kinds - allowed kinds
 * @returns the message to report, or `undefined` when exactly one known kind is present
 */
function kindProblem(values: readonly string[], kinds: readonly string[]): MessageIds | undefined {
  const [kind] = values;

  if (kind === undefined) {
    return "missing";
  }

  if (values.length > 1) {
    return "multiple";
  }

  return kinds.includes(kind) ? undefined : "unknown";
}

/**
 * `local/require-kind`: every top-level function carries exactly one known `@kind` tag.
 *
 * @see ADR-0011
 */
export const REQUIRE_KIND_RULE = createRule<Options, MessageIds>({
  create: (context, [{ kinds }]) => ({
    [TOP_LEVEL_FUNCTION_SELECTOR]: (node: TopLevelFunction): void => {
      const values = tagValues(jsdocOf(context.sourceCode, node), "kind");
      const messageId = kindProblem(values, kinds);

      if (messageId === undefined) {
        return;
      }

      const details = { kind: values[0] ?? "", kinds: kinds.join(", ") };

      context.report({ data: details, messageId, node: node.id ?? node });
    },
  }),
  defaultOptions: [{ kinds: [] }],
  meta: {
    docs: { description: "Require exactly one known `@kind` JSDoc tag on top-level functions." },
    messages: {
      missing: "Top-level function without `@kind` tag (ADR-0011).",
      multiple: "A function has exactly one `@kind`: split it if it mixes concerns.",
      unknown: "Unknown kind `{{kind}}`; expected one of: {{kinds}}.",
    },
    schema: [
      {
        additionalProperties: false,
        properties: { kinds: { items: { type: "string" }, type: "array" } },
        required: ["kinds"],
        type: "object",
      },
    ],
    type: "problem",
  },
  name: "require-kind",
});

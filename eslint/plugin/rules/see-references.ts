import { AST_TOKEN_TYPES } from "@typescript-eslint/utils";

import {
  TOP_LEVEL_FUNCTION_SELECTOR,
  createRule,
  jsdocOf,
  kindOf,
  referenceStatus,
  tagValues,
} from "../utils";

import type { ReferenceSources, TopLevelFunction } from "../utils";

/** Options of `local/see-references`. */
type Options = [
  {
    readonly requiredKinds: readonly string[];
    readonly root?: string;
    readonly sources: ReferenceSources;
  },
];

/** Message identifiers of `local/see-references`. */
type MessageIds = "missing" | "missingSee" | "unknown-format" | "unverified";

/**
 * `local/see-references`: `@see` targets exist and are verified; required on kinds that need one.
 *
 * Guards against invented references (CLAUDE.md, "Interdits absolus").
 *
 * @see docs/conventions/documentation.md
 */
export const SEE_REFERENCES_RULE = createRule<Options, MessageIds>({
  create: (context, [{ requiredKinds, root = context.cwd, sources }]) => ({
    [TOP_LEVEL_FUNCTION_SELECTOR]: (node: TopLevelFunction): void => {
      const kind = kindOf(context.sourceCode, node) ?? "";
      const hasSee = tagValues(jsdocOf(context.sourceCode, node), "see").length > 0;

      if (!hasSee && requiredKinds.includes(kind)) {
        context.report({ data: { kind }, messageId: "missingSee", node: node.id ?? node });
      }
    },
    Program: (): void => {
      const referenceContext = { root, sources };
      const citations = context.sourceCode
        .getAllComments()
        .filter(
          (comment) => comment.type === AST_TOKEN_TYPES.Block && comment.value.startsWith("*"),
        )
        .flatMap((comment) => tagValues(comment, "see").map((target) => ({ comment, target })));

      for (const { comment, target } of citations) {
        const status = referenceStatus(referenceContext, target);

        if (status !== "valid") {
          context.report({ data: { target }, loc: comment.loc, messageId: status });
        }
      }
    },
  }),
  defaultOptions: [
    {
      requiredKinds: [],
      sources: {
        adrDirectory: "",
        derivationsDirectory: "",
        referencesFile: "",
        unverifiedMarker: "",
      },
    },
  ],
  meta: {
    docs: {
      description: "Require existing, verified `@see` references (REF-*, DERIV-*, ADR-*, docs/…).",
    },
    messages: {
      missing: "`@see {{target}}` does not exist in the documentation.",
      missingSee: "`{{kind}}` function without `@see`: cite a REF-*, DERIV-*, ADR-* or docs/ page.",
      "unknown-format": "`@see {{target}}`: expected REF-*, DERIV-*, ADR-NNNN or docs/…/file.md.",
      unverified:
        "`@see {{target}}` is marked [unverified] in docs/references.md: verify it before citing it.",
    },
    schema: [
      {
        additionalProperties: false,
        properties: {
          requiredKinds: { items: { type: "string" }, type: "array" },
          root: { type: "string" },
          sources: { type: "object" },
        },
        required: ["requiredKinds", "sources"],
        type: "object",
      },
    ],
    type: "problem",
  },
  name: "see-references",
});

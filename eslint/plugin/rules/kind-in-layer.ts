import { relative } from "node:path";

import { TOP_LEVEL_FUNCTION_SELECTOR, createRule, kindOf } from "../utils";

import type { TopLevelFunction } from "../utils";

/** Layer description consumed by `local/kind-in-layer`. */
type Layer = {
  readonly kinds: readonly string[];
  readonly name: string;
  readonly path: string;
};

/** Options of `local/kind-in-layer`. */
type Options = [{ readonly layers: readonly Layer[]; readonly root?: string }];

/**
 * Layer containing a file, matched by path prefix relative to the repository root.
 *
 * @param layers - layer table
 * @param path - file path relative to the root, `/`-separated
 * @returns the layer, or `undefined` outside any layer
 */
function layerOf(layers: readonly Layer[], path: string): Layer | undefined {
  return layers.find((layer) => path.startsWith(`${layer.path}/`));
}

/**
 * `local/kind-in-layer`: a function's kind is allowed in its file's layer.
 *
 * @see docs/conventions/architecture.md
 */
export const KIND_IN_LAYER_RULE = createRule<Options, "forbidden">({
  create: (context, [{ layers, root = context.cwd }]) => {
    const layer = layerOf(layers, relative(root, context.filename).replaceAll("\\", "/"));

    return {
      [TOP_LEVEL_FUNCTION_SELECTOR]: (node: TopLevelFunction): void => {
        const kind = kindOf(context.sourceCode, node);

        if (layer === undefined || kind === undefined || layer.kinds.includes(kind)) {
          return;
        }

        const details = { kind, kinds: layer.kinds.join(", "), layer: layer.name };

        context.report({ data: details, messageId: "forbidden", node: node.id ?? node });
      },
    };
  },
  defaultOptions: [{ layers: [] }],
  meta: {
    docs: { description: "Restrict the `@kind` values allowed in each architectural layer." },
    messages: {
      forbidden: "Kind `{{kind}}` is not allowed in layer `{{layer}}` (allowed: {{kinds}}).",
    },
    schema: [
      {
        additionalProperties: false,
        properties: {
          layers: {
            items: {
              additionalProperties: true,
              properties: {
                kinds: { items: { type: "string" }, type: "array" },
                name: { type: "string" },
                path: { type: "string" },
              },
              required: ["kinds", "name", "path"],
              type: "object",
            },
            type: "array",
          },
          root: { type: "string" },
        },
        required: ["layers"],
        type: "object",
      },
    ],
    type: "problem",
  },
  name: "kind-in-layer",
});

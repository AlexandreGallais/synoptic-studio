import { dirname } from "node:path";

import { AST_NODE_TYPES } from "@typescript-eslint/utils";

import { canonicalImport, createRule } from "../utils";

import type { TSESTree } from "@typescript-eslint/utils";

/** Message identifiers of `local/canonical-import-path`. */
type MessageIds = "ancestorBarrel" | "notCanonical";

/** Statements and expressions carrying a module specifier. */
type ModuleReference =
  | TSESTree.ExportAllDeclaration
  | TSESTree.ExportNamedDeclaration
  | TSESTree.ImportDeclaration
  | TSESTree.ImportExpression;

/** Module specifier of an import or re-export, with the literal holding it. */
type Specifier = { readonly literal: TSESTree.Literal; readonly value: string };

/**
 * Module specifier written as a string literal.
 *
 * @param node - import, export or dynamic import
 * @returns the specifier, or `undefined` when absent or computed
 */
function specifierOf(node: ModuleReference): Specifier | undefined {
  const { source } = node;

  return source?.type === AST_NODE_TYPES.Literal && typeof source.value === "string"
    ? { literal: source, value: source.value }
    : undefined;
}

/**
 * `local/canonical-import-path`: relative imports use the shortest legal path (ADR-0012).
 *
 * Same folder → the file (`./vector`). Other folder → the folder's barrel (`../geometry`),
 * never `index` nor an extension. Autofixed on save.
 *
 * @see ADR-0012
 */
export const CANONICAL_IMPORT_PATH_RULE = createRule<[], MessageIds>({
  create: (context) => {
    const directory = dirname(context.filename);

    /**
     * Reports a relative specifier that is not canonical.
     *
     * @param specifier - relative module specifier
     */
    function checkRelative(specifier: Specifier): void {
      const { literal, value } = specifier;
      const result = canonicalImport(directory, value);

      if (result.kind === "ancestor-barrel") {
        context.report({ messageId: "ancestorBarrel", node: literal });
      } else if (result.kind === "canonical" && result.specifier !== value) {
        const replacement = `${literal.raw.charAt(0)}${result.specifier}${literal.raw.charAt(0)}`;

        context.report({
          data: { expected: result.specifier },
          fix: (fixer) => fixer.replaceText(literal, replacement),
          messageId: "notCanonical",
          node: literal,
        });
      }
    }

    /**
     * Checks the specifier of an import or re-export when it is relative.
     *
     * @param node - import, export or dynamic import
     */
    function check(node: ModuleReference): void {
      const specifier = specifierOf(node);

      if (specifier?.value.startsWith(".") === true) {
        checkRelative(specifier);
      }
    }

    return {
      ExportAllDeclaration: check,
      ExportNamedDeclaration: check,
      ImportDeclaration: check,
      ImportExpression: check,
    };
  },
  defaultOptions: [],
  meta: {
    docs: {
      description:
        "Require the shortest legal relative import path (folder barrels, no index, no extension).",
    },
    fixable: "code",
    messages: {
      ancestorBarrel:
        "Importing the barrel of this folder or of an ancestor creates a cycle: import the file.",
      notCanonical: "Use the canonical import path `{{expected}}`.",
    },
    schema: [],
    type: "suggestion",
  },
  name: "canonical-import-path",
});

import { AST_NODE_TYPES, AST_TOKEN_TYPES } from "@typescript-eslint/utils";

import type { TSESLint, TSESTree } from "@typescript-eslint/utils";

/** Top-level function declaration, exported or not. */
export type TopLevelFunction = TSESTree.FunctionDeclaration;

/** AST selector matching every top-level function declaration. */
export const TOP_LEVEL_FUNCTION_SELECTOR = [
  "Program > FunctionDeclaration",
  "Program > ExportNamedDeclaration > FunctionDeclaration",
].join(", ");

/**
 * Node a JSDoc block is attached to: the export statement when the function is exported.
 *
 * @param node - top-level function declaration
 * @returns node preceded by the JSDoc block
 */
function documentedNode(node: TopLevelFunction): TSESTree.Node {
  const { parent } = node;

  return parent.type === AST_NODE_TYPES.ExportNamedDeclaration ? parent : node;
}

/**
 * JSDoc block (`/** … *\/`) immediately preceding a top-level function.
 *
 * @param sourceCode - source code of the linted file
 * @param node - top-level function declaration
 * @returns the JSDoc comment, or `undefined` when absent
 */
export function jsdocOf(
  sourceCode: Readonly<TSESLint.SourceCode>,
  node: TopLevelFunction,
): TSESTree.Comment | undefined {
  const comment = sourceCode.getCommentsBefore(documentedNode(node)).at(-1);

  return comment?.type === AST_TOKEN_TYPES.Block && comment.value.startsWith("*")
    ? comment
    : undefined;
}

/**
 * Values of every occurrence of a block tag in a JSDoc comment.
 *
 * @param comment - JSDoc comment
 * @param tag - tag name without `@`
 * @returns first word following each `@tag`, in order
 */
export function tagValues(comment: TSESTree.Comment | undefined, tag: string): string[] {
  const pattern = new RegExp(String.raw`@${tag}(?:[ \t]+(?<value>\S+))?`, "gu");

  return (comment?.value ?? "")
    .matchAll(pattern)
    .map((match) => match.groups?.["value"] ?? "")
    .toArray();
}

/**
 * Kind of a top-level function, when it carries exactly one `@kind` tag.
 *
 * @param sourceCode - source code of the linted file
 * @param node - top-level function declaration
 * @returns the kind value, or `undefined` when missing or ambiguous
 */
export function kindOf(
  sourceCode: Readonly<TSESLint.SourceCode>,
  node: TopLevelFunction,
): string | undefined {
  const kinds = tagValues(jsdocOf(sourceCode, node), "kind");

  return kinds.length === 1 ? kinds[0] : undefined;
}

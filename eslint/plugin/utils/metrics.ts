import { AST_NODE_TYPES } from "@typescript-eslint/utils";

import type { TSESLint, TSESTree } from "@typescript-eslint/utils";

/** Size and shape measurements of a function, compared with the limits of its kind. */
export type FunctionMetrics = {
  readonly complexity: number;
  readonly depth: number;
  readonly lines: number;
  readonly statements: number;
};

/** Node types adding one independent path (McCabe decision points). */
const DECISION_TYPES = new Set<string>([
  AST_NODE_TYPES.CatchClause,
  AST_NODE_TYPES.ConditionalExpression,
  AST_NODE_TYPES.DoWhileStatement,
  AST_NODE_TYPES.ForInStatement,
  AST_NODE_TYPES.ForOfStatement,
  AST_NODE_TYPES.ForStatement,
  AST_NODE_TYPES.IfStatement,
  AST_NODE_TYPES.LogicalExpression,
  AST_NODE_TYPES.WhileStatement,
]);

/** Node types opening a nesting level (same set as the core `max-depth` rule). */
const NESTING_TYPES = new Set<string>([
  AST_NODE_TYPES.DoWhileStatement,
  AST_NODE_TYPES.ForInStatement,
  AST_NODE_TYPES.ForOfStatement,
  AST_NODE_TYPES.ForStatement,
  AST_NODE_TYPES.IfStatement,
  AST_NODE_TYPES.SwitchStatement,
  AST_NODE_TYPES.TryStatement,
  AST_NODE_TYPES.WhileStatement,
]);

/**
 * Whether a node adds a decision point: branches, loops, `catch`, `case`, `&&`, `||`, `??`.
 *
 * @param node - any AST node
 * @returns `true` when the node adds an independent path
 */
function isDecision(node: TSESTree.Node): boolean {
  return (
    DECISION_TYPES.has(node.type) ||
    (node.type === AST_NODE_TYPES.SwitchCase && node.test !== null) ||
    (node.type === AST_NODE_TYPES.AssignmentExpression &&
      ["&&=", "||=", "??="].includes(node.operator))
  );
}

/**
 * Whether a node is a statement counted by `max-statements` (blocks themselves excluded).
 *
 * @param node - any AST node
 * @returns `true` for statements and declarations other than blocks
 */
function isStatement(node: TSESTree.Node): boolean {
  return (
    node.type !== AST_NODE_TYPES.BlockStatement &&
    (node.type.endsWith("Statement") || node.type.endsWith("Declaration"))
  );
}

/**
 * Direct children of a node, following the parser's visitor keys.
 *
 * @param sourceCode - source code giving the visitor keys
 * @param node - parent node
 * @returns child nodes in source order
 */
function childrenOf(
  sourceCode: Readonly<TSESLint.SourceCode>,
  node: TSESTree.Node,
): TSESTree.Node[] {
  const keys = sourceCode.visitorKeys[node.type] ?? [];

  return keys.flatMap((key) => [slot(node, key)].flat()).filter(isNode);
}

/**
 * Content of a visitor-key slot of a node: a node, a list of nodes, or nothing.
 *
 * @param node - parent node
 * @param key - visitor key
 * @returns the slot value, untyped
 */
function slot(node: TSESTree.Node, key: string): unknown {
  return Reflect.get(node, key);
}

/**
 * Type guard for AST nodes found in visitor-key slots.
 *
 * @param value - slot value
 * @returns `true` when the value is an AST node
 */
function isNode(value: unknown): value is TSESTree.Node {
  return typeof value === "object" && value !== null && "type" in value;
}

/**
 * Number of lines holding at least one token: blank and comment-only lines are skipped.
 *
 * @param sourceCode - source code of the linted file
 * @param node - measured node
 * @returns count of lines holding code
 */
function codeLines(sourceCode: Readonly<TSESLint.SourceCode>, node: TSESTree.Node): number {
  const lines = new Set(sourceCode.getTokens(node).flatMap((token) => linesOf(token.loc)));

  return lines.size;
}

/**
 * Line numbers covered by a source location.
 *
 * @param loc - source location
 * @returns every line number from start to end
 */
function linesOf(loc: Readonly<TSESTree.SourceLocation>): number[] {
  return Array.from(
    { length: loc.end.line - loc.start.line + 1 },
    (entry, index) => loc.start.line + index,
  );
}

/**
 * Recursive measurement of decision points, statements and nesting below a node.
 *
 * @param sourceCode - source code giving the visitor keys
 * @param node - root of the measured subtree
 * @returns partial metrics of the subtree
 */
function measure(
  sourceCode: Readonly<TSESLint.SourceCode>,
  node: TSESTree.Node,
): Omit<FunctionMetrics, "lines"> {
  const children = childrenOf(sourceCode, node).map((child) => measure(sourceCode, child));
  const own = { complexity: Number(isDecision(node)), statements: Number(isStatement(node)) };

  return {
    complexity: own.complexity + sum(children.map((child) => child.complexity)),
    depth:
      Number(NESTING_TYPES.has(node.type)) + Math.max(0, ...children.map((child) => child.depth)),
    statements: own.statements + sum(children.map((child) => child.statements)),
  };
}

/**
 * Sum of a list of numbers.
 *
 * @param values - numbers to add
 * @returns their sum, 0 for an empty list
 */
function sum(values: readonly number[]): number {
  return values.reduce((total, value) => total + value, 0);
}

/**
 * Metrics of a function: lines of code, statements, McCabe complexity, nesting depth.
 *
 * Complexity = decision points + 1 (McCabe 1976), nested callbacks included.
 *
 * @param sourceCode - source code of the linted file
 * @param node - measured function
 * @returns its metrics
 * @see REF-NIST-500-235
 */
export function functionMetrics(
  sourceCode: Readonly<TSESLint.SourceCode>,
  node: TSESTree.FunctionDeclaration,
): FunctionMetrics {
  const body = measure(sourceCode, node.body);

  return { ...body, complexity: body.complexity + 1, lines: codeLines(sourceCode, node) };
}

import { basename } from "node:path";

import { AST_NODE_TYPES } from "@typescript-eslint/utils";

import { SOURCE_EXTENSION, createRule } from "../utils";

import type { TSESTree } from "@typescript-eslint/utils";

/** Message identifiers of `local/one-export-per-file`. */
type MessageIds =
  | "exportList"
  | "filenameMismatch"
  | "multipleTypes"
  | "multipleValues"
  | "noExport"
  | "reexport"
  | "unrelatedType";

/** Exported declaration, classified as a runtime value or a type. */
type ExportedName = {
  readonly isType: boolean;
  readonly name: string;
  readonly node: TSESTree.Node;
};

/** Declarations exporting a type. */
const TYPE_DECLARATIONS = new Set<string>([
  AST_NODE_TYPES.TSInterfaceDeclaration,
  AST_NODE_TYPES.TSTypeAliasDeclaration,
]);

/**
 * Names exported by an `export <declaration>` statement.
 *
 * @param declaration - exported declaration
 * @returns one entry per declared name
 */
function declaredNames(declaration: TSESTree.NamedExportDeclarations): ExportedName[] {
  const isType = TYPE_DECLARATIONS.has(declaration.type);

  if (declaration.type === AST_NODE_TYPES.VariableDeclaration) {
    return declaration.declarations
      .map((declarator) => declarator.id)
      .filter((id) => id.type === AST_NODE_TYPES.Identifier)
      .map((id) => ({ isType, name: id.name, node: id }));
  }

  return "id" in declaration && declaration.id?.type === AST_NODE_TYPES.Identifier
    ? [{ isType, name: declaration.id.name, node: declaration.id }]
    : [];
}

/**
 * Exported declarations of a module, in source order.
 *
 * @param program - module root
 * @returns names declared by `export <declaration>` statements, `export { … }` lists excluded
 */
function exportedNames(program: TSESTree.Program): ExportedName[] {
  return program.body.flatMap((statement) =>
    statement.type === AST_NODE_TYPES.ExportNamedDeclaration && statement.declaration !== null
      ? declaredNames(statement.declaration)
      : [],
  );
}

/**
 * Statement exporting otherwise than `export <declaration>`, if any.
 *
 * @param program - module root
 * @returns the offending statement and its message, or `undefined`
 */
function irregularExport(
  program: TSESTree.Program,
): { readonly messageId: MessageIds; readonly node: TSESTree.Node } | undefined {
  const reexport = program.body.find(
    (statement) =>
      statement.type === AST_NODE_TYPES.ExportAllDeclaration ||
      (statement.type === AST_NODE_TYPES.ExportNamedDeclaration && statement.source !== null),
  );

  if (reexport !== undefined) {
    return { messageId: "reexport", node: reexport };
  }

  const list = program.body.find(
    (statement) =>
      statement.type === AST_NODE_TYPES.ExportNamedDeclaration && statement.declaration === null,
  );

  return list === undefined ? undefined : { messageId: "exportList", node: list };
}

/**
 * Pattern matching a name as a whole word.
 *
 * @param name - identifier to look for
 * @returns a regular expression bounded by word boundaries
 */
function wordPattern(name: string): RegExp {
  return new RegExp(String.raw`\b${name}\b`, "u");
}

/**
 * Names of the types referenced anywhere inside a node (annotations, parameters, return type).
 *
 * @param text - source text of the exported value's declaration
 * @param candidates - exported type names
 * @returns the candidates appearing as whole words in the declaration
 */
function referencedTypes(text: string, candidates: readonly string[]): ReadonlySet<string> {
  return new Set(candidates.filter((name) => wordPattern(name).test(text)));
}

/**
 * Kebab-case file name of an exported identifier: `formatSvgNumber` → `format-svg-number`.
 *
 * @param name - camelCase, PascalCase or UPPER_CASE identifier
 * @returns lower-case words joined by hyphens
 */
function kebabCase(name: string): string {
  return name
    .replaceAll(/(?<lower>[a-z0-9])(?<upper>[A-Z])/gu, "$<lower>-$<upper>")
    .replaceAll(/(?<acronym>[A-Z]+)(?<word>[A-Z][a-z])/gu, "$<acronym>-$<word>")
    .replaceAll("_", "-")
    .toLowerCase();
}

/** Problem found in a module, ready to be reported. */
type Problem = {
  readonly data?: Readonly<Record<string, string>>;
  readonly messageId: MessageIds;
  readonly node: TSESTree.Node;
};

/**
 * Problems for every exported name beyond the first one.
 *
 * @param entries - exported names of one category
 * @param messageId - message for the surplus names
 * @returns one problem per surplus name
 */
function surplus(entries: readonly ExportedName[], messageId: MessageIds): Problem[] {
  return entries
    .slice(1)
    .map((entry) => ({ data: { name: entry.name }, messageId, node: entry.node }));
}

/**
 * Problems for exported types that are not part of the exported value's declaration.
 *
 * @param value - the exported value
 * @param types - exported types
 * @param declarationText - source text of the value's declaration
 * @returns one problem per unrelated type
 */
function unrelatedTypes(
  value: ExportedName,
  types: readonly ExportedName[],
  declarationText: string,
): Problem[] {
  const contract = referencedTypes(
    declarationText,
    types.map((type) => type.name),
  );

  return types
    .filter((type) => type.name !== value.name && !contract.has(type.name))
    .map((type) => ({ data: { name: type.name }, messageId: "unrelatedType", node: type.node }));
}

/**
 * Every export problem of a module that has at least one export.
 *
 * @param exported - exported names, in source order
 * @param expected - file name without extension
 * @param textOf - source text of a node
 * @returns problems, empty when the module is well-formed
 */
function exportProblems(
  exported: readonly ExportedName[],
  expected: string,
  textOf: (node: TSESTree.Node) => string,
): Problem[] {
  const values = exported.filter((entry) => !entry.isType);
  const types = exported.filter((entry) => entry.isType);
  const [main] = values.length > 0 ? values : types;

  if (main === undefined) {
    return [];
  }

  const typeProblems =
    values.length === 0
      ? surplus(types, "multipleTypes")
      : unrelatedTypes(main, types, textOf(main.node.parent ?? main.node));

  return [
    ...surplus(values, "multipleValues"),
    ...typeProblems,
    ...filenameProblems(main, expected),
  ];
}

/**
 * Problem when the main export is not named like the file.
 *
 * @param main - the module's main export
 * @param expected - file name without extension
 * @returns a mismatch problem, or nothing
 */
function filenameProblems(main: ExportedName, expected: string): Problem[] {
  const fileName = kebabCase(main.name);

  return fileName === expected
    ? []
    : [
        {
          data: { expected, fileName, name: main.name },
          messageId: "filenameMismatch",
          node: main.node,
        },
      ];
}

/**
 * `local/one-export-per-file`: a module exports one value named like the file.
 *
 * Types may be exported next to the value only when they appear in its declaration (its
 * contract). A module without value exports exactly one type, named like the file.
 *
 * @see ADR-0019
 */
export const ONE_EXPORT_PER_FILE_RULE = createRule<[], MessageIds>({
  create: (context) => ({
    "Program:exit": (program: TSESTree.Program): void => {
      const irregular = irregularExport(program);
      const exported = exportedNames(program);

      if (irregular !== undefined) {
        context.report(irregular);
      } else if (exported.length === 0) {
        context.report({ loc: { column: 0, line: 1 }, messageId: "noExport" });
      }

      const expected = basename(context.filename, SOURCE_EXTENSION);
      const problems =
        irregular === undefined
          ? exportProblems(exported, expected, (node) => context.sourceCode.getText(node))
          : [];

      for (const problem of problems) {
        context.report(problem);
      }
    },
  }),
  defaultOptions: [],
  meta: {
    docs: { description: "Require one exported value per module, named like the file." },
    messages: {
      exportList:
        "Export the declaration itself (`export function …`), not an `export { … }` list.",
      filenameMismatch:
        "The file exports `{{name}}`: rename the file `{{name}}.ts` (found `{{expected}}.ts`).",
      multipleTypes:
        "A type-only module exports exactly one type; move `{{name}}` to its own kebab-case file.",
      multipleValues: "One exported value per module; move `{{name}}` to its own kebab-case file.",
      noExport: "A library module exports exactly one value or type.",
      reexport: "Re-exports belong to the folder's `index.ts`.",
      unrelatedType:
        "Type `{{name}}` is not part of the exported value's signature: move it to its own kebab-case file.",
    },
    schema: [],
    type: "problem",
  },
  name: "one-export-per-file",
});

import { readdirSync } from "node:fs";
import { dirname, join } from "node:path";

import { AST_NODE_TYPES } from "@typescript-eslint/utils";

import { INDEX_FILE, SOURCE_EXTENSION, createRule, hasIndex } from "../utils";

import type { TSESTree } from "@typescript-eslint/utils";
import type { Dirent } from "node:fs";

/** Message identifiers of `local/barrel-exports`. */
type MessageIds = "onlyReexports" | "outOfSync";

/** Suffixes of files that are not modules of the folder's public surface. */
const EXCLUDED_SUFFIXES = [".d.ts", ".test.ts"];

/**
 * Re-export specifier of a folder entry: a source module or a sub-folder with a barrel.
 *
 * @param directory - absolute folder path
 * @param entry - folder entry
 * @returns `./name`, or nothing for non-module entries
 */
function moduleSpecifier(directory: string, entry: Readonly<Dirent>): string[] {
  const { name } = entry;

  if (entry.isDirectory()) {
    return hasIndex(join(directory, name)) ? [`./${name}`] : [];
  }

  return isPublicModule(name) ? [`./${name.slice(0, -SOURCE_EXTENSION.length)}`] : [];
}

/**
 * Whether a file name designates a module of the folder's public surface.
 *
 * @param name - file name
 * @returns `true` for a `.ts` source other than the barrel, tests and declarations
 */
function isPublicModule(name: string): boolean {
  return (
    name.endsWith(SOURCE_EXTENSION) &&
    name !== INDEX_FILE &&
    EXCLUDED_SUFFIXES.every((suffix) => !name.endsWith(suffix))
  );
}

/**
 * Sorted re-export specifiers a barrel must contain.
 *
 * @param directory - absolute folder path of the barrel
 * @returns `./name` for every sibling module and sub-folder
 */
function expectedSpecifiers(directory: string): string[] {
  return readdirSync(directory, { withFileTypes: true })
    .flatMap((entry) => moduleSpecifier(directory, entry))
    .toSorted((left, right) => left.localeCompare(right, "en"));
}

/**
 * Whether a statement is a plain `export * from "./x"`.
 *
 * @param statement - top-level statement of the barrel
 * @returns `true` for an un-aliased star re-export of a relative module
 */
function isStarReexport(
  statement: TSESTree.ProgramStatement,
): statement is TSESTree.ExportAllDeclaration {
  return (
    statement.type === AST_NODE_TYPES.ExportAllDeclaration &&
    statement.exported === null &&
    statement.source.value.startsWith("./")
  );
}

/**
 * `local/barrel-exports`: an `index.ts` only re-exports, and re-exports every sibling, sorted.
 *
 * Autofix regenerates the barrel from the folder content.
 *
 * @see ADR-0012
 */
export const BARREL_EXPORTS_RULE = createRule<[], MessageIds>({
  create: (context) => ({
    Program: (node): void => {
      const strayStatement = node.body.find((statement) => !isStarReexport(statement));

      if (strayStatement !== undefined) {
        context.report({ messageId: "onlyReexports", node: strayStatement });

        return;
      }

      const actual = node.body.filter(isStarReexport).map((statement) => statement.source.value);
      const expected = expectedSpecifiers(dirname(context.filename));

      if (actual.join("\n") === expected.join("\n")) {
        return;
      }

      const text = expected.map((specifier) => `export * from "${specifier}";\n`).join("");

      context.report({
        data: { expected: expected.join(", ") },
        fix: (fixer) => fixer.replaceTextRange([0, context.sourceCode.text.length], text),
        messageId: "outOfSync",
        node,
      });
    },
  }),
  defaultOptions: [],
  meta: {
    docs: { description: "Require barrels to re-export exactly every sibling module, sorted." },
    fixable: "code",
    messages: {
      onlyReexports: 'A barrel only contains `export * from "./module";` statements.',
      outOfSync: "Barrel out of sync with its folder; expected: {{expected}}.",
    },
    schema: [],
    type: "problem",
  },
  name: "barrel-exports",
});

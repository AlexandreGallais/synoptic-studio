import { readdirSync } from "node:fs";
import { basename, dirname, relative } from "node:path";

import { INDEX_FILE, SOURCE_EXTENSION, createRule } from "../utils";

import type { Dirent } from "node:fs";

/** Options of `local/folder-layout`. */
type Options = [
  { readonly listed: readonly string[]; readonly root?: string; readonly signal: number },
];

/** Extension of test files, which a module count leaves out. */
const TEST_EXTENSION = ".test.ts";

/**
 * Whether a file name is a module: TypeScript, neither a test nor the barrel.
 *
 * @param name - file name
 * @returns `true` for a module
 */
function isModule(name: string): boolean {
  return name.endsWith(SOURCE_EXTENSION) && !name.endsWith(TEST_EXTENSION) && name !== INDEX_FILE;
}

/**
 * Whether a file sits beside domain folders: any file but the barrel, in a folder of folders.
 *
 * @param name - file name
 * @param entries - entries of its folder
 * @returns `true` when the file should move into one of the folders
 */
function isMixed(name: string, entries: readonly Dirent[]): boolean {
  return name !== INDEX_FILE && entries.some((entry) => entry.isDirectory());
}

/**
 * `local/folder-layout`: a code folder holds files or domain folders, never both, and asks for
 * a domain to split it past the signal unless it is listed.
 *
 * @see ADR-0043
 */
export const FOLDER_LAYOUT_RULE = createRule<Options, "mixed" | "past">({
  create: (context, [{ listed, root = context.cwd, signal }]) => ({
    Program: (node): void => {
      const directory = dirname(context.filename);
      const name = basename(context.filename);
      const path = relative(root, directory).replaceAll("\\", "/");
      const entries = readdirSync(directory, { withFileTypes: true });
      const loc = { column: 0, line: 1 };

      if (isMixed(name, entries)) {
        context.report({ data: { name, path }, loc, messageId: "mixed", node });
      }

      const modules = entries.filter((entry) => entry.isFile() && isModule(entry.name)).length;

      if (isModule(name) && modules > signal && !listed.includes(path)) {
        context.report({ data: { modules, path, signal }, loc, messageId: "past", node });
      }
    },
  }),
  defaultOptions: [{ listed: [], signal: 15 }],
  meta: {
    docs: { description: "Keep files and domain folders apart, and folders under the signal." },
    messages: {
      mixed: "Folder `{{path}}` holds domain folders: move `{{name}}` into one of them.",
      past: "Folder `{{path}}` holds {{modules}} modules, past {{signal}}: split it by domain, or list it with its reason in docs/conventions/files.md.",
    },
    schema: [
      {
        additionalProperties: false,
        properties: {
          listed: { items: { type: "string" }, type: "array" },
          root: { type: "string" },
          signal: { type: "number" },
        },
        required: ["listed", "signal"],
        type: "object",
      },
    ],
    type: "suggestion",
  },
  name: "folder-layout",
});

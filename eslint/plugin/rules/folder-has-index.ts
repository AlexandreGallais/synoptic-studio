import { dirname, relative } from "node:path";

import { createRule, hasIndex } from "../utils";

/** Options of `local/folder-has-index`. */
type Options = [{ readonly exemptDirectories: readonly string[]; readonly root?: string }];

/**
 * `local/folder-has-index`: every folder holding source files has an `index.ts`.
 *
 * @see ADR-0012
 */
export const FOLDER_HAS_INDEX_RULE = createRule<Options, "missing">({
  create: (context, [{ exemptDirectories, root = context.cwd }]) => ({
    Program: (node): void => {
      const directory = dirname(context.filename);
      const relativePath = relative(root, directory).replaceAll("\\", "/");
      const path = relativePath === "" ? "." : relativePath;

      if (!exemptDirectories.includes(path) && !hasIndex(directory)) {
        context.report({ data: { path }, loc: { column: 0, line: 1 }, messageId: "missing", node });
      }
    },
  }),
  defaultOptions: [{ exemptDirectories: ["."] }],
  meta: {
    docs: { description: "Require an `index.ts` barrel in every source folder." },
    messages: { missing: "Folder `{{path}}` has no `index.ts`: create its barrel." },
    schema: [
      {
        additionalProperties: false,
        properties: {
          exemptDirectories: { items: { type: "string" }, type: "array" },
          root: { type: "string" },
        },
        required: ["exemptDirectories"],
        type: "object",
      },
    ],
    type: "problem",
  },
  name: "folder-has-index",
});

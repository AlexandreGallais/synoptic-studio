import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Folders whose files Node.js runs without a build (the `node-run` scope of `tools/lint/src/scopes.rs`). */
const NODE_RUN_FOLDERS = ["scripts", join(".claude", "hooks")];

/** A relative import's specifier, static (`from "./x"`) or dynamic (`import("./x")`). */
const RELATIVE_SPECIFIER = /(?:from |import\()"(?<specifier>\.\.?\/[^"]+)"/gu;

/**
 * The files Node.js starts or imports in a folder: its `.ts` files, its tests left out (Vitest runs them).
 *
 * @param folder - the folder, relative to the repository's root
 * @returns the files' paths, relative to the root
 */
function nodeRunFiles(folder: string): string[] {
  return readdirSync(join(import.meta.dirname, "..", folder))
    .filter((name) => name.endsWith(".ts") && !name.endsWith(".test.ts"))
    .map((name) => join(folder, name));
}

/**
 * The relative imports of a file that do not name a `.ts` file.
 *
 * @param text - the file's text
 * @returns the specifiers without the `.ts` extension
 */
function specifiersWithoutExtension(text: string): string[] {
  return text
    .matchAll(RELATIVE_SPECIFIER)
    .map((match) => match.groups?.["specifier"] ?? "")
    .filter((specifier) => !specifier.endsWith(".ts"))
    .toArray();
}

describe("the files Node.js runs", () => {
  it("finds a relative import without its extension, static or dynamic", () => {
    expect(
      specifiersWithoutExtension(
        'import { a } from "./a";\nimport { b } from "./b.ts";\n} from "../c";\nawait import("./d");',
      ),
    ).toStrictEqual(["./a", "../c", "./d"]);
  });

  // Node.js resolves only a file's full name; Vitest resolves without it and hid a missing
  // extension in story:finish's imports (AUD-020, N1).
  it.each(NODE_RUN_FOLDERS.flatMap((folder) => nodeRunFiles(folder)))(
    "%s names the .ts file of each relative import",
    (file) => {
      const text = readFileSync(join(import.meta.dirname, "..", file), "utf8");

      expect(specifiersWithoutExtension(text)).toStrictEqual([]);
    },
  );
});

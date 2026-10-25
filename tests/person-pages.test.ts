import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/** The pages a person reads that name the commands to run. */
const COMMAND_PAGES = ["README.md", "docs/index.md"];

/**
 * A file of the repository.
 *
 * @param path - the file, from the root
 * @returns its text
 */
function read(path: string): string {
  return readFileSync(join(ROOT, path), "utf8");
}

/**
 * A field of a parsed JSON value.
 *
 * @param value - the parsed value
 * @param key - the field's name
 * @returns its value, or an empty object when the value has no such field
 */
function field(value: unknown, key: string): unknown {
  const entries = typeof value === "object" && value !== null ? Object.entries(value) : [];

  return entries.find(([name]) => name === key)?.[1] ?? {};
}

/**
 * The names of the scripts `package.json` holds.
 *
 * @returns each script's name
 */
function scriptNames(): readonly string[] {
  const scripts = field(JSON.parse(read("package.json")), "scripts");

  return typeof scripts === "object" && scripts !== null ? Object.keys(scripts) : [];
}

describe("the pages a person reads, kept true to the repository", () => {
  const scripts = scriptNames();

  it.each(COMMAND_PAGES)("names in %s only the npm scripts package.json holds", (page) => {
    const named = read(page)
      .matchAll(/npm run (?<script>[\w:-]+)/gu)
      .map((match) => match.groups?.["script"] ?? "")
      .toArray();

    // The two commands a person types: the playground and this site.
    expect(named.length).toBeGreaterThan(1);
    expect(named.filter((script) => !scripts.includes(script))).toEqual([]);
  });
});

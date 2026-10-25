import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/** The pages a person reads that name the commands to run. */
const COMMAND_PAGES = ["README.md", "docs/commands.md"];

/** An epic as the tables write it: `E01 — Draw symbol shapes from numbers`. */
const EPIC = /\| (?:\d+ \| )?(?<epic>E\d+ — [^|]+?) \|(?: (?<status>[a-z-]+) \|)?/gu;

/** An epic and its status, as a table row gives them. */
type Epic = { readonly epic: string; readonly status: string };

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
 * The epics of the table rows of a text, each with its status cell when the row has one.
 *
 * @param text - Markdown holding tables
 * @returns each epic in the order written
 */
function epicsOf(text: string): readonly Epic[] {
  return text
    .matchAll(EPIC)
    .map((match) => ({
      epic: match.groups?.["epic"] ?? "",
      status: match.groups?.["status"] ?? "",
    }))
    .toArray();
}

/**
 * The part of a page under a heading, up to the next heading of its level.
 *
 * @param text - the page
 * @param heading - the heading, as written, `## What was done`
 * @returns the lines under it
 */
function section(text: string, heading: string): string {
  const after = text.split(`\n${heading}\n`, 2)[1] ?? "";

  return after.split("\n## ", 1)[0] ?? "";
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

    expect(named.length).toBeGreaterThan(3);
    expect(named.filter((script) => !scripts.includes(script))).toEqual([]);
  });

  it("lists as done, in the order of the epics, every epic the backlog says done", () => {
    const backlog = epicsOf(read("docs/backlog/README.md"));
    const page = read("docs/where-it-stands.md");

    expect(epicsOf(section(page, "## What was done")).map((row) => row.epic)).toEqual(
      backlog.filter((row) => row.status === "done").map((row) => row.epic),
    );

    expect(epicsOf(section(page, "## Under way")).map((row) => row.epic)).toEqual(
      backlog.filter((row) => row.status === "in-progress").map((row) => row.epic),
    );
  });
});

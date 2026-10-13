import { readFileSync, readdirSync } from "node:fs";
import { join, relative } from "node:path";

import { describe, expect, it } from "vitest";

/** Repository root. */
const ROOT = join(import.meta.dirname, "..");

/** Most modules a folder holds before it asks for a theme to split it (ADR-0042). */
const SIGNAL = 15;

/** Folders of code the layout rule covers. */
const CODE_FOLDERS = ["src", "playground"];

/**
 * Every folder below a root, the root included.
 *
 * @param folder - absolute path
 * @returns absolute paths of the folder and of each folder below it
 */
function foldersBelow(folder: string): readonly string[] {
  return [
    folder,
    ...readdirSync(folder, { recursive: true, withFileTypes: true })
      .filter((entry) => entry.isDirectory())
      .map((entry) => join(entry.parentPath, entry.name)),
  ];
}

/**
 * Number of modules of a folder: its TypeScript files, tests and barrel left out.
 *
 * @param folder - absolute path
 * @returns how many modules sit directly in it
 */
function moduleCount(folder: string): number {
  return readdirSync(folder).filter(
    (name) => name.endsWith(".ts") && !name.endsWith(".test.ts") && name !== "index.ts",
  ).length;
}

/** Folders listed in `docs/conventions/files.md` as staying whole past the signal, with their count. */
const LISTED = new Map(
  readFileSync(join(ROOT, "docs", "conventions", "files.md"), "utf8")
    .split("### Folders past 15 modules", 2)[1]
    ?.matchAll(/^\| `(?<folder>[^`]+)` +\| (?<count>\d+)/gmu)
    .map((match): [string, number] => [
      match.groups?.["folder"] ?? "",
      Number(match.groups?.["count"]),
    ])
    .toArray(),
);

describe("folders (ADR-0042)", () => {
  const counts = CODE_FOLDERS.flatMap((root) => foldersBelow(join(ROOT, root))).map((folder) => ({
    count: moduleCount(folder),
    folder: relative(ROOT, folder),
  }));

  it("reads the code folders and the list of exceptions", () => {
    expect(counts.length).toBeGreaterThan(10);
    expect(LISTED.size).toBeGreaterThan(0);
  });

  it("splits a folder past 15 modules along a theme, or lists it with its reason", () => {
    expect(
      counts
        .filter(({ count, folder }) => count > SIGNAL && !LISTED.has(folder))
        .map(({ count, folder }) => `${folder} (${String(count)})`),
    ).toEqual([]);
  });

  it("lists only folders still past 15 modules, each with its true count", () => {
    const past = new Map(
      counts.filter(({ count }) => count > SIGNAL).map(({ count, folder }) => [folder, count]),
    );

    expect([...LISTED].filter(([folder, count]) => past.get(folder) !== count)).toEqual([]);
  });
});

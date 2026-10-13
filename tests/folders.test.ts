import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

import { FOLDERS_PAST_SIGNAL, FOLDER_SIGNAL } from "../eslint";

/** Repository root. */
const ROOT = join(import.meta.dirname, "..");

/**
 * Number of modules of a folder: its TypeScript files, tests and barrel left out.
 *
 * @param folder - path from the repository root
 * @returns how many modules sit directly in it
 */
function moduleCount(folder: string): number {
  return readdirSync(join(ROOT, folder)).filter(
    (name) => name.endsWith(".ts") && !name.endsWith(".test.ts") && name !== "index.ts",
  ).length;
}

/** Folders listed in `docs/conventions/files.md` as staying whole past the signal, with their count. */
const LISTED = new Map(
  readFileSync(join(ROOT, "docs", "conventions", "files.md"), "utf8")
    .split(`### Folders past ${String(FOLDER_SIGNAL)} modules`, 2)[1]
    ?.matchAll(/^\| `(?<folder>[^`]+)` +\| (?<count>\d+)/gmu)
    .map((match): [string, number] => [
      match.groups?.["folder"] ?? "",
      Number(match.groups?.["count"]),
    ])
    .toArray(),
);

// `local/folder-layout` checks each folder (ADR-0043); this test keeps its list and the page as one.
describe("folders past the signal (ADR-0043)", () => {
  it("lists in docs/conventions/files.md the folders the lint lets past the signal", () => {
    expect(FOLDERS_PAST_SIGNAL.length).toBeGreaterThan(0);
    expect(new Set(LISTED.keys())).toEqual(new Set(FOLDERS_PAST_SIGNAL));
  });

  it("lists only folders still past the signal, each with its true count", () => {
    expect(
      [...LISTED].filter(
        ([folder, count]) => count <= FOLDER_SIGNAL || moduleCount(folder) !== count,
      ),
    ).toEqual([]);
  });
});

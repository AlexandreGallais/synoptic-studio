import { readFileSync, readdirSync } from "node:fs";
import { join, relative } from "node:path";

import { describe, expect, it } from "vitest";

/** Repository root. */
const ROOT = join(import.meta.dirname, "..");
/** The page of the folders' rules. */
const FILES_PAGE = readFileSync(join(ROOT, "docs", "conventions", "files.md"), "utf8");

/**
 * Most modules a folder holds before it asks for a domain to split it (ADR-0043), read from the
 * page's heading; `local/folder-layout` (tools/lint) keeps the same signal and list, its test
 * checks them against this page.
 */
const FOLDER_SIGNAL = Number(
  /^### Folders past (?<signal>\d+) modules$/mu.exec(FILES_PAGE)?.groups?.["signal"],
);

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

/** Rows of the table of folders past the signal in `docs/conventions/files.md`. */
const ROWS = (
  FILES_PAGE.split(`### Folders past ${String(FOLDER_SIGNAL)} modules`, 2)[1]
    ?.matchAll(/^\| `(?<folder>[^`]+)` +\| (?<count>\d+) +\|(?<reason>[^|]*)\|/gmu)
    .toArray() ?? []
).map((match) => match.groups ?? {});

/** Folders listed in `docs/conventions/files.md` as staying whole past the signal, with their count. */
const LISTED = new Map(
  ROWS.map((row): [string, number] => [row["folder"] ?? "", Number(row["count"])]),
);

/**
 * Folders of `src/` and `playground/` holding more modules than the signal, counted by walking
 * the disk rather than by the lint.
 *
 * @returns paths from the repository root
 */
function foldersPastSignal(): readonly string[] {
  return ["src", "playground"]
    .flatMap((root) => [
      root,
      ...readdirSync(join(ROOT, root), { recursive: true, withFileTypes: true })
        .filter((entry) => entry.isDirectory())
        .map((entry) => relative(ROOT, join(entry.parentPath, entry.name))),
    ])
    .filter((folder) => moduleCount(folder) > FOLDER_SIGNAL);
}

/** Files allowed beside domain folders: the barrel, and the playground's page; hidden files (`.DS_Store`) are skipped. */
const BESIDE_FOLDERS = new Set(["index.ts", "index.html"]);

/**
 * Files of `src/` and `playground/` that sit beside folders, the barrel and the page left out.
 *
 * The lint sees TypeScript files only; this walk covers every other file (`.css`, `.json`…).
 *
 * @returns paths from the repository root
 */
function filesBesideFolders(): readonly string[] {
  const entries = ["src", "playground"].flatMap((root) =>
    readdirSync(join(ROOT, root), { recursive: true, withFileTypes: true }),
  );
  const parents = new Set(
    entries.filter((entry) => entry.isDirectory()).map((entry) => entry.parentPath),
  );

  return entries
    .filter(
      (entry) =>
        entry.isFile() &&
        parents.has(entry.parentPath) &&
        !BESIDE_FOLDERS.has(entry.name) &&
        !entry.name.startsWith("."),
    )
    .map((entry) => relative(ROOT, join(entry.parentPath, entry.name)));
}

// `local/folder-layout` (tools/lint) checks each folder (ADR-0043) and keeps its list equal to the page.
describe("folders past the signal (ADR-0043)", () => {
  it("reads the signal from the page", () => {
    expect(FOLDER_SIGNAL).toBe(15);
  });

  it("[F11.AC4] keeps every file of src and playground out of folders that hold folders", () => {
    expect(filesBesideFolders()).toEqual([]);
  });

  it("[F18.AC18] lists every folder of src and playground past the signal, each with its reason", () => {
    expect(foldersPastSignal().filter((folder) => !LISTED.has(folder))).toEqual([]);
    expect(ROWS.filter((row) => (row["reason"] ?? "").trim() === "")).toEqual([]);
  });

  it("lists only folders still past the signal, each with its true count", () => {
    expect(
      [...LISTED].filter(
        ([folder, count]) => count <= FOLDER_SIGNAL || moduleCount(folder) !== count,
      ),
    ).toEqual([]);
  });
});

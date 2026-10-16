import { readFileSync, readdirSync } from "node:fs";
import { join, relative } from "node:path";

import { ESLint } from "eslint";
import { describe, expect, it } from "vitest";

import { FOLDERS_PAST_SIGNAL, FOLDER_SIGNAL, PROJECT_ROOT } from "../eslint";

/** Repository root. */
const ROOT = join(import.meta.dirname, "..");
/** The project's ESLint, read from its configuration. */
const LINT = new ESLint({ cwd: ROOT });

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
  readFileSync(join(ROOT, "docs", "conventions", "files.md"), "utf8")
    .split(`### Folders past ${String(FOLDER_SIGNAL)} modules`, 2)[1]
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

// `local/folder-layout` checks each folder (ADR-0043); this test keeps its list and the page as one.
describe("folders past the signal (ADR-0043)", () => {
  it("[F11.AC4] lists in docs/conventions/files.md the folders the lint lets past the signal", () => {
    expect(FOLDERS_PAST_SIGNAL.length).toBeGreaterThan(0);
    expect(new Set(LISTED.keys())).toEqual(new Set(FOLDERS_PAST_SIGNAL));
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

  it.each([
    "src/math/vector/norm.ts",
    "playground/points/index.ts",
    "playground/points/delete-points.test.ts",
  ])("applies local/folder-layout to %s, with the list and the signal", async (file) => {
    const config: unknown = await LINT.calculateConfigForFile(file);

    expect(config).toMatchObject({
      rules: {
        "local/folder-layout": [
          2,
          { listed: FOLDERS_PAST_SIGNAL, root: PROJECT_ROOT, signal: FOLDER_SIGNAL },
        ],
      },
    });
  });
});

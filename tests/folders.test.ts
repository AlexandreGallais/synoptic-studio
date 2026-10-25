import { readFileSync, readdirSync } from "node:fs";
import { join, relative } from "node:path";

import { describe, expect, it } from "vitest";

import { isPlaygroundProject } from "./playground-project";

/** Repository root. */
const ROOT = join(import.meta.dirname, "..");
/** The lint's settings, which hold the signal and the folders let past it (`tools/lint`). */
const SETTINGS = readFileSync(join(ROOT, "tools", "lint", "src", "settings.rs"), "utf8");

/**
 * Most modules a folder holds before it asks for a domain to split it (ADR-0043), read from
 * `FOLDER_SIGNAL` of the lint's settings, which `local/folder-layout` applies.
 */
const FOLDER_SIGNAL = Number(
  /^pub const FOLDER_SIGNAL: usize = (?<signal>\d+);$/mu.exec(SETTINGS)?.groups?.["signal"],
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

/**
 * The folders of `FOLDERS_PAST_SIGNAL`, each with the comment above it, its reason.
 *
 * @param settings - the text of the lint's settings
 * @returns each folder listed and its reason, in their order
 */
function listedFolders(settings: string): ReadonlyMap<string, string> {
  const block = /FOLDERS_PAST_SIGNAL: \[&str; \d+\] = \[\n(?<entries>[\s\S]*?)\n\];/u.exec(settings)
    ?.groups?.["entries"];

  return new Map(
    (block ?? "")
      .split(/(?<="),\n/u)
      .map((entry): [string, string] => [
        /"(?<folder>[^"]+)"/u.exec(entry)?.groups?.["folder"] ?? "",
        entry
          .split("\n")
          .filter((line) => line.trimStart().startsWith("//"))
          .map((line) => line.trim().replace(/^\/\/ ?/u, ""))
          .join(" "),
      ])
      .filter(([folder]) => folder !== ""),
  );
}

/** Folders the lint lets past the signal, each with its reason. */
const LISTED = listedFolders(SETTINGS);

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
        .filter(
          (entry) =>
            entry.isDirectory() && !isPlaygroundProject(join(entry.parentPath, entry.name)),
        )
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
    readdirSync(join(ROOT, root), { recursive: true, withFileTypes: true }).filter(
      (entry) => !isPlaygroundProject(join(entry.parentPath, entry.name)),
    ),
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

// `local/folder-layout` (tools/lint) checks each folder (ADR-0043) against its settings' list.
describe("folders past the signal (ADR-0043)", () => {
  it("reads the signal and each folder's reason from the lint's settings", () => {
    expect(FOLDER_SIGNAL).toBe(15);

    expect(
      listedFolders(
        'pub const FOLDERS_PAST_SIGNAL: [&str; 2] = [\n    // One theme,\n    // two lines.\n    "a/b",\n    "c",\n];',
      ),
    ).toEqual(
      new Map([
        ["a/b", "One theme, two lines."],
        ["c", ""],
      ]),
    );
  });

  it("[F11.AC4] keeps every file of src and playground out of folders that hold folders", () => {
    expect(filesBesideFolders()).toEqual([]);
  });

  it("[F18.AC18] lists every folder of src and playground past the signal, each with its reason", () => {
    expect(foldersPastSignal().filter((folder) => !LISTED.has(folder))).toEqual([]);
    expect([...LISTED].filter(([, reason]) => reason.trim() === "")).toEqual([]);
  });

  it("lists only folders still past the signal", () => {
    expect(
      LISTED.keys()
        .filter((folder) => moduleCount(folder) <= FOLDER_SIGNAL)
        .toArray(),
    ).toEqual([]);
  });
});

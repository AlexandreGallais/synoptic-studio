import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { ESLint } from "eslint";
import { getFileInfo } from "prettier";
import { describe, expect, it } from "vitest";

/** Repository root. */
const ROOT = join(import.meta.dirname, "..");

/** The project's ESLint, read from its configuration. */
const LINT = new ESLint({ cwd: ROOT });

/** Time limit of a test asking a tool about every tracked file: four times its 3 s alone, and a margin. */
const WALK_TIMEOUT = 20_000;

/** Prettier's ignore files, as its command line reads them. */
const PRETTIER_IGNORE = [join(ROOT, ".gitignore"), join(ROOT, ".prettierignore")];

/**
 * Whether a parsed JSON value is an object.
 *
 * @param value - any parsed value
 * @returns true for an object, `null` left out
 */
function isObject(value: unknown): value is object {
  return typeof value === "object" && value !== null;
}

/**
 * The `scripts` field of a parsed manifest.
 *
 * @param manifest - `package.json`, parsed
 * @returns the field, or an empty object when it is missing
 */
function scriptsField(manifest: unknown): object {
  const field: unknown = isObject(manifest) && "scripts" in manifest ? manifest.scripts : {};

  return isObject(field) ? field : {};
}

/**
 * Scripts of `package.json`, by name.
 *
 * @returns each script's command
 */
function readScripts(): ReadonlyMap<string, string> {
  const parsed: unknown = JSON.parse(readFileSync(join(ROOT, "package.json"), "utf8"));
  const entries = Object.entries(scriptsField(parsed));

  return new Map(entries.map(([name, command]): [string, string] => [name, String(command)]));
}

/** Scripts of `package.json`. */
const SCRIPTS = readScripts();

/** Files tracked by Git. */
const TRACKED = execFileSync("git", ["ls-files"], { cwd: ROOT, encoding: "utf8" })
  .split("\n")
  .filter((path) => path !== "");

/**
 * Targets given to a tool in one command of a script: its arguments that are not options.
 *
 * @param script - the npm script's name
 * @param tool - the command's first word (`prettier`, `eslint`)
 * @returns its paths and globs, unquoted
 */
function targetsOf(script: string, tool: string): readonly string[] {
  const command = (SCRIPTS.get(script) ?? "")
    .split("&&")
    .map((part) => part.trim())
    .find((part) => part.startsWith(`${tool} `));
  const words = (command ?? "").split(" ").slice(1);

  // An option's value follows it: `--cache-location <folder>`.
  return words
    .filter((word, index) => !word.startsWith("-") && words[index - 1] !== "--cache-location")
    .map((word) => word.replaceAll('"', ""));
}

/**
 * Endings a root glob asks for: `.ts` and `.md` for `*.{ts,md}`, any ending for `*`.
 *
 * @param glob - `*` or `*.{ts,md}`
 * @returns the endings, `""` matching every file
 */
function globEndings(glob: string): readonly string[] {
  const list = /^\*\.\{(?<list>[^}]+)\}$/u.exec(glob)?.groups?.["list"] ?? "";

  return list === "" ? [""] : list.split(",").map((extension) => `.${extension}`);
}

/**
 * Whether a root glob covers a tracked file: a root file with one of its endings.
 *
 * @param glob - `*` or `*.{ts,md}`
 * @param path - a tracked file, from the root
 * @returns true when the glob matches the file
 */
function globCovers(glob: string, path: string): boolean {
  return !path.includes("/") && globEndings(glob).some((ending) => path.endsWith(ending));
}

/**
 * Whether a target covers a tracked file: the file, a folder holding it, or a root glob.
 *
 * @param target - a path or a root glob (`*`, `*.{ts,md}`)
 * @param path - a tracked file, from the root
 * @returns true when the tool reads the file through this target
 */
function covers(target: string, path: string): boolean {
  return target.startsWith("*")
    ? globCovers(target, path)
    : path === target || path.startsWith(`${target}/`);
}

/**
 * Tracked files that a tool should read but none of its targets covers.
 *
 * @param targets - the tool's targets
 * @param candidates - the tracked files the tool checks when it is given everything
 * @returns the files left out
 */
function leftOut(targets: readonly string[], candidates: readonly string[]): readonly string[] {
  return candidates.filter((path) => targets.every((target) => !covers(target, path)));
}

describe("check tools given explicit targets (EN-045)", () => {
  it("[F18.AC20] gives Prettier the same targets in format, format:check and fix", () => {
    const targets = targetsOf("format:check", "prettier");

    expect(targets.length).toBeGreaterThan(0);
    expect(targetsOf("format", "prettier")).toEqual(targets);
    expect(targetsOf("fix", "prettier")).toEqual(targets);
  });

  it("[F18.AC20] gives ESLint the same targets in lint, lint:fix and fix", () => {
    const targets = targetsOf("lint", "eslint");

    expect(targets.length).toBeGreaterThan(0);
    expect(targetsOf("lint:fix", "eslint")).toEqual(targets);
    expect(targetsOf("fix", "eslint")).toEqual(targets);
  });

  it(
    "[F18.AC20] leaves no tracked file that ESLint checks out of its targets",
    { timeout: WALK_TIMEOUT },
    async () => {
      const linted = await Promise.all(
        TRACKED.filter((path) => /\.(?:ts|md)$/u.test(path)).map(async (path) =>
          (await LINT.isPathIgnored(join(ROOT, path))) ? "" : path,
        ),
      );

      expect(
        leftOut(
          targetsOf("lint", "eslint"),
          linted.filter((path) => path !== ""),
        ),
      ).toEqual([]);
    },
  );

  it(
    "[F18.AC20] leaves no tracked file that Prettier formats out of its targets",
    { timeout: WALK_TIMEOUT },
    async () => {
      const formatted = await Promise.all(
        TRACKED.map(async (path) => {
          const fileInfo = await getFileInfo(join(ROOT, path), { ignorePath: PRETTIER_IGNORE });

          return fileInfo.ignored || fileInfo.inferredParser === null ? "" : path;
        }),
      );

      expect(
        leftOut(
          targetsOf("format:check", "prettier"),
          formatted.filter((path) => path !== ""),
        ),
      ).toEqual([]);
    },
  );

  it("[F18.AC20] runs secretlint on the tracked files only", () => {
    expect(SCRIPTS.get("lint:secrets")).toMatch(/^git ls-files -z \| xargs -0 secretlint /u);
  });
});

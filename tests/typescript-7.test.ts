import { execFileSync, spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/** The `tsc` that `npm run typecheck` runs; the declarations are Oxc's since EN-091. */
const TSC = join(ROOT, "node_modules", ".bin", "tsc");

/**
 * Runs `tsc` with the given arguments.
 *
 * @param tscArguments - command-line arguments of `tsc`
 * @returns its exit code and standard output
 */
function runTsc(tscArguments: readonly string[]): {
  readonly status: number | null;
  readonly stdout: string;
} {
  const result = spawnSync(TSC, tscArguments, { cwd: ROOT, encoding: "utf8" });

  return { status: result.status, stdout: result.stdout };
}

/** An import of TypeScript's API, which TypeScript 7 lacks until 7.1, under its name or TypeScript 6's alias. */
const TYPESCRIPT_IMPORT = /from\s+["'](?:typescript|@typescript\/typescript6)["']/u;

describe("TypeScript 7", () => {
  it("[F19.AC6] is the tsc of the type check", () => {
    expect(runTsc(["--version"]).stdout).toMatch(/^Version 7\./u);
  });

  it("[F19.AC6] is the only TypeScript, TypeScript 6 and its alias gone from the packages", () => {
    const manifest = readFileSync(join(ROOT, "package.json"), "utf8");
    const lock = readFileSync(join(ROOT, "package-lock.json"), "utf8");

    expect(manifest).toContain('"typescript": "7.0.2"');
    expect(`${manifest}${lock}`).not.toMatch(/@typescript\/(?:typescript6|native)/u);
  });

  it("[F19.AC6] leaves TypeScript 6's API unimported by the project's own code", () => {
    const sources = execFileSync("git", ["ls-files", "*.ts"], { cwd: ROOT, encoding: "utf8" })
      .split("\n")
      .filter((path) => path !== "");

    expect(
      sources.filter((path) => TYPESCRIPT_IMPORT.test(readFileSync(join(ROOT, path), "utf8"))),
    ).toEqual([]);
  });
});

import { execFileSync, spawnSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/** The `tsc` that `npx tsc --noEmit` runs; the declarations are Oxc's since EN-091. */
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

/**
 * A JSON file of the repository, parsed to an object.
 *
 * @param path - the file, from the repository's root
 * @returns its object, empty when it holds another value
 */
function jsonFile(path: string): object {
  const value: unknown = JSON.parse(readFileSync(join(ROOT, path), "utf8"));

  return typeof value === "object" && value !== null ? value : {};
}

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

  it("[F33.AC1] is the editor's: `.vscode/settings.json` names its installed folder, which exists", () => {
    const settings = jsonFile(".vscode/settings.json");
    const folder = String(Reflect.get(settings, "js/ts.tsdk.path"));
    const manifest = join(folder, "package.json");
    const installed = existsSync(join(ROOT, manifest)) ? jsonFile(manifest) : {};

    // The settings of TypeScript 7's extension (`REF-TS7-VSCODE`), recommended beside Oxc's.
    expect(Reflect.get(settings, "js/ts.experimental.useTsgo")).toBe(true);
    expect(folder).toBe("./node_modules/typescript");
    expect(Reflect.get(installed, "name")).toBe("typescript");
    expect(String(Reflect.get(installed, "version"))).toMatch(/^7\./u);

    expect(Reflect.get(jsonFile(".vscode/extensions.json"), "recommendations")).toContain(
      "TypeScriptTeam.native-preview",
    );
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

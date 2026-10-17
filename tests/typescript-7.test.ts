import { execFileSync, spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
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

/** An import of TypeScript 6's API, under its own name or its alias. */
const TYPESCRIPT_IMPORT = /from\s+["'](?:typescript|@typescript\/typescript6)["']/u;

describe("TypeScript 7", () => {
  it("[F19.AC6] is the tsc of the type check", () => {
    expect(runTsc(["--version"]).stdout).toMatch(/^Version 7\./u);
  });

  it("[F19.AC6] reports a planted type error at its place, with exit code 1", () => {
    const folder = mkdtempSync(join(tmpdir(), "en-071-"));

    writeFileSync(
      join(folder, "tsconfig.json"),
      JSON.stringify({
        compilerOptions: { noEmit: true, strict: true, types: [] },
        files: ["planted.ts"],
      }),
    );

    // Line 2, column 9: the name `planted`, given a string where a number is declared (TS2322).
    writeFileSync(
      join(folder, "planted.ts"),
      'export function half(value: number): number {\n  const planted: number = "half";\n  return value / 2 + planted;\n}\n',
    );

    const result = runTsc(["-p", folder]);

    rmSync(folder, { force: true, recursive: true });

    expect(result.status).toBe(1);

    expect(result.stdout).toContain(
      "planted.ts(2,9): error TS2322: Type 'string' is not assignable to type 'number'.",
    );
  });

  it("[F19.AC6] keeps TypeScript 6 only as the API that typescript-eslint loads", () => {
    const manifest = readFileSync(join(ROOT, "package.json"), "utf8");

    expect(manifest).toContain('"typescript": "npm:@typescript/typescript6@6.0.2"');
    expect(manifest).toContain('"@typescript/native": "npm:typescript@7.0.2"');
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

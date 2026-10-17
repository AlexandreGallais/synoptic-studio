import { spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/** The oxlint that `npm run lint`, `npm run fix` and the pre-commit hook run. */
const OXLINT = join(ROOT, "node_modules", ".bin", "oxlint");

/** `package.json`, as text. */
const MANIFEST = readFileSync(join(ROOT, "package.json"), "utf8");

/**
 * The command line of an npm script.
 *
 * @param name - `lint`, `fix` or another key of `scripts`
 * @returns its command, its escaped quotes kept, empty when it is missing
 */
function scriptOf(name: string): string {
  const entry = new RegExp(String.raw`"${name}": "(?<line>(?:[^"\\]|\\.)+)"`, "u");

  return entry.exec(MANIFEST)?.groups?.["line"] ?? "";
}

describe("type errors with the lint (EN-091)", () => {
  it("reports a planted type error as tsc does, with its code and place", () => {
    const folder = mkdtempSync(join(tmpdir(), "en-091-"));

    writeFileSync(
      join(folder, "tsconfig.json"),
      JSON.stringify({ compilerOptions: { noEmit: true, strict: true, types: [] } }),
    );

    // Line 2, column 9: the name `planted`, given a string where a number is declared (TS2322).
    writeFileSync(
      join(folder, "planted.ts"),
      'export function half(value: number): number {\n  const planted: number = "half";\n  return value / 2 + planted;\n}\n',
    );

    const result = spawnSync(OXLINT, ["--type-aware", "--type-check", "planted.ts"], {
      cwd: folder,
      encoding: "utf8",
    });

    rmSync(folder, { force: true, recursive: true });

    expect(result.status).toBe(1);

    expect(result.stdout).toContain(
      "planted.ts:2:9: error typescript(TS2322): Type 'string' is not assignable to type 'number'.",
    );
  });

  it("runs the type check in every oxlint pass: the lint, the fix, and no tsc in check:all", () => {
    for (const name of ["lint", "fix"]) {
      expect(scriptOf(name)).toContain("oxlint --type-check ");
    }

    expect(scriptOf("check")).not.toContain("typecheck");
  });
});

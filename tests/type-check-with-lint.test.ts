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
  it("[F19.AC6] [F22.AC1] reports a planted type error as tsc does, with its code and place", () => {
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

    // The format named: without it oxlint picks one from the environment, GitHub's annotations
    // under `GITHUB_ACTIONS` (`REF-OXLINT-OUTPUT-FORMATS`), this agent format under an agent's
    // variables (`CLAUDECODE`, `AI_AGENT`, tried 2026-10-18), its default boxes elsewhere (EN-116).
    const result = spawnSync(
      OXLINT,
      ["--type-aware", "--type-check", "--format", "agent", "planted.ts"],
      { cwd: folder, encoding: "utf8" },
    );

    rmSync(folder, { force: true, recursive: true });

    expect(result.status).toBe(1);

    expect(result.stdout).toContain(
      "planted.ts:2:9: error typescript(TS2322): Type 'string' is not assignable to type 'number'.",
    );
  });

  it("[F19.AC1] runs the type-aware rules of the repository's configuration: a floating promise", () => {
    const folder = mkdtempSync(join(tmpdir(), "aud-021-"));

    writeFileSync(
      join(folder, "tsconfig.json"),
      JSON.stringify({ compilerOptions: { noEmit: true, strict: true, types: [] } }),
    );

    // Line 2, column 1: a promise neither awaited nor handled, which only its type tells.
    writeFileSync(join(folder, "floating.ts"), "declare function p(): Promise<number>;\np();\n");

    const result = spawnSync(
      OXLINT,
      ["-c", join(ROOT, ".oxlintrc.json"), "--type-check", "--format", "unix", "floating.ts"],
      {
        cwd: folder,
        encoding: "utf8",
        env: {
          ...process.env,
          PATH: `${join(ROOT, "node_modules", ".bin")}:${process.env["PATH"] ?? ""}`,
        },
      },
    );

    rmSync(folder, { force: true, recursive: true });

    expect(result.stdout).toContain(
      "floating.ts:2:1: Promises must be awaited, add await operator. [Error/typescript(no-floating-promises)]",
    );
  });

  it("[F33.AC1] reports a planted type error with the repository's configuration alone, no flag", () => {
    const folder = mkdtempSync(join(tmpdir(), "en-121-"));

    writeFileSync(
      join(folder, "tsconfig.json"),
      JSON.stringify({ compilerOptions: { noEmit: true, strict: true, types: [] } }),
    );

    // Line 2, column 9, as in the first test: `typeCheck` of `.oxlintrc.json` stands for the flag.
    writeFileSync(
      join(folder, "planted.ts"),
      'export function half(value: number): number {\n  const planted: number = "half";\n  return value / 2 + planted;\n}\n',
    );

    const result = spawnSync(
      OXLINT,
      ["-c", join(ROOT, ".oxlintrc.json"), "--format", "agent", "planted.ts"],
      {
        cwd: folder,
        encoding: "utf8",
        env: {
          ...process.env,
          PATH: `${join(ROOT, "node_modules", ".bin")}:${process.env["PATH"] ?? ""}`,
        },
      },
    );

    rmSync(folder, { force: true, recursive: true });

    expect(result.stdout).toContain(
      "planted.ts:2:9: error typescript(TS2322): Type 'string' is not assignable to type 'number'.",
    );
  });

  it("runs the type check in every oxlint pass: the lint, the fix, and no script runs tsc", () => {
    // The check comes from `.oxlintrc.json` since EN-121, `[F33.AC1]` in tests/oxlint-config.test.ts.
    for (const name of ["lint", "fix"]) {
      expect(scriptOf(name)).toMatch(/(?:^|&& )oxlint --/u);
    }

    // `typecheck` went with EN-115: `npx tsc --noEmit` stays for a person who wants tsc's own run.
    expect(MANIFEST).not.toMatch(/\btsc\b/u);
  });
});

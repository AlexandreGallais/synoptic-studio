import { execFileSync } from "node:child_process";
import { chmodSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { afterEach, describe, expect, it } from "vitest";

/** The pre-push hook's line that runs the golden check. */
const GOLDEN_LINE =
  readFileSync(join(import.meta.dirname, "..", ".husky", "pre-push"), "utf8")
    .split("\n")
    .find((line) => line.includes("golden:check")) ?? "";

/** Temporary FOLDERS to remove after each test. */
const FOLDERS: string[] = [];

/**
 * The environment without Git's variables: the pre-push hook sets `GIT_DIR`, which would point the
 * commands of a test run under it at the real repository (EN-050).
 *
 * @param path - the folders searched for programs, the stub first
 * @returns the variables of this process, Git's left out, with that search path
 */
function cleanEnvironment(path: string): NodeJS.ProcessEnv {
  return {
    ...Object.fromEntries(Object.entries(process.env).filter(([name]) => !name.startsWith("GIT_"))),
    PATH: path,
  };
}

/**
 * Runs the hook's golden line in a new repository, `npm` replaced by a stub that prints its arguments.
 *
 * @param hasGolden - whether the repository's Git directory holds a golden file
 * @returns what the line printed
 */
function goldenLine(hasGolden: boolean): string {
  const folder = mkdtempSync(join(tmpdir(), "pre-push-"));
  const bin = join(folder, "bin");
  const environment = cleanEnvironment(`${bin}:${process.env["PATH"] ?? ""}`);

  FOLDERS.push(folder);
  mkdirSync(bin);
  writeFileSync(join(bin, "npm"), '#!/bin/sh\necho "npm $*"\n');
  chmodSync(join(bin, "npm"), 0o755);
  execFileSync("git", ["init", "-q", folder], { env: environment });

  if (hasGolden) {
    mkdirSync(join(folder, ".git", "metrics"));
    writeFileSync(join(folder, ".git", "metrics", "golden.json"), "[]");
  }

  return execFileSync("sh", ["-e", "-c", GOLDEN_LINE], {
    cwd: folder,
    encoding: "utf8",
    env: environment,
  });
}

afterEach(() => {
  for (const folder of FOLDERS.splice(0)) {
    rmSync(folder, { force: true, recursive: true });
  }
});

describe("pre-push", () => {
  it("[F18.AC11] runs the golden check when a golden file is written", () => {
    expect(goldenLine(true)).toBe("npm run golden:check\n");
  });

  it("[F18.AC11] skips the golden check when no golden file is written", () => {
    expect(goldenLine(false)).toBe("");
  });
});

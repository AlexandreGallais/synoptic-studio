import { execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { afterEach, describe, expect, it } from "vitest";

/** The pre-push hook's line that runs the golden check. */
const GOLDEN_LINE =
  readFileSync(join(import.meta.dirname, "..", ".githooks", "pre-push"), "utf8")
    .split("\n")
    .find((line) => line.includes("golden-check")) ?? "";

/** What the hook's golden line runs, through the hooks' `tool` function. */
const GOLDEN_RUN =
  "tool heavy-lock heavy-lock npx vitest run tests/golden-results.test.ts --mode golden-check\n";

/** Temporary FOLDERS to remove after each test. */
const FOLDERS: string[] = [];

/**
 * The environment without Git's variables: the pre-push hook sets `GIT_DIR`, which would point the
 * commands of a test run under it at the real repository (EN-050).
 *
 * @param path - the folders searched for programs
 * @returns the variables of this process, Git's left out, with that search path
 */
function cleanEnvironment(path: string): NodeJS.ProcessEnv {
  return {
    ...Object.fromEntries(Object.entries(process.env).filter(([name]) => !name.startsWith("GIT_"))),
    PATH: path,
  };
}

/**
 * Runs the hook's golden line in a new repository, the hooks' `tool` function replaced by one that
 * prints its arguments.
 *
 * @param hasGolden - whether the repository's Git directory holds a golden file
 * @param isLinked - whether the line runs in a linked worktree, the golden file staying in the main
 * checkout's Git directory, shared by every worktree
 * @returns what the line printed
 */
function goldenLine(hasGolden: boolean, isLinked: boolean): string {
  const folder = mkdtempSync(join(tmpdir(), "pre-push-"));
  const environment = cleanEnvironment(process.env["PATH"] ?? "");

  FOLDERS.push(folder);
  execFileSync("git", ["init", "-q", folder], { env: environment });

  if (hasGolden) {
    mkdirSync(join(folder, ".git", "metrics"));
    writeFileSync(join(folder, ".git", "metrics", "golden.json"), "[]");
  }

  const worktree = join(folder, "linked");

  if (isLinked) {
    const author = [
      "-c",
      "user.name=Test",
      "-c",
      "user.email=test@example.com",
      "-c",
      "commit.gpgsign=false",
    ];

    execFileSync("git", [...author, "commit", "-q", "--allow-empty", "-m", "start"], {
      cwd: folder,
      env: environment,
    });

    execFileSync("git", ["worktree", "add", "-q", worktree], { cwd: folder, env: environment });
  }

  return execFileSync("sh", ["-e", "-c", `tool() { echo "tool $*"; }\n${GOLDEN_LINE}`], {
    cwd: isLinked ? worktree : folder,
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
    expect(goldenLine(true, false)).toBe(GOLDEN_RUN);
  });

  it("[F18.AC11] skips the golden check when no golden file is written", () => {
    expect(goldenLine(false, false)).toBe("");
  });

  it("[F18.AC11] finds the golden file of the main checkout from a linked worktree", () => {
    expect(goldenLine(true, true)).toBe(GOLDEN_RUN);
    expect(goldenLine(false, true)).toBe("");
  });
});

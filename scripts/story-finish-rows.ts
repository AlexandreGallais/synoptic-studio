// The rows of a merged story moved into the logs on its base by `story:finish` (ADR-0065, decision
// 4; EN-050): the Rust tool `move-rows` writes them, then only the files it wrote are committed, and
// the base pushed whenever it holds commits its remote has not (AUD-020).

import { execFileSync } from "node:child_process";

/** The Rust tool moving a story's rows into the logs. */
export const MOVE_ROWS = [
  "run",
  "--release",
  "-q",
  "--manifest-path",
  "tools/shared-files/Cargo.toml",
  "--bin",
  "move-rows",
  "--",
];

/**
 * The message of the commit moving a merged story's rows into the logs.
 *
 * @param branch - the story's branch, `chore/en-050-topic`
 * @returns the commit message, its `Refs:` footer naming the story
 */
export function rowsMessage(branch: string): string {
  const id = (branch.split("/", 2)[1] ?? "").split("-").slice(0, 2).join("-").toUpperCase();

  return `docs(docs): move the rows of ${id} into the logs\n\nRefs: ${id}\n`;
}

/**
 * Output of a command.
 *
 * @param command - the program
 * @param options - its arguments
 * @param cwd - the folder it runs in
 * @returns its trimmed standard output
 */
export function run(command: string, options: readonly string[], cwd: string): string {
  return execFileSync(command, options, { cwd, encoding: "utf8" }).trim();
}

/**
 * Commits the files a merged story's rows were moved into, and only them: whatever else the
 * checkout of the base holds staged stays staged, out of the commit (AUD-020).
 *
 * @param branch - the merged story's branch
 * @param files - the logs and the story file written
 * @param cwd - the checkout of the base
 */
export function commitRows(branch: string, files: readonly string[], cwd: string): void {
  run("git", ["commit", "--quiet", "-m", rowsMessage(branch), "--", ...files], cwd);
}

/**
 * Pushes the base when it holds commits its remote has not: those of this run, or those a run
 * stopped before pushing (AUD-020).
 *
 * @param cwd - the checkout of the base
 * @returns true when it pushed
 */
export function pushAhead(cwd: string): boolean {
  if (run("git", ["rev-list", "--count", "@{upstream}..HEAD"], cwd) === "0") {
    return false;
  }

  execFileSync("git", ["push", "--quiet"], { cwd, stdio: "inherit" });

  return true;
}

/**
 * Moves a merged story's rows into the logs, then commits and pushes them on the base.
 *
 * @param branch - the merged story's branch
 * @param cwd - the checkout of the base
 */
export function moveRows(branch: string, cwd: string): void {
  const moved = run("cargo", [...MOVE_ROWS, branch], cwd);
  const files = moved === "" ? [] : moved.split("\n");

  if (files.length > 0) {
    run("npx", ["prettier", "--write", ...files], cwd);
    commitRows(branch, files, cwd);
  }

  if (pushAhead(cwd)) {
    process.stdout.write(
      `base pushed, rows moved: ${files.length === 0 ? "none new" : files.join(", ")}\n`,
    );
  }
}

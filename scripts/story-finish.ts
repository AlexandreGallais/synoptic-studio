// The end of a story in a run (RET-002, the agent's tools in docs/tooling/agent-tools.md): its
// pull request opened with the `autonomous` label, its merge by GitHub awaited for at most three
// minutes, then the base brought up to date and the story's branch deleted here and on GitHub
// (ADR-0045). Replaces the waiting loops written by hand. Once merged, the rows the story file
// holds for the logs are moved into them by the Rust tool `move-rows`, committed and pushed on the
// base (ADR-0065, decision 4; EN-050). It runs under the merge lock of `tools/heavy-lock/`, so
// stories merge one at a time: the next one waits, then rebases on the updated base before its pull
// request (ADR-0065, decision 5; EN-051). Run by
// `npm run story:finish -- <base> <title> <body file>` once the branch is pushed.

import { execFileSync, spawnSync } from "node:child_process";
import { setTimeout as delay } from "node:timers/promises";

/** Time between two readings of the pull request's state. */
const POLL_MS = 10_000;

/** Readings before giving up: three minutes. */
const POLLS = 18;

/** The Rust tool moving a story's rows into the logs. */
const MOVE_ROWS = [
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
function run(command: string, options: readonly string[], cwd: string): string {
  return execFileSync(command, options, { cwd, encoding: "utf8" }).trim();
}

/**
 * Whether a Git command ends with success.
 *
 * @param options - its arguments
 * @param cwd - the folder it runs in
 * @returns true when it exits with 0
 */
function gitSucceeds(options: readonly string[], cwd: string): boolean {
  return spawnSync("git", options, { cwd, stdio: "ignore" }).status === 0;
}

/**
 * Brings the story's branch onto its base as the remote holds it now, after another story merged.
 *
 * @param base - the base branch, `feature/f18-topic`
 * @param cwd - the story's checkout
 * @returns `rebased` when the base had moved, `up to date` when the branch already holds it
 * @throws {Error} on a conflict: the rebase is aborted and the branch left as it was
 */
export function rebaseOnBase(base: string, cwd: string): "rebased" | "up to date" {
  run("git", ["fetch", "--quiet", "origin", base], cwd);

  if (gitSucceeds(["merge-base", "--is-ancestor", `origin/${base}`, "HEAD"], cwd)) {
    return "up to date";
  }

  if (!gitSucceeds(["rebase", "--quiet", `origin/${base}`], cwd)) {
    run("git", ["rebase", "--abort"], cwd);

    throw new Error(`conflict rebasing on origin/${base}: rebase by hand, then run again`);
  }

  return "rebased";
}

/**
 * Moves a merged story's rows into the logs, then commits and pushes them on the base.
 *
 * @param branch - the merged story's branch
 * @param cwd - the checkout of the base
 */
function moveRows(branch: string, cwd: string): void {
  const moved = run("cargo", [...MOVE_ROWS, branch], cwd);

  if (moved === "") {
    return;
  }

  const files = moved.split("\n");

  run("npx", ["prettier", "--write", ...files], cwd);
  run("git", ["add", "--", ...files], cwd);
  run("git", ["commit", "--quiet", "-m", rowsMessage(branch)], cwd);
  execFileSync("git", ["push", "--quiet"], { cwd, stdio: "inherit" });
  process.stdout.write(`rows moved into the logs and pushed: ${files.join(", ")}\n`);
}

/**
 * What to do after a reading of the pull request's state.
 *
 * @param state - `OPEN`, `MERGED` or `CLOSED`
 * @param readings - readings made so far, this one included
 * @returns `clean` once merged, `wait` while open and time is left, `stop` otherwise
 */
export function nextStep(state: string, readings: number): "clean" | "stop" | "wait" {
  if (state === "MERGED") {
    return "clean";
  }

  return state === "OPEN" && readings < POLLS ? "wait" : "stop";
}

/**
 * The pull request's state, read until it merges, closes, or time runs out.
 *
 * @param url - the pull request
 * @param readings - readings made so far
 * @param cwd - the story's checkout
 * @returns its last state
 */
async function awaitMerge(url: string, readings: number, cwd: string): Promise<string> {
  const state = run("gh", ["pr", "view", url, "--json", "state", "-q", ".state"], cwd);

  if (nextStep(state, readings + 1) !== "wait") {
    return state;
  }

  await delay(POLL_MS);

  return await awaitMerge(url, readings + 1, cwd);
}

if (import.meta.main) {
  const [base = "", title = "", body = ""] = process.argv.slice(2);
  const here = process.cwd();
  const branch = run("git", ["branch", "--show-current"], here);

  if (rebaseOnBase(base, here) === "rebased") {
    execFileSync("git", ["push", "--quiet", "--force-with-lease"], { cwd: here, stdio: "inherit" });
    process.stdout.write(`${branch} rebased on origin/${base} and pushed\n`);
  }

  const url = run(
    "gh",
    [
      "pr",
      "create",
      "--base",
      base,
      "--label",
      "autonomous",
      "--title",
      title,
      "--body-file",
      body,
    ],
    here,
  );

  process.stdout.write(`${url}\n`);

  const state = await awaitMerge(url, 0, here);

  if (state === "MERGED") {
    const repository = run(
      "gh",
      ["repo", "view", "--json", "nameWithOwner", "-q", ".nameWithOwner"],
      here,
    );

    run("git", ["switch", "--quiet", base], here);
    run("git", ["pull", "--quiet", "--prune"], here);
    run("git", ["branch", "-D", branch], here);
    run("gh", ["api", "-X", "DELETE", `repos/${repository}/git/refs/heads/${branch}`], here);
    process.stdout.write(`merged; ${base} up to date, ${branch} deleted\n`);
    moveRows(branch, here);
  } else {
    process.stderr.write(
      `${state} after 3 min: read gh run list --workflow story-automerge.yml --limit 1\n`,
    );

    process.exitCode = 1;
  }
}

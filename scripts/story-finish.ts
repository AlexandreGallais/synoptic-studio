// The end of a story in a run (RET-002, the agent's tools in docs/tooling/agent-tools.md): its
// pull request opened with the `autonomous` label, its merge by GitHub awaited for at most three
// minutes, then the base brought up to date and the story's branch deleted here and on GitHub
// (ADR-0045). Replaces the waiting loops written by hand. Once merged, the rows the story file
// holds for the logs are moved into them by the Rust tool `move-rows`, committed and pushed on the
// base (ADR-0065, decision 4; EN-050). Run by `npm run story:finish -- <base> <title> <body file>`
// once the branch is pushed.

import { execFileSync } from "node:child_process";
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
 * Moves a merged story's rows into the logs, then commits and pushes them on the base.
 *
 * @param branch - the merged story's branch
 */
function moveRows(branch: string): void {
  const moved = run("cargo", [...MOVE_ROWS, branch]);

  if (moved === "") {
    return;
  }

  const files = moved.split("\n");

  run("npx", ["prettier", "--write", ...files]);
  run("git", ["add", "--", ...files]);
  run("git", ["commit", "--quiet", "-m", rowsMessage(branch)]);
  execFileSync("git", ["push", "--quiet"], { stdio: "inherit" });
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
 * Output of a command.
 *
 * @param command - the program
 * @param options - its arguments
 * @returns its trimmed standard output
 */
function run(command: string, options: readonly string[]): string {
  return execFileSync(command, options, { encoding: "utf8" }).trim();
}

/**
 * The pull request's state, read until it merges, closes, or time runs out.
 *
 * @param url - the pull request
 * @param readings - readings made so far
 * @returns its last state
 */
async function awaitMerge(url: string, readings: number): Promise<string> {
  const state = run("gh", ["pr", "view", url, "--json", "state", "-q", ".state"]);

  if (nextStep(state, readings + 1) !== "wait") {
    return state;
  }

  await delay(POLL_MS);

  return await awaitMerge(url, readings + 1);
}

if (import.meta.main) {
  const [base = "", title = "", body = ""] = process.argv.slice(2);
  const branch = run("git", ["branch", "--show-current"]);
  const url = run("gh", [
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
  ]);

  process.stdout.write(`${url}\n`);

  const state = await awaitMerge(url, 0);

  if (state === "MERGED") {
    const repository = run("gh", [
      "repo",
      "view",
      "--json",
      "nameWithOwner",
      "-q",
      ".nameWithOwner",
    ]);

    run("git", ["switch", "--quiet", base]);
    run("git", ["pull", "--quiet", "--prune"]);
    run("git", ["branch", "-D", branch]);
    run("gh", ["api", "-X", "DELETE", `repos/${repository}/git/refs/heads/${branch}`]);
    process.stdout.write(`merged; ${base} up to date, ${branch} deleted\n`);
    moveRows(branch);
  } else {
    process.stderr.write(
      `${state} after 3 min: read gh run list --workflow story-automerge.yml --limit 1\n`,
    );

    process.exitCode = 1;
  }
}

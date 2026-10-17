// The end of a story in a run (RET-002, the agent's tools in docs/tooling/agent-tools.md): its
// pull request opened with the `autonomous` label, its merge by GitHub awaited for at most three
// minutes, then the base brought up to date and the story's branch deleted here and on GitHub
// (ADR-0045). Replaces the waiting loops written by hand. Once merged, the rows the story file
// holds for the logs are moved into them by the Rust tool `move-rows`, committed and pushed on the
// base (ADR-0065, decision 4; EN-050). It runs under the merge lock of `tools/heavy-lock/`, so
// stories merge one at a time: the next one waits, then rebases on the updated base before its pull
// request (ADR-0065, decision 5; EN-051). From a worktree, it updates the base where it is
// checked out, without switching branch, then removes the worktree. Run again after a stop, it
// reuses the story's pull request, and goes straight to the clean-up once merged (AUD-020). Run by
// `npm run story:finish -- <base> <title> <body file>` once the branch is pushed.

import { execFileSync, spawnSync } from "node:child_process";
import { setTimeout as delay } from "node:timers/promises";

import { cleanUp, cleanUpPlan, manualSteps, worktrees } from "./story-finish-plan";
import { moveRows, run } from "./story-finish-rows";

import type { CleanUp } from "./story-finish-plan";

/** Time between two readings of the pull request's state. */
const POLL_MS = 10_000;

/** Readings before giving up: three minutes. */
const POLLS = 18;

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

/** A story's pull request as GitHub holds it, both fields `""` when it has none. */
type PullRequest = {
  /** `OPEN`, `MERGED` or `CLOSED`. */
  readonly state: string;
  /** Its address. */
  readonly url: string;
};

/**
 * Where a run of `story:finish` starts, from the story's pull request: one is opened when there is
 * none or it was closed, an open one is awaited, and a merged one goes straight to the clean-up, so
 * a run stopped midway is run again as it is (AUD-020).
 *
 * @param state - the pull request's state, `""` when there is none
 * @returns `create`, `await` or `clean`
 */
export function startStep(state: string): "await" | "clean" | "create" {
  if (state === "MERGED") {
    return "clean";
  }

  return state === "OPEN" ? "await" : "create";
}

/**
 * The pull request of a branch, read from GitHub.
 *
 * @param branch - the story's branch
 * @param cwd - the story's checkout
 * @returns its state and address, both `""` when it has none
 */
function pullRequestOf(branch: string, cwd: string): PullRequest {
  const result = spawnSync(
    "gh",
    ["pr", "view", branch, "--json", "state,url", "-q", '.state + " " + .url'],
    {
      cwd,
      encoding: "utf8",
    },
  );
  const [state = "", url = ""] = result.status === 0 ? result.stdout.trim().split(" ") : [];

  return { state, url };
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

/**
 * Cleans up a merged story, or says why the clean-up stopped midway.
 *
 * @param state - the pull request's state
 * @param plan - the clean-up's steps
 * @param branch - the story's branch
 * @returns true once cleaned up, false when not merged, the text of the error that stopped it
 */
function cleanUpOrStop(state: string, plan: CleanUp, branch: string): boolean | string {
  try {
    return cleanUp(state, plan, (cwd) => {
      moveRows(branch, cwd);
    });
  } catch (error) {
    return String(error);
  }
}

if (import.meta.main) {
  const [base = "", title = "", body = ""] = process.argv.slice(2);
  const here = run("git", ["rev-parse", "--show-toplevel"], process.cwd());
  const branch = run("git", ["branch", "--show-current"], here);

  const existing = pullRequestOf(branch, here);
  const start = startStep(existing.state);

  if (start !== "clean" && rebaseOnBase(base, here) === "rebased") {
    execFileSync("git", ["push", "--quiet", "--force-with-lease"], { cwd: here, stdio: "inherit" });
    process.stdout.write(`${branch} rebased on origin/${base} and pushed\n`);
  }

  const url =
    start === "create"
      ? run(
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
        )
      : existing.url;

  process.stdout.write(`${url}\n`);

  const state = start === "clean" ? "MERGED" : await awaitMerge(url, 0, here);
  const repository = run(
    "gh",
    ["repo", "view", "--json", "nameWithOwner", "-q", ".nameWithOwner"],
    here,
  );
  const plan = cleanUpPlan(
    { base, branch, repository },
    here,
    worktrees(run("git", ["worktree", "list", "--porcelain"], here)),
  );

  const outcome = cleanUpOrStop(state, plan, branch);

  if (outcome === true) {
    process.stdout.write(`merged; ${base} up to date, ${branch} deleted here and on GitHub\n`);
  } else if (outcome === false) {
    process.stderr.write(
      `${state} after 3 min, ${branch} kept: read gh run list --workflow story-automerge.yml --limit 1, then run again: it reuses the pull request\n`,
    );

    process.exitCode = 1;
  } else {
    process.stderr.write(
      `the clean-up stopped: ${outcome}\nRun again from the worktree while it exists, or finish by hand, skipping the steps already done:\n${manualSteps(plan, branch)}\n`,
    );

    process.exitCode = 1;
  }
}

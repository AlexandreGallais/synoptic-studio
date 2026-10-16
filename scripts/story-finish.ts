// The end of a story in a run (RET-002, the agent's tools in docs/tooling/agent-tools.md): its
// pull request opened with the `autonomous` label, its merge by GitHub awaited for at most three
// minutes, then the base brought up to date and the story's branch deleted here and on GitHub
// (ADR-0045). Replaces the waiting loops written by hand. Once merged, the rows the story file
// holds for the logs are moved into them by the Rust tool `move-rows`, committed and pushed on the
// base (ADR-0065, decision 4; EN-050). It runs under the merge lock of `tools/heavy-lock/`, so
// stories merge one at a time: the next one waits, then rebases on the updated base before its pull
// request (ADR-0065, decision 5; EN-051). From a worktree, it updates the base where it is
// checked out, without switching branch, then removes the worktree. Run by
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

/** A checkout of the repository: the main one or a linked worktree. */
export type Worktree = {
  /** The branch checked out, `""` when detached. */
  readonly branch: string;
  /** Its folder. */
  readonly path: string;
};

/**
 * The value of a field in a block of `git worktree list --porcelain`.
 *
 * @param lines - the block's lines
 * @param key - the field, `worktree` or `branch`
 * @returns its value, `""` when absent
 */
function fieldOf(lines: readonly string[], key: string): string {
  return lines.find((line) => line.startsWith(`${key} `))?.slice(key.length + 1) ?? "";
}

/**
 * The checkouts of the repository, the main one first.
 *
 * @param porcelain - the output of `git worktree list --porcelain`
 * @returns each checkout's folder and branch
 */
export function worktrees(porcelain: string): readonly Worktree[] {
  return porcelain
    .split(/\n\n+/u)
    .filter((block) => block.startsWith("worktree "))
    .map((block) => {
      const lines = block.split("\n");

      return {
        branch: fieldOf(lines, "branch").replace(/^refs\/heads\//u, ""),
        path: fieldOf(lines, "worktree"),
      };
    });
}

/** A Git command and the folder it runs in. */
type Step = {
  /** The folder. */
  readonly cwd: string;
  /** Git's arguments. */
  readonly options: readonly string[];
};

/** The clean-up of a merged story. */
export type CleanUp = {
  /** The checkout of the base, where its rows are moved. */
  readonly place: string;
  /** The base brought up to its remote. */
  readonly update: readonly Step[];
  /** The story's worktree and branch removed. */
  readonly remove: readonly Step[];
};

/** A story and the branch it merges into. */
export type Story = {
  /** The base branch, `feature/f18-topic`. */
  readonly base: string;
  /** The story's branch. */
  readonly branch: string;
};

/**
 * Where the base is brought up to date: from a worktree, the checkout holding the base if any; the
 * checkout `story:finish` runs in otherwise, which then switches to it.
 *
 * @param base - the base branch
 * @param here - the checkout `story:finish` runs in
 * @param checkouts - the repository's checkouts, the main one first
 * @returns the folder of that checkout
 */
function baseCheckout(base: string, here: string, checkouts: readonly Worktree[]): string {
  const isLinked = here !== checkouts[0]?.path;
  const holder = isLinked ? checkouts.find((checkout) => checkout.branch === base) : undefined;

  return holder?.path ?? here;
}

/**
 * The removal of a linked worktree, from the main checkout.
 *
 * @param worktree - the worktree's folder
 * @param main - the main checkout
 * @returns the step removing it
 */
function worktreeRemovalStep(worktree: string, main: string): Step {
  return { cwd: main, options: ["worktree", "remove", worktree] };
}

/**
 * The clean-up of a merged story, from the checkout it was finished in. From a worktree, the base is
 * brought up to date where it is checked out — the main checkout in a run — without switching any
 * branch, then the worktree is removed; elsewhere the checkout switches to the base first.
 *
 * @param story - the story's branch and its base
 * @param here - the checkout `story:finish` runs in
 * @param checkouts - the repository's checkouts, the main one first
 * @returns the steps to bring the base up to date, then to remove the story
 */
export function cleanUpPlan(story: Story, here: string, checkouts: readonly Worktree[]): CleanUp {
  const main = checkouts.at(0)?.path ?? here;
  const place = baseCheckout(story.base, here, checkouts);
  const fetch = { cwd: here, options: ["fetch", "--quiet", "--prune", "origin"] };
  const toBase = place === here ? [{ cwd: here, options: ["switch", "--quiet", story.base] }] : [];
  const worktreeRemoval = here === main ? [] : [worktreeRemovalStep(here, main)];

  return {
    place,
    remove: [...worktreeRemoval, { cwd: main, options: ["branch", "-D", story.branch] }],
    update: [
      fetch,
      ...toBase,
      { cwd: place, options: ["merge", "--ff-only", "--quiet", `origin/${story.base}`] },
    ],
  };
}

/**
 * Cleans up a story once its pull request merged: the base brought up to date, the rows moved there,
 * the worktree and the branch removed. A pull request not merged keeps everything.
 *
 * @param state - the pull request's state: `OPEN`, `MERGED` or `CLOSED`
 * @param plan - the clean-up's steps
 * @param moveRowsIn - moves the story's rows into the logs in a checkout of the base
 * @returns true when it cleaned up
 */
export function cleanUp(state: string, plan: CleanUp, moveRowsIn: (cwd: string) => void): boolean {
  if (state !== "MERGED") {
    return false;
  }

  for (const step of plan.update) {
    run("git", step.options, step.cwd);
  }

  moveRowsIn(plan.place);

  for (const step of plan.remove) {
    run("git", step.options, step.cwd);
  }

  return true;
}

if (import.meta.main) {
  const [base = "", title = "", body = ""] = process.argv.slice(2);
  const here = run("git", ["rev-parse", "--show-toplevel"], process.cwd());
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
  const repository = run(
    "gh",
    ["repo", "view", "--json", "nameWithOwner", "-q", ".nameWithOwner"],
    here,
  );
  const plan = cleanUpPlan(
    { base, branch },
    here,
    worktrees(run("git", ["worktree", "list", "--porcelain"], here)),
  );

  if (
    cleanUp(state, plan, (cwd) => {
      moveRows(branch, cwd);
    })
  ) {
    run("gh", ["api", "-X", "DELETE", `repos/${repository}/git/refs/heads/${branch}`], plan.place);
    process.stdout.write(`merged; ${base} up to date in ${plan.place}, ${branch} deleted\n`);
  } else {
    process.stderr.write(
      `${state} after 3 min, ${branch} kept: read gh run list --workflow story-automerge.yml --limit 1\n`,
    );

    process.exitCode = 1;
  }
}

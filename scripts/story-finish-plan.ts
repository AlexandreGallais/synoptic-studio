// The clean-up of a story merged by `story:finish` (ADR-0045, EN-051): the base brought up to date
// where it is checked out, the rows moved there, the worktree and the branch removed here and on
// GitHub; and the same steps as commands to type when it stopped midway (AUD-020).

import { MOVE_ROWS, rowsMessage, run } from "./story-finish-rows.ts";

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

/** A command and the folder it runs in. */
type Step = {
  /** The program: Git, or GitHub's command line for the branch on GitHub. */
  readonly command: "gh" | "git";
  /** The folder. */
  readonly cwd: string;
  /** Its arguments. */
  readonly options: readonly string[];
};

/** The clean-up of a merged story. */
export type CleanUp = {
  /** The checkout of the base, where its rows are moved. */
  readonly place: string;
  /** The base brought up to its remote. */
  readonly update: readonly Step[];
  /** The story's worktree and branch removed, here then on GitHub, from the main checkout. */
  readonly remove: readonly Step[];
};

/** A story and the branch it merges into. */
export type Story = {
  /** The base branch, `feature/f18-topic`. */
  readonly base: string;
  /** The story's branch. */
  readonly branch: string;
  /** The repository on GitHub, `owner/name`. */
  readonly repository: string;
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
  return { command: "git", cwd: main, options: ["worktree", "remove", worktree] };
}

/**
 * The clean-up of a merged story, from the checkout it was finished in. From a worktree, the base is
 * brought up to date where it is checked out — the main checkout in a run — without switching any
 * branch, then the worktree is removed; elsewhere the checkout switches to the base first. Every
 * step after the worktree's removal runs from the main checkout: the worktree may be the checkout of
 * the base, and a command started in a removed folder fails (EN-051, PR #272).
 *
 * @param story - the story's branch and its base
 * @param here - the checkout `story:finish` runs in
 * @param checkouts - the repository's checkouts, the main one first
 * @returns the steps to bring the base up to date, then to remove the story
 */
export function cleanUpPlan(story: Story, here: string, checkouts: readonly Worktree[]): CleanUp {
  const main = checkouts.at(0)?.path ?? here;
  const place = baseCheckout(story.base, here, checkouts);
  const fetch: Step = {
    command: "git",
    cwd: here,
    options: ["fetch", "--quiet", "--prune", "origin"],
  };
  const toBase: Step[] =
    place === here
      ? [{ command: "git", cwd: here, options: ["switch", "--quiet", story.base] }]
      : [];
  const worktreeRemoval = here === main ? [] : [worktreeRemovalStep(here, main)];
  const remoteReference = `repos/${story.repository}/git/refs/heads/${story.branch}`;

  return {
    place,
    remove: [
      ...worktreeRemoval,
      { command: "git", cwd: main, options: ["branch", "-D", story.branch] },
      // Through GitHub's API rather than `git push --delete`, which would run the pre-push checks.
      { command: "gh", cwd: main, options: ["api", "-X", "DELETE", remoteReference] },
    ],
    update: [
      fetch,
      ...toBase,
      {
        command: "git",
        cwd: place,
        options: ["merge", "--ff-only", "--quiet", `origin/${story.base}`],
      },
    ],
  };
}

/**
 * Cleans up a story once its pull request merged: the base brought up to date, the rows moved there,
 * the worktree and the branch removed here and on GitHub. A pull request not merged keeps everything.
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
    run(step.command, step.options, step.cwd);
  }

  moveRowsIn(plan.place);

  for (const step of plan.remove) {
    run(step.command, step.options, step.cwd);
  }

  return true;
}

/**
 * A step written as a command to type.
 *
 * @param step - the command, its arguments and its folder
 * @returns `cd "<folder>" && <command> <arguments>`
 */
function typed(step: Step): string {
  return `cd "${step.cwd}" && ${[step.command, ...step.options].join(" ")}`;
}

/**
 * The clean-up's commands to type when it stopped midway: the base brought up to date, the rows
 * moved, committed and pushed, then the worktree and the branch removed (AUD-020).
 *
 * @param plan - the clean-up's steps
 * @param branch - the merged story's branch
 * @returns one command per line
 */
export function manualSteps(plan: CleanUp, branch: string): string {
  const paragraphs = rowsMessage(branch)
    .split("\n")
    .filter((line) => line !== "")
    .map((line) => `-m "${line}"`);
  const rows = [
    `cd "${plan.place}" && cargo ${[...MOVE_ROWS, branch].join(" ")}`,
    `cd "${plan.place}" && git commit ${paragraphs.join(" ")} -- <the files it printed>`,
    `cd "${plan.place}" && git push`,
  ];

  return [...plan.update, ...plan.remove]
    .map((step) => typed(step))
    .toSpliced(plan.update.length, 0, ...rows)
    .join("\n");
}

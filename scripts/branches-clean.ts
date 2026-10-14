// Clean-up of the local clone after a feature (ADR-0038): local branches already merged into
// `develop` or whose branch on GitHub was deleted are removed, stale remote branches pruned, and
// what is left reported (other branches on GitHub, stashes, worktrees). Run by
// `npm run branches:clean` after each merged feature pull request and each release.

import { execFileSync } from "node:child_process";

/** Branches never deleted. */
const KEPT = new Set(["develop", "main"]);

/** A local branch as Git reports it. */
export type LocalBranch = {
  /** Its branch on GitHub was deleted (`[gone]`). */
  readonly isGone: boolean;
  /** Its commits are all in `origin/develop`. */
  readonly isMerged: boolean;
  readonly name: string;
};

/**
 * Local branches to delete: merged or gone, never `main`, `develop` nor the one checked out.
 *
 * @param branches - local branches
 * @param current - branch checked out
 * @returns names to delete
 */
export function staleBranches(
  branches: readonly LocalBranch[],
  current: string,
): readonly string[] {
  return branches
    .filter((branch) => !KEPT.has(branch.name) && branch.name !== current)
    .filter((branch) => branch.isGone || branch.isMerged)
    .map((branch) => branch.name);
}

/**
 * Lines of a Git command's output.
 *
 * @param options - what follows `git`
 * @returns its non-empty lines
 */
function git(...options: readonly string[]): readonly string[] {
  return execFileSync("git", options, { encoding: "utf8" })
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line !== "");
}

if (import.meta.main) {
  git("fetch", "--prune", "origin");
  git("worktree", "prune");

  const merged = new Set(git("branch", "--format=%(refname:short)", "--merged", "origin/develop"));
  const branches = git(
    "for-each-ref",
    "--format=%(refname:short)|%(upstream:track)",
    "refs/heads",
  ).map((line): LocalBranch => {
    const [name = "", track = ""] = line.split("|", 2);

    return { isGone: track === "[gone]", isMerged: merged.has(name), name };
  });
  const stale = staleBranches(branches, git("branch", "--show-current")[0] ?? "");

  for (const name of stale) {
    git("branch", "-D", name);
  }

  const report = [
    `deleted: ${stale.length === 0 ? "none" : stale.join(", ")}`,
    `local: ${git("branch", "--format=%(refname:short)").join(", ")}`,
    `on GitHub: ${git("branch", "-r", "--format=%(refname:short)")
      .filter((name) => name !== "origin/HEAD" && name !== "origin")
      .join(", ")}`,
    `stashes: ${String(git("stash", "list").length)}`,
    `worktrees: ${String(git("worktree", "list").length)}`,
  ];

  process.stdout.write(`${report.join("\n")}\n`);
}

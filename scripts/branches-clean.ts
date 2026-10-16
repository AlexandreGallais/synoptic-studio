// Clean-up of the local clone after a feature (ADR-0038): local branches already merged into
// `develop` or whose branch on GitHub was deleted are removed with their worktrees, stale remote
// branches pruned, and what is left reported (other branches on GitHub, stashes, worktrees). Run
// by `npm run branches:clean` after each merged feature pull request and each release.

import { execFileSync, spawnSync } from "node:child_process";

/** Branches never deleted. */
const KEPT = new Set(["develop", "main"]);

/** Prefix of a branch in `git worktree list --porcelain`. */
const HEADS = "refs/heads/";

/** A local branch as Git reports it. */
export type LocalBranch = {
  /** Its branch on GitHub was deleted (`[gone]`). */
  readonly isGone: boolean;
  /** Its commits are all in `origin/develop`. */
  readonly isMerged: boolean;
  readonly name: string;
};

/** A worktree as `git worktree list --porcelain` reports it. */
export type Worktree = {
  /** Branch checked out in it, empty when its head is detached. */
  readonly branch: string;
  readonly path: string;
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
 * Worktrees listed by Git.
 *
 * @param porcelain - output of `git worktree list --porcelain`, one block per worktree
 * @returns each worktree's path and branch, the main checkout first
 */
export function worktreesOf(porcelain: string): readonly Worktree[] {
  return porcelain
    .split(/\n\s*\n/u)
    .map((block) => block.split("\n"))
    .map((lines) => ({
      branch:
        lines.find((line) => line.startsWith("branch "))?.slice(`branch ${HEADS}`.length) ?? "",
      path: lines.find((line) => line.startsWith("worktree "))?.slice("worktree ".length) ?? "",
    }))
    .filter((worktree) => worktree.path !== "");
}

/**
 * Worktrees to remove: those whose branch is stale or no longer exists, never the main checkout
 * nor a detached one.
 *
 * @param worktrees - worktrees, the main checkout first
 * @param stale - branches about to be deleted
 * @param local - local branches that exist
 * @returns paths to remove
 */
export function staleWorktrees(
  worktrees: readonly Worktree[],
  stale: readonly string[],
  local: readonly string[],
): readonly string[] {
  return worktrees
    .slice(1)
    .filter((worktree) => worktree.branch !== "")
    .filter((worktree) => stale.includes(worktree.branch) || !local.includes(worktree.branch))
    .map((worktree) => worktree.path);
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

/**
 * Removes a worktree, unless it holds changes not committed.
 *
 * @param path - the worktree's folder
 * @returns the reason it was kept, or an empty string once removed
 */
function removeWorktree(path: string): string {
  const result = spawnSync("git", ["worktree", "remove", path], { encoding: "utf8" });

  return result.status === 0 ? "" : result.stderr.trim().replaceAll("\n", " ");
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
  const worktrees = worktreesOf(
    execFileSync("git", ["worktree", "list", "--porcelain"], { encoding: "utf8" }),
  );
  const removals = staleWorktrees(
    worktrees,
    stale,
    branches.map((branch) => branch.name),
  ).map((path) => ({ path, reason: removeWorktree(path) }));
  const kept = removals.filter((removal) => removal.reason !== "");

  git("worktree", "prune");

  // A branch still checked out in a worktree kept cannot be deleted.
  const keptPaths = new Set(kept.map((removal) => removal.path));
  const keptBranches = new Set(
    worktrees.filter((tree) => keptPaths.has(tree.path)).map((tree) => tree.branch),
  );
  const deleted = stale.filter((name) => !keptBranches.has(name));

  for (const name of deleted) {
    git("branch", "-D", name);
  }

  const report = [
    `deleted: ${deleted.length === 0 ? "none" : deleted.join(", ")}`,
    `worktrees kept with changes: ${
      kept.length === 0
        ? "none"
        : kept.map((removal) => `${removal.path} (${removal.reason})`).join(", ")
    }`,
    `local: ${git("branch", "--format=%(refname:short)").join(", ")}`,
    `on GitHub: ${git("branch", "-r", "--format=%(refname:short)")
      .filter((name) => name !== "origin/HEAD" && name !== "origin")
      .join(", ")}`,
    `stashes: ${String(git("stash", "list").length)}`,
    `worktrees: ${String(git("worktree", "list").length)}`,
  ];

  process.stdout.write(`${report.join("\n")}\n`);
}

// Clean-up of the local clone after a feature (ADR-0038): local branches already merged into
// `develop` or whose branch on GitHub was deleted are removed with their worktrees, stale remote
// branches pruned, and what is left reported (other branches on GitHub, stashes, worktrees). A
// branch just made for a story — never pushed, without a commit beyond the branch it was made from
// — and a worktree a running process works in are kept (AUD-020). Run by `npm run branches:clean`
// after each merged feature pull request and each release.

import { execFileSync, spawnSync } from "node:child_process";

/** Branches never deleted. */
const KEPT = new Set(["develop", "main"]);

/** Prefix of a branch in `git worktree list --porcelain`. */
const HEADS = "refs/heads/";

/** Fields of a branch's line of `git for-each-ref`: its name, track and upstream. */
const FIELDS = 3;

/** The branch a story's commits are compared with when it has no other base. */
const DEVELOP = "origin/develop";

/** A local branch as Git reports it. */
export type LocalBranch = {
  /** Its own branch on GitHub was deleted (`[gone]`). */
  readonly isGone: boolean;
  /** Its commits are all in `origin/develop`. */
  readonly isMerged: boolean;
  /** Never pushed under its own name and without a commit beyond its base: a story just started. */
  readonly isFresh: boolean;
  readonly name: string;
};

/** A local branch's line of `git for-each-ref`. */
export type BranchFields = {
  readonly name: string;
  /** `%(upstream:track)`: `[gone]`, `[ahead 1]` or empty. */
  readonly track: string;
  /** `%(upstream:short)`: `origin/<name>` once pushed, the branch it was made from otherwise. */
  readonly upstream: string;
};

/** A worktree as `git worktree list --porcelain` reports it. */
export type Worktree = {
  /** Branch checked out in it, empty when its head is detached. */
  readonly branch: string;
  readonly path: string;
};

/**
 * Whether a branch was pushed under its own name.
 *
 * @param fields - the branch's line of `git for-each-ref`
 * @returns true when its upstream is `origin/<name>`
 */
function isPublished(fields: BranchFields): boolean {
  return fields.upstream === `origin/${fields.name}`;
}

/**
 * The branch a local branch's own commits are counted from: the branch it was made from, which a
 * worktree of `worktree:new` tracks, or `origin/develop`.
 *
 * @param fields - the branch's line of `git for-each-ref`
 * @returns the base to count from
 */
export function baseOf(fields: BranchFields): string {
  return fields.upstream === "" || isPublished(fields) || fields.track === "[gone]"
    ? DEVELOP
    : fields.upstream;
}

/**
 * A local branch read from its line of `git for-each-ref`: gone only when its own branch on GitHub
 * was deleted, never because the branch it was made from was.
 *
 * @param fields - the branch's line
 * @param isMerged - whether its commits are all in `origin/develop`
 * @param ownCommits - its commits beyond its base
 * @returns its flags: gone, merged, fresh
 */
export function readBranch(
  fields: BranchFields,
  isMerged: boolean,
  ownCommits: number,
): LocalBranch {
  const isPushed = isPublished(fields);

  return {
    isFresh: !isPushed && ownCommits === 0,
    isGone: isPushed && fields.track === "[gone]",
    isMerged,
    name: fields.name,
  };
}

/**
 * Local branches to delete: merged or gone, never `main`, `develop`, the one checked out nor a
 * story just started.
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
    .filter((branch) => branch.isGone || (branch.isMerged && !branch.isFresh))
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
 * The working folders of the running processes, from `lsof -d cwd -Fn`.
 *
 * @param lsof - its output: a line `p<pid>` then `n<folder>` per process
 * @returns each process's folder
 */
export function workingFolders(lsof: string): readonly string[] {
  return lsof
    .split("\n")
    .filter((line) => line.startsWith("n/"))
    .map((line) => line.slice(1));
}

/**
 * Whether a running process works in a worktree or in one of its folders.
 *
 * @param path - the worktree's folder
 * @param folders - the working folders of the running processes
 * @returns true when one of them is that folder or lies inside it
 */
export function isInUse(path: string, folders: readonly string[]): boolean {
  return folders.some((folder) => folder === path || folder.startsWith(`${path}/`));
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
 * A branch's commits beyond its base, 0 when Git cannot count them: such a branch is kept.
 *
 * @param fields - the branch's line of `git for-each-ref`
 * @returns the count
 */
function ownCommitsOf(fields: BranchFields): number {
  const result = spawnSync("git", ["rev-list", "--count", `${baseOf(fields)}..${fields.name}`], {
    encoding: "utf8",
  });

  return result.status === 0 ? Number(result.stdout.trim()) : 0;
}

/**
 * Removes a worktree, unless a running process works in it or it holds changes not committed.
 *
 * @param path - the worktree's folder
 * @param folders - the working folders of the running processes
 * @returns the reason it was kept, or an empty string once removed
 */
function removeWorktree(path: string, folders: readonly string[]): string {
  if (isInUse(path, folders)) {
    return "a running process works in it";
  }

  const result = spawnSync("git", ["worktree", "remove", path], { encoding: "utf8" });

  return result.status === 0 ? "" : result.stderr.trim().replaceAll("\n", " ");
}

if (import.meta.main) {
  git("fetch", "--prune", "origin");
  git("worktree", "prune");

  const merged = new Set(git("branch", "--format=%(refname:short)", "--merged", DEVELOP));
  const branches = git(
    "for-each-ref",
    "--format=%(refname:short)|%(upstream:track)|%(upstream:short)",
    "refs/heads",
  ).map((line): LocalBranch => {
    const [name = "", track = "", upstream = ""] = line.split("|", FIELDS);
    const fields = { name, track, upstream };

    return readBranch(fields, merged.has(name), ownCommitsOf(fields));
  });
  const stale = staleBranches(branches, git("branch", "--show-current")[0] ?? "");
  const worktrees = worktreesOf(
    execFileSync("git", ["worktree", "list", "--porcelain"], { encoding: "utf8" }),
  );
  // lsof exits with 1 when it cannot read some process: its output is still read.
  const folders = workingFolders(
    spawnSync("lsof", ["-d", "cwd", "-Fn"], { encoding: "utf8" }).stdout,
  );
  const removals = staleWorktrees(
    worktrees,
    stale,
    branches.map((branch) => branch.name),
  ).map((path) => ({ path, reason: removeWorktree(path, folders) }));
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
    `worktrees kept: ${
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

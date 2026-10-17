import { describe, expect, it } from "vitest";

import {
  baseOf,
  isInUse,
  readBranch,
  staleBranches,
  staleWorktrees,
  workingFolders,
  worktreesOf,
} from "./branches-clean";

/** The feature branch a story's worktree is made from, as `worktree:new` tracks it. */
const FEATURE = "origin/feature/f18-safe-and-cheap-parallel-run";

describe("staleBranches", () => {
  it("deletes merged or gone branches, never main, develop nor the one checked out", () => {
    const branches = [
      { isFresh: false, isGone: false, isMerged: true, name: "main" },
      { isFresh: false, isGone: false, isMerged: true, name: "develop" },
      { isFresh: false, isGone: true, isMerged: false, name: "feat/val-010-validate-f11" },
      {
        isFresh: false,
        isGone: false,
        isMerged: true,
        name: "feature/f11-code-base-ready-for-curves",
      },
      { isFresh: false, isGone: false, isMerged: false, name: "refactor/en-024-work-in-progress" },
      { isFresh: false, isGone: true, isMerged: true, name: "docs/sp-011-research-f12" },
    ];

    expect(staleBranches(branches, "docs/sp-011-research-f12")).toEqual([
      "feat/val-010-validate-f11",
      "feature/f11-code-base-ready-for-curves",
    ]);
  });

  it("[F18.AC8] keeps a story just started, though all its commits are in develop", () => {
    const fresh = { isFresh: true, isGone: false, isMerged: true, name: "fix/aud-020-topic" };

    expect(staleBranches([fresh], "develop")).toEqual([]);
    expect(staleBranches([{ ...fresh, isFresh: false }], "develop")).toEqual(["fix/aud-020-topic"]);
  });
});

describe("readBranch", () => {
  it("[F18.AC8] reads a story just made from its feature as fresh, never gone", () => {
    // `worktree:new` made it from the feature branch; no commit yet.
    const made = { name: "fix/aud-020-topic", track: "", upstream: FEATURE };

    expect(readBranch(made, true, 0)).toEqual({
      isFresh: true,
      isGone: false,
      isMerged: true,
      name: "fix/aud-020-topic",
    });

    // One commit of its own: no longer fresh.
    expect(readBranch({ ...made, track: "[ahead 1]" }, false, 1).isFresh).toBe(false);
    // The feature branch deleted on GitHub: the story's own branch is not gone.
    expect(readBranch({ ...made, track: "[gone]" }, false, 3).isGone).toBe(false);
  });

  it("reads a branch pushed under its own name as gone once GitHub deleted it", () => {
    const pushed = {
      name: "feat/us-061-font-list",
      track: "[gone]",
      upstream: "origin/feat/us-061-font-list",
    };

    expect(readBranch(pushed, false, 0)).toEqual({
      isFresh: false,
      isGone: true,
      isMerged: false,
      name: "feat/us-061-font-list",
    });

    expect(readBranch({ ...pushed, track: "" }, true, 0).isGone).toBe(false);
    // A branch without upstream and without a commit of its own beyond develop.
    expect(readBranch({ name: "spare", track: "", upstream: "" }, true, 0).isFresh).toBe(true);
  });
});

describe("baseOf", () => {
  it("counts a story's commits from the branch it was made from, develop otherwise", () => {
    expect(baseOf({ name: "fix/aud-020-topic", track: "", upstream: FEATURE })).toBe(FEATURE);

    expect(baseOf({ name: "fix/aud-020-topic", track: "[gone]", upstream: FEATURE })).toBe(
      "origin/develop",
    );

    expect(
      baseOf({ name: "fix/aud-020-topic", track: "", upstream: "origin/fix/aud-020-topic" }),
    ).toBe("origin/develop");

    expect(baseOf({ name: "spare", track: "", upstream: "" })).toBe("origin/develop");
  });
});

describe("worktreesOf", () => {
  it("reads each worktree's path and branch, a detached one without branch", () => {
    const porcelain = [
      "worktree /work/SVG editor",
      "HEAD 5f061278",
      "branch refs/heads/develop",
      "",
      "worktree /work/SVG editor/.claude/worktrees/us-061",
      "HEAD 9dea434e",
      "branch refs/heads/feat/us-061-font-list",
      "",
      "worktree /work/SVG editor/.claude/worktrees/review",
      "HEAD fee645b2",
      "detached",
      "",
    ].join("\n");

    expect(worktreesOf(porcelain)).toEqual([
      { branch: "develop", path: "/work/SVG editor" },
      { branch: "feat/us-061-font-list", path: "/work/SVG editor/.claude/worktrees/us-061" },
      { branch: "", path: "/work/SVG editor/.claude/worktrees/review" },
    ]);
  });
});

describe("staleWorktrees", () => {
  it("[F18.AC8] removes the worktree of a merged or deleted branch, never the main one nor a detached one", () => {
    const worktrees = [
      { branch: "feat/us-061-font-list", path: "/main" },
      { branch: "feat/us-061-font-list", path: "/main/.claude/worktrees/us-061" },
      { branch: "chore/en-052-worktrees", path: "/main/.claude/worktrees/en-052" },
      { branch: "chore/en-051-story-finish", path: "/main/.claude/worktrees/en-051" },
      { branch: "", path: "/main/.claude/worktrees/review" },
    ];
    const local = ["develop", "feat/us-061-font-list", "chore/en-052-worktrees"];

    // US-061 merged, EN-051's branch deleted by hand, EN-052 still at work.
    expect(staleWorktrees(worktrees, ["feat/us-061-font-list"], local)).toEqual([
      "/main/.claude/worktrees/us-061",
      "/main/.claude/worktrees/en-051",
    ]);
  });
});

describe("isInUse", () => {
  it("[F18.AC8] knows a worktree a running process works in, or in one of its folders", () => {
    const folders = workingFolders(
      [
        "p101",
        "n/main",
        "p102",
        "n/main/.claude/worktrees/us-061/tools/hooks",
        "p103",
        "n/main/.claude/worktrees/en-0520",
        "",
      ].join("\n"),
    );

    expect(folders).toEqual([
      "/main",
      "/main/.claude/worktrees/us-061/tools/hooks",
      "/main/.claude/worktrees/en-0520",
    ]);

    expect(isInUse("/main/.claude/worktrees/us-061", folders)).toBe(true);
    expect(isInUse("/main/.claude/worktrees/en-0520", folders)).toBe(true);
    // A longer name beside it is not inside it.
    expect(isInUse("/main/.claude/worktrees/en-052", folders)).toBe(false);
    expect(isInUse("/main/.claude/worktrees/us-062", [])).toBe(false);
  });
});

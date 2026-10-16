import { describe, expect, it } from "vitest";

import { staleBranches, staleWorktrees, worktreesOf } from "./branches-clean";

describe("staleBranches", () => {
  it("deletes merged or gone branches, never main, develop nor the one checked out", () => {
    const branches = [
      { isGone: false, isMerged: true, name: "main" },
      { isGone: false, isMerged: true, name: "develop" },
      { isGone: true, isMerged: false, name: "feat/val-010-validate-f11" },
      { isGone: false, isMerged: true, name: "feature/f11-code-base-ready-for-curves" },
      { isGone: false, isMerged: false, name: "refactor/en-024-work-in-progress" },
      { isGone: true, isMerged: true, name: "docs/sp-011-research-f12" },
    ];

    expect(staleBranches(branches, "docs/sp-011-research-f12")).toEqual([
      "feat/val-010-validate-f11",
      "feature/f11-code-base-ready-for-curves",
    ]);
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

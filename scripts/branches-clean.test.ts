import { describe, expect, it } from "vitest";

import { staleBranches } from "./branches-clean";

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

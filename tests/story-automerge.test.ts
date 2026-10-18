import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Text of the story auto-merge workflow, its comments left out. */
const WORKFLOW = readFileSync(
  join(import.meta.dirname, "..", ".github", "workflows", "story-automerge.yml"),
  "utf8",
)
  .split("\n")
  .filter((line) => !line.trimStart().startsWith("#"))
  .join("\n");

/**
 * The pattern of a story's branch, as the workflow's `grep -Eq` reads it.
 *
 * @returns the pattern, or one matching nothing when the workflow has none
 */
function storyPattern(): RegExp {
  const found = /grep -Eq '(?<pattern>[^']+)'/u.exec(WORKFLOW);

  return new RegExp(found?.groups?.["pattern"] ?? String.raw`$.^`, "u");
}

describe("story auto-merge (ADR-0086)", () => {
  it("[F22.AC4] merges a story's pull request into develop by a merge commit", () => {
    expect(WORKFLOW).toContain("github.event.pull_request.base.ref == 'develop'");
    expect(WORKFLOW).toContain(`gh pr merge "$PR_URL" --auto --merge`);
    expect(WORKFLOW).not.toMatch(/--rebase|feature\//u);
  });

  it("[F22.AC4] merges only a labelled, ready pull request of the owner", () => {
    expect(WORKFLOW).toContain("contains(github.event.pull_request.labels.*.name, 'autonomous')");
    expect(WORKFLOW).toContain("github.event.pull_request.author_association == 'OWNER'");
    expect(WORKFLOW).toContain("!github.event.pull_request.draft");
  });

  it("[F22.AC4] still refuses a branch that is not a story's", () => {
    const pattern = storyPattern();

    for (const branch of [
      "chore/en-122-one-story-at-a-time-into-develop",
      "feat/us-061-font-list",
      "feat/val-023-validate-f22",
      "docs/rev-009-review-e17",
      "refactor/nam-004-naming-pass",
    ]) {
      expect(branch).toMatch(pattern);
    }

    for (const branch of [
      "docs/f21-refinement",
      "feature/f22-the-release",
      "chore/tidy-up",
      "chore/en-12-short",
      "dependabot/npm_and_yarn/vite-8.3.4",
      "release/0.3.0",
    ]) {
      expect(branch).not.toMatch(pattern);
    }
  });
});

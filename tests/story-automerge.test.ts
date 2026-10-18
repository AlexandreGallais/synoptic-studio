import { spawnSync } from "node:child_process";
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

/** Indentation of a step's script lines under `run: |`. */
const SCRIPT_INDENT = " ".repeat(10);

/**
 * The script of the step that checks the branches, as GitHub runs it.
 *
 * @returns the script's lines without their indentation, or an empty text when the step is missing
 */
function checkScript(): string {
  const step = WORKFLOW.slice(WORKFLOW.indexOf("- name: Check the branch"));
  const lines = step.slice(step.indexOf("run: |\n") + "run: |\n".length).split("\n");
  const end = lines.findIndex((line) => !line.startsWith(SCRIPT_INDENT));

  return lines
    .slice(0, end)
    .map((line) => line.slice(SCRIPT_INDENT.length))
    .join("\n");
}

/**
 * Whether the check lets a pull request through, its script run by Bash as GitHub's runner does.
 *
 * @param base - the branch the pull request goes into
 * @param branch - the pull request's branch
 * @returns whether the script exits 0
 */
function passes(base: string, branch: string): boolean {
  const run = spawnSync("bash", ["-e", "-c", checkScript()], {
    env: { BASE: base, BRANCH: branch, PATH: process.env["PATH"] ?? "" },
  });

  return run.status === 0;
}

describe("story auto-merge (ADR-0086)", () => {
  it("[F22.AC4] merges into a feature branch or develop, always by a merge commit", () => {
    expect(WORKFLOW).toContain("startsWith(github.event.pull_request.base.ref, 'feature/')");
    expect(WORKFLOW).toContain("github.event.pull_request.base.ref == 'develop'");
    expect(WORKFLOW).toContain(`gh pr merge "$PR_URL" --auto --merge`);
    expect(WORKFLOW).not.toMatch(/--rebase|\$METHOD/u);
  });

  it("[F22.AC4] merges only a labelled, ready pull request of the owner", () => {
    expect(WORKFLOW).toContain("contains(github.event.pull_request.labels.*.name, 'autonomous')");
    expect(WORKFLOW).toContain("github.event.pull_request.author_association == 'OWNER'");
    expect(WORKFLOW).toContain("!github.event.pull_request.draft");
  });

  it("[F22.AC4] lets a story into its feature, and a feature or a review into develop", () => {
    const feature = "feature/f22-release-by-a-button";

    expect(passes(feature, "chore/en-122-one-story-at-a-time-into-develop")).toBe(true);
    expect(passes(feature, "feat/us-061-font-list")).toBe(true);
    expect(passes(feature, "feat/val-023-validate-f22")).toBe(true);
    expect(passes(feature, "docs/aud-024-audit-f22")).toBe(true);
    expect(passes("develop", feature)).toBe(true);
    expect(passes("develop", "docs/rev-006-review")).toBe(true);
  });

  it("[F22.AC4] still refuses a branch that is neither a story's nor a feature's", () => {
    const feature = "feature/f22-release-by-a-button";

    // A story goes into its feature, never straight into develop; a review into develop only.
    expect(passes("develop", "chore/en-122-one-story-at-a-time-into-develop")).toBe(false);
    expect(passes(feature, "docs/rev-006-review")).toBe(false);
    expect(passes(feature, "feature/f21-other")).toBe(false);
    expect(passes(feature, "chore/tidy-up")).toBe(false);
    expect(passes(feature, "chore/en-12-short")).toBe(false);
    expect(passes("develop", "docs/f21-refinement")).toBe(false);
    expect(passes("develop", "feature/f2-short")).toBe(false);
    expect(passes("develop", "dependabot/npm_and_yarn/vite-8.3.4")).toBe(false);
  });
});

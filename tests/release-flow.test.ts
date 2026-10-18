import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Text of the release workflow. */
const WORKFLOW = readFileSync(
  join(import.meta.dirname, "..", ".github", "workflows", "release.yml"),
  "utf8",
);

/**
 * One job of the release workflow, from its name to the next job.
 *
 * @param name - the job's key under `jobs:`
 * @returns the job's lines, comments left out, or an empty text when it is missing
 */
function jobOf(name: string): string {
  const jobs = WORKFLOW.slice(WORKFLOW.indexOf("\njobs:\n"));
  const start = jobs.indexOf(`\n  ${name}:\n`);

  if (start === -1) {
    return "";
  }

  const next = jobs.slice(start + 1).search(/\n {2}[\w-]+:\n/u);
  const body = next === -1 ? jobs.slice(start) : jobs.slice(start, start + 1 + next);

  return body
    .split("\n")
    .filter((line) => !line.trimStart().startsWith("#"))
    .join("\n");
}

describe("release flow (ADR-0067)", () => {
  it("[F18.AC1] runs on pushes to main and to develop", () => {
    expect(WORKFLOW).toContain("branches: [main, develop]");
  });

  it("[F18.AC1] opens the release pull request on develop, never a release", () => {
    const job = jobOf("release-pr");

    expect(job).toContain("if: github.ref_name == 'develop'");
    expect(job).toContain("target-branch: develop");
    expect(job).toContain("skip-github-release: true");
    expect(job).not.toContain("skip-github-pull-request");
  });

  it("[F18.AC1] only tags and publishes on main, from the pull request merged into develop", () => {
    const job = jobOf("release-please");

    expect(job).toContain("if: github.ref_name == 'main'");
    expect(job).toContain("target-branch: develop");
    expect(job).toContain("skip-github-pull-request: true");
    expect(job).not.toContain("skip-github-release");
  });

  it("[F18.AC1] merges no pull request by itself", () => {
    expect(WORKFLOW).not.toMatch(/gh pr merge/u);
  });
});

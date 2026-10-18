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

describe("release flow (ADR-0079)", () => {
  it("[F21.AC6] runs on pushes to main only, and by hand", () => {
    expect(WORKFLOW).toContain("on:\n  push:\n    branches: [main]\n  workflow_dispatch:\n");
  });

  it("[F21.AC6] publishes on main with the project's tool, on GITHUB_TOKEN", () => {
    const job = jobOf("release");

    expect(job).toContain("if: github.ref_name == 'main'");
    expect(job).toContain("- run: cargo build --release --locked -q -p release\n");
    expect(job).toContain("run: target/release/release publish\n");
    expect(job).toContain(`GH_TOKEN: \${{ github.token }}`);
    expect(job).toContain("fetch-depth: 2");
  });

  it("[F21.AC6] publishes the site after a release, its output read", () => {
    const job = jobOf("docs");

    expect(job).toContain("needs: release\n");
    expect(job).toContain("if: needs.release.outputs.released == 'true'");
  });

  it("[F21.AC6] uses no release-please, no personal token, and merges or commits nothing", () => {
    expect(WORKFLOW).not.toMatch(/release-please|secrets\.|gh pr merge|git commit|git push/u);
  });
});

import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** The repository's root. */
const ROOT = join(import.meta.dirname, "..");

/**
 * Text of a workflow.
 *
 * @param file - its file name in `.github/workflows/`
 * @returns its text, or an empty text when it is missing
 */
function workflowOf(file: string): string {
  const path = join(ROOT, ".github", "workflows", file);

  return existsSync(path) ? readFileSync(path, "utf8") : "";
}

/** Text of the release workflow. */
const WORKFLOW = workflowOf("release.yml");

/** Text of the site's workflow. */
const SITE = workflowOf("site.yml");

/** Text of the Rust caches' workflow. */
const RUST_CACHES = workflowOf("rust-caches.yml");

/**
 * A Markdown link or reference definition to a file of the repository named `CHANGELOG.md`; an
 * address of another site, in code font or not, is left out (the history, `LOG-history` of the records, quotes
 * Dependabot's links to the changelogs of the actions).
 */
const CHANGELOG_LINK =
  /\]\((?!`?https?:)[^)\s]*CHANGELOG\.md|^\[[^\]]+\]: (?!https?:)\S*CHANGELOG\.md/mu;

/**
 * One job of a workflow, from its name to the next job.
 *
 * @param workflow - text of the workflow
 * @param name - the job's key under `jobs:`
 * @returns the job's lines, comments left out, or an empty text when it is missing
 */
function jobOf(workflow: string, name: string): string {
  const jobs = workflow.slice(workflow.indexOf("\njobs:\n"));
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

/**
 * A JSON file of the root, parsed.
 *
 * @param file - its name
 * @returns its value
 */
function jsonOf(file: string): unknown {
  return JSON.parse(readFileSync(join(ROOT, file), "utf8"));
}

/**
 * The version a parsed manifest holds at a path of keys.
 *
 * @param value - the parsed manifest
 * @param keys - the keys down to the object holding `version`
 * @returns its version, or an empty text when there is none
 */
function versionAt(value: unknown, keys: readonly string[]): string {
  const found = keys.reduce<unknown>(
    (object, key) =>
      typeof object === "object" && object !== null ? Reflect.get(object, key) : undefined,
    value,
  );
  const version: unknown =
    typeof found === "object" && found !== null ? Reflect.get(found, "version") : undefined;

  return typeof version === "string" ? version : "";
}

describe("release flow (ADR-0086)", () => {
  it("[F22.AC2] [F28.AC2] runs the release by its button alone", () => {
    // The site is deployed by its own workflow, the caches saved by theirs.
    expect(WORKFLOW).toContain("\non:\n  workflow_dispatch:\n\n");
    expect(WORKFLOW).not.toContain("push:");
  });

  it("[F22.AC2] [F28.AC2] releases with the project's tool, which checks the branch, on GITHUB_TOKEN", () => {
    const job = jobOf(WORKFLOW, "release");

    // `release publish` refuses a branch other than main (tools/release/tests/publish.rs).
    expect(job).not.toContain("if:");
    expect(job).toContain("- run: cargo build --release --locked -q -p release\n");
    expect(job).toContain("run: target/release/release publish\n");
    expect(job).toContain(`GH_TOKEN: \${{ github.token }}`);
    // Every commit and tag: the last tag and the commits since make the version.
    expect(job).toContain("fetch-depth: 0\n");
    expect(job).not.toContain("outputs:");
  });

  it("[F22.AC2] [F28.AC2] deploys the site on a push to main alone, never on develop nor by the button", () => {
    const job = jobOf(SITE, "site");

    expect(SITE).toContain("\non:\n  push:\n    branches: [main]\n\n");
    expect(job).not.toContain("if:");
    expect(job).not.toContain("needs:");
    expect(job).toContain("uses: actions/deploy-pages@");
    expect(WORKFLOW).not.toContain("deploy-pages");
  });

  it("[F22.AC2] [F24.AC3] [F28.AC2] fetches every target's crates on main before its caches are saved", () => {
    const job = jobOf(RUST_CACHES, "rust-caches");
    const fetch = job.indexOf("- run: cargo fetch --locked\n");

    // Every push to main, the one trigger of the workflow: no condition on the job; develop's
    // caches are the develop CI's (tests/ci-caches.test.ts).
    expect(job).not.toMatch(/^ {4}if:/mu);
    expect(fetch).toBeGreaterThan(job.indexOf("key: rust-build-"));
    expect(fetch).toBeLessThan(job.indexOf("- run: cargo build --release --workspace --locked\n"));
  });

  it("[F18.AC1] [F22.AC2] only releases on main: no release-please, no personal token, no merge, no commit", () => {
    for (const workflow of [WORKFLOW, SITE, RUST_CACHES]) {
      expect(workflow).not.toMatch(
        /release-please|secrets\.|gh pr merge|git commit|git push|git tag/u,
      );
    }
  });

  it("[F22.AC2] keeps no version in the manifests: the tags hold it", () => {
    expect(versionAt(jsonOf("package.json"), [])).toBe("0.0.0-development");
    expect(versionAt(jsonOf("package-lock.json"), [])).toBe("0.0.0-development");
    expect(versionAt(jsonOf("package-lock.json"), ["packages", ""])).toBe("0.0.0-development");
  });

  it("[F22.AC2] has no CHANGELOG.md, and nothing links to it nor leaves it out", () => {
    expect(existsSync(join(ROOT, "CHANGELOG.md"))).toBe(false);

    const tracked = execFileSync("git", ["ls-files", "-z"], { cwd: ROOT, encoding: "utf8" })
      .split("\0")
      .filter((path) => path !== "");
    const linking = tracked
      .filter((path) => path.endsWith(".md"))
      .filter((path) => CHANGELOG_LINK.test(readFileSync(join(ROOT, path), "utf8")));
    const configurations = [
      ".oxfmtrc.json",
      ".oxlintrc.json",
      ".github/workflows/links.yml",
      "tests/check-targets.test.ts",
    ].filter((path) => readFileSync(join(ROOT, path), "utf8").includes("CHANGELOG.md"));

    expect(linking).toStrictEqual([]);
    expect(configurations).toStrictEqual([]);
  });
});

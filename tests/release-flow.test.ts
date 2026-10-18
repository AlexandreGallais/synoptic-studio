import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** The repository's root. */
const ROOT = join(import.meta.dirname, "..");

/** Text of the release workflow. */
const WORKFLOW = readFileSync(join(ROOT, ".github", "workflows", "release.yml"), "utf8");

/**
 * A Markdown link or reference definition to a file of the repository named `CHANGELOG.md`; an
 * address of another site, in code font or not, is left out (`docs/process/history.md` quotes
 * Dependabot's links to the changelogs of the actions).
 */
const CHANGELOG_LINK =
  /\]\((?!`?https?:)[^)\s]*CHANGELOG\.md|^\[[^\]]+\]: (?!https?:)\S*CHANGELOG\.md/mu;

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
  it("[F22.AC2] runs on pushes to main and by its button", () => {
    expect(WORKFLOW).toContain("on:\n  push:\n    branches: [main]\n  workflow_dispatch:\n");
  });

  it("[F22.AC2] releases by the button on main only, with the project's tool, on GITHUB_TOKEN", () => {
    const job = jobOf("release");

    expect(job).toContain(
      "if: github.event_name == 'workflow_dispatch' && github.ref_name == 'main'\n",
    );

    expect(job).toContain("- run: cargo build --release --locked -q -p release\n");
    expect(job).toContain("run: target/release/release publish\n");
    expect(job).toContain(`GH_TOKEN: \${{ github.token }}`);
    // Every commit and tag: the last tag and the commits since make the version.
    expect(job).toContain("fetch-depth: 0\n");
    expect(job).not.toContain("outputs:");
  });

  it("[F22.AC2] deploys the site on a push to main, never by the button", () => {
    const job = jobOf("docs");

    expect(job).toContain("if: github.event_name == 'push'\n");
    expect(job).not.toContain("needs:");
    expect(job).toContain("uses: actions/deploy-pages@");
  });

  it("[F22.AC2] fetches every target's crates on main before its caches are saved", () => {
    const job = jobOf("rust-cache");
    const fetch = job.indexOf("- run: cargo fetch --locked\n");

    expect(job).toContain("if: github.ref_name == 'main' && github.event_name == 'push'\n");
    expect(fetch).toBeGreaterThan(job.indexOf("key: rust-build-"));
    expect(fetch).toBeLessThan(job.indexOf("- run: npm run rust:build\n"));
  });

  it("[F18.AC1] [F22.AC2] only releases on main: no release-please, no personal token, no merge, no commit", () => {
    expect(WORKFLOW).not.toMatch(
      /release-please|secrets\.|gh pr merge|git commit|git push|git tag/u,
    );
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
      ".secret-scan-ignore",
      ".github/workflows/links.yml",
      "tests/check-targets.test.ts",
    ].filter((path) => readFileSync(join(ROOT, path), "utf8").includes("CHANGELOG.md"));

    expect(linking).toStrictEqual([]);
    expect(configurations).toStrictEqual([]);
  });
});

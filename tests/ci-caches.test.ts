import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/** Text of the CI of the pull requests into main. */
const CI = readFileSync(join(ROOT, ".github", "workflows", "ci.yml"), "utf8");

/** Text of the release workflow, whose run on main saves the caches. */
const RELEASE = readFileSync(join(ROOT, ".github", "workflows", "release.yml"), "utf8");

/** The manifest of the repository, parsed. */
const MANIFEST: unknown = JSON.parse(readFileSync(join(ROOT, "package.json"), "utf8"));

/** The `scripts` field of the manifest, or nothing when it is not an object. */
const SCRIPTS_FIELD: unknown =
  typeof MANIFEST === "object" && MANIFEST !== null ? Reflect.get(MANIFEST, "scripts") : {};

/** The npm scripts of `package.json`, by name. */
const SCRIPTS: ReadonlyMap<string, string> = new Map(
  Object.entries(
    typeof SCRIPTS_FIELD === "object" && SCRIPTS_FIELD !== null ? SCRIPTS_FIELD : {},
  ).map(([name, command]): [string, string] => [name, String(command)]),
);

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
 * The caches of a job: each `key:` line with the `path:` lines before it.
 *
 * @param job - the job's lines
 * @returns each cache's paths and key, in the job's order
 */
function cachesOf(job: string): readonly string[] {
  return job
    .matchAll(/path: \|\n(?<paths>(?: {12}\S.*\n)+) {10}key: (?<key>.*)\n/gu)
    .map((match) => `${match.groups?.["paths"] ?? ""}key: ${match.groups?.["key"] ?? ""}`)
    .toArray();
}

/**
 * A variable of a workflow's `env:`.
 *
 * @param workflow - text of the workflow
 * @param name - the variable's name
 * @returns its value, or an empty text when it is missing
 */
function environmentOf(workflow: string, name: string): string {
  const pattern = new RegExp(String.raw`\n {2}${name}: (?<value>\S+)\n`, "u");

  return pattern.exec(workflow)?.groups?.["value"] ?? "";
}

describe("the Rust caches of the CI (EN-111)", () => {
  const lanes = jobOf(CI, "lanes");
  const warm = jobOf(RELEASE, "rust-cache");

  it("[F21.AC4] restores the toolchain, the cargo tools and the build in the CI, never saves them", () => {
    // A pull request's cache is scoped to its merge ref: only a push to develop or main saves
    // (REF-GH-CACHE).
    expect(cachesOf(lanes)).toHaveLength(2);
    expect(lanes.match(/uses: actions\/cache\/restore@v\d+/gu)).toHaveLength(2);
    expect(CI).not.toMatch(/uses: actions\/cache(?:\/save)?@/u);
  });

  it("[F21.AC4] [F24.AC3] saves the same caches on a push to develop or main, under the same keys and paths", () => {
    // A pull request into main restores the caches of its head branch, develop, and of its base
    // and default branch, main (REF-GH-CACHE).
    expect(RELEASE).toContain("\non:\n  push:\n    branches: [develop, main]\n");
    expect(warm).toContain("if: github.event_name == 'push'\n");
    expect(warm.match(/uses: actions\/cache@v\d+/gu)).toHaveLength(2);
    expect(cachesOf(warm)).toStrictEqual(cachesOf(lanes));
  });

  it("[F21.AC4] keys the build on Cargo.lock and the toolchain, the tools on their versions", () => {
    const [tools, build] = cachesOf(lanes);

    expect(tools).toContain(
      `key: rust-tools-\${{ runner.os }}-\${{ hashFiles('rust-toolchain.toml') }}-cargo-llvm-cov-\${{ env.CARGO_LLVM_COV_VERSION }}-cargo-deny-\${{ env.CARGO_DENY_VERSION }}`,
    );

    expect(build).toContain(
      `key: rust-build-\${{ runner.os }}-\${{ hashFiles('rust-toolchain.toml', 'Cargo.lock') }}`,
    );
  });

  it("[F21.AC4] starts each new key on main from an empty build, never an older key's", () => {
    expect(warm).not.toContain("restore-keys");
  });

  it("[F21.AC4] installs the cargo tools only on a missed key, at the versions the scripts name", () => {
    const install = `run: cargo install --locked cargo-llvm-cov@\${{ env.CARGO_LLVM_COV_VERSION }} cargo-deny@\${{ env.CARGO_DENY_VERSION }}\n`;

    // In the CI, by the Rust lane alone, the one running them (EN-124).
    expect(lanes).toContain(
      `- if: matrix.scripts == 'check:rust' && steps.tools.outputs.cache-hit != 'true'\n        ${install}`,
    );

    expect(warm).toContain(`- if: steps.tools.outputs.cache-hit != 'true'\n        ${install}`);

    for (const workflow of [CI, RELEASE]) {
      expect(SCRIPTS.get("rust:coverage")).toContain(
        `cargo-llvm-cov@${environmentOf(workflow, "CARGO_LLVM_COV_VERSION")} `,
      );

      expect(SCRIPTS.get("rust:advisories")).toContain(
        `cargo-deny@${environmentOf(workflow, "CARGO_DENY_VERSION")}, `,
      );
    }
  });

  it("[F21.AC4] compiles on main what check:rust compiles, running no test", () => {
    // Same commands and flags, so that the pull request compiles no dependency again.
    expect(warm).toContain("- run: npm run rust:build\n");

    expect(SCRIPTS.get("check:rust")).toContain(
      "cargo clippy --workspace --all-targets --locked --quiet && ",
    );

    expect(warm).toContain("- run: cargo clippy --workspace --all-targets --locked --quiet\n");

    expect(SCRIPTS.get("rust:coverage")).toContain(
      "; cargo llvm-cov --workspace --locked --quiet ",
    );

    // `--list` is passed to the test binaries, which print their tests' names and run none.
    expect(warm).toContain("- run: cargo llvm-cov --workspace --locked --no-report -- --list\n");

    expect(SCRIPTS.get("check:rust")).toContain(
      "cargo check --workspace --all-targets --locked --quiet --target x86_64-pc-windows-msvc && ",
    );

    expect(warm).toContain(
      "- run: cargo check --workspace --all-targets --locked --quiet --target x86_64-pc-windows-msvc\n",
    );
  });

  it("[F21.AC4] builds the site on main's caches, restored only", () => {
    const site = jobOf(RELEASE, "docs");

    expect(cachesOf(site)).toStrictEqual(cachesOf(warm));
    expect(site.match(/uses: actions\/cache\/restore@v\d+/gu)).toHaveLength(2);
    expect(site).not.toMatch(/uses: actions\/cache(?:\/save)?@/u);
  });

  it("[F21.AC4] checks the history with the lint lane's binary, building none", () => {
    const history = jobOf(CI, "history");

    expect(lanes).toContain(
      "- run: npm run rust:build\n      - if: matrix.scripts == 'lint'\n        uses: actions/upload-artifact@v7\n        with:\n          name: git-hooks\n          path: target/release/git-hooks\n",
    );

    expect(history).toContain("needs: lanes\n");

    expect(history).toContain(
      "- uses: actions/download-artifact@v8\n        with:\n          name: git-hooks\n          path: target/release\n      - run: chmod +x target/release/git-hooks\n",
    );

    expect(history).not.toContain("cargo ");
  });

  it("[F21.AC4] runs on demand the lanes alone, the jobs reading a pull request skipped", () => {
    expect(CI).toContain("\non:\n  pull_request:\n    branches: [main]\n  workflow_dispatch:\n");

    for (const job of ["history", "dependencies"]) {
      expect(jobOf(CI, job)).toContain(`if: \${{ github.event_name == 'pull_request' }}\n`);
    }
  });

  it("[F21.AC6] skips no release-please branch, release-please gone (EN-112)", () => {
    const codeql = readFileSync(join(ROOT, ".github", "workflows", "codeql.yml"), "utf8");

    expect(`${CI}\n${codeql}`).not.toContain("release-please");
  });
});

import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

import { CHECK_ALL_LANES } from "./check-all-lanes";

/** Root of the repository. */
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

/** Text of the CI of the pull requests into main. */
const CI = workflowOf("ci.yml");

/** Text of the Rust caches' workflow, whose run on a push to main saves main's caches. */
const RUST_CACHES = workflowOf("rust-caches.yml");

/** Text of the develop CI, whose run after each merge into develop saves develop's caches. */
const DEVELOP_CI = workflowOf("develop-ci.yml");

/** Text of the CI into main's ruleset, its required checks, in the CI's page. */
const MAIN_RULESET =
  readFileSync(join(ROOT, "docs", "tooling", "ci.md"), "utf8")
    .split("\n")
    .find((line) => line.trimStart().startsWith("| `main` | default branch |")) ?? "";

/** Text of the release's workflow, run by its button. */
const RELEASE = workflowOf("release.yml");

/** Text of the site's workflow, whose run on a push to main restores the caches, saving none. */
const SITE = workflowOf("site.yml");

/** The checks of `check:all`, by name, with their commands (ADR-0103). */
const CHECKS: ReadonlyMap<string, string> = new Map(
  CHECK_ALL_LANES.map(({ command, name }): [string, string] => [name, command]),
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
  const warm = jobOf(RUST_CACHES, "rust-caches");
  const site = jobOf(SITE, "site");

  it("[F21.AC4] restores the toolchain, the cargo tools and the build in the CI, never saves them", () => {
    // A pull request's cache is scoped to its merge ref: only a push to develop or main saves
    // (REF-GH-CACHE).
    expect(cachesOf(lanes)).toHaveLength(2);
    expect(lanes.match(/uses: actions\/cache\/restore@v\d+/gu)).toHaveLength(2);
    expect(CI).not.toMatch(/uses: actions\/cache(?:\/save)?@/u);
  });

  it("[F21.AC4] [F24.AC3] [F28.AC2] [F30.AC1] saves the same caches on main, under the same keys and paths, develop's by the develop CI", () => {
    // A pull request into main restores the caches of its head branch, develop, and of its base
    // and default branch, main; a run on main restores main's alone (REF-GH-CACHE), so the site
    // and the release find the caches the push to main saved. Develop's are the develop CI's.
    expect(RUST_CACHES).toContain("\non:\n  push:\n    branches: [main]\n\n");
    // No condition on the job: its one step's is the tools' cache missed.
    expect(warm.match(/if:/gu)).toHaveLength(1);
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

  it("[F24.AC3] restores in the CI the build of an older key by its prefix, compiling what changed", () => {
    // A new Cargo.lock misses the exact key: develop's or main's latest build is restored instead.
    expect(lanes).toContain(
      `key: rust-build-\${{ runner.os }}-\${{ hashFiles('rust-toolchain.toml', 'Cargo.lock') }}\n          restore-keys: rust-build-\${{ runner.os }}-\n`,
    );
  });

  it("[F21.AC4] [F28.AC2] starts each new key on develop and main from an empty build, never an older key's", () => {
    // The one workflow that saves them, on both branches.
    expect(warm).not.toContain("restore-keys");
  });

  it("[F21.AC4] installs the cargo tools only on a missed key, at the versions the scripts name", () => {
    const install = `run: cargo install --locked cargo-llvm-cov@\${{ env.CARGO_LLVM_COV_VERSION }} cargo-deny@\${{ env.CARGO_DENY_VERSION }}\n`;

    // In the CI, each by the lanes calling it, as their matrix entries say (EN-124, EN-133).
    expect(lanes).toContain(
      `- if: matrix.cargo-llvm-cov && steps.tools.outputs.cache-hit != 'true'\n        run: cargo install --locked cargo-llvm-cov@\${{ env.CARGO_LLVM_COV_VERSION }}\n`,
    );

    expect(lanes).toContain(
      `- if: matrix.cargo-deny && steps.tools.outputs.cache-hit != 'true'\n        run: cargo install --locked cargo-deny@\${{ env.CARGO_DENY_VERSION }}\n`,
    );

    expect(warm).toContain(`- if: steps.tools.outputs.cache-hit != 'true'\n        ${install}`);

    // The site and the release key the tools' cache they restore on the same versions.
    for (const workflow of [CI, RUST_CACHES, DEVELOP_CI, RELEASE, SITE]) {
      expect(CHECKS.get("rust:coverage")).toContain(
        `cargo-llvm-cov@${environmentOf(workflow, "CARGO_LLVM_COV_VERSION")} `,
      );

      expect(CHECKS.get("check:rust")).toContain(
        `cargo-deny@${environmentOf(workflow, "CARGO_DENY_VERSION")}, `,
      );
    }
  });

  it("[F21.AC4] [F30.AC1] compiles on main what check:rust compiles, running no test", () => {
    // Same commands and flags, so that the pull request compiles no dependency again.
    expect(warm).toContain("- run: cargo build --release --workspace --locked\n");

    expect(CHECKS.get("check:rust")).toContain(
      "cargo clippy --workspace --all-targets --locked --quiet && ",
    );

    expect(warm).toContain("- run: cargo clippy --workspace --all-targets --locked --quiet\n");

    expect(CHECKS.get("rust:coverage")).toContain("; cargo llvm-cov --workspace --locked --quiet ");

    // `--list` is passed to the test binaries, which print their tests' names and run none.
    expect(warm).toContain("- run: cargo llvm-cov --workspace --locked --no-report -- --list\n");

    expect(CHECKS.get("check:rust")).toContain(
      "cargo check --workspace --all-targets --locked --quiet --target x86_64-pc-windows-msvc && ",
    );

    expect(warm).toContain(
      "- run: cargo check --workspace --all-targets --locked --quiet --target x86_64-pc-windows-msvc\n",
    );
  });

  it("[F24.AC3] [F28.AC2] restores in the release job the caches the lanes and rust-caches save, restored only", () => {
    const release = jobOf(RELEASE, "release");

    expect(cachesOf(release)).toStrictEqual(cachesOf(warm));
    expect(release.match(/uses: actions\/cache\/restore@v\d+/gu)).toHaveLength(2);
    expect(RELEASE).not.toMatch(/uses: actions\/cache(?:\/save)?@/u);
  });

  it("[F21.AC4] [F28.AC2] builds the site on the caches rust-caches saves on main, restored only", () => {
    // The tools' key promises cargo-llvm-cov and cargo-deny, which the site never installs, and
    // rust-caches.yml saves both caches on the same push to main: the site saves neither.
    expect(cachesOf(site)).toStrictEqual(cachesOf(warm));
    expect(site.match(/uses: actions\/cache\/restore@v\d+/gu)).toHaveLength(2);
    expect(SITE).not.toMatch(/uses: actions\/cache(?:\/save)?@/u);
    expect(site).not.toContain("cargo install");
  });

  it("[F21.AC4] checks the history with the lint lane's binary, building none", () => {
    const history = jobOf(CI, "history");

    expect(lanes).toContain(
      "run: npm ci\n      - if: matrix.git-hooks\n        uses: actions/upload-artifact@v7\n        with:\n          name: git-hooks\n          path: target/release/git-hooks\n",
    );

    expect(history).toContain("needs: lanes\n");

    expect(history).toContain(
      "- uses: actions/download-artifact@v8\n        with:\n          name: git-hooks\n          path: target/release\n      - run: chmod +x target/release/git-hooks\n",
    );

    expect(history).not.toContain("cargo ");
  });

  it("[F30.AC1] runs the Rust checks and tests on Linux in the develop CI, then saves its caches", () => {
    const job = jobOf(DEVELOP_CI, "rust");
    const coverage = job.indexOf("  run: target/release/check-all --lane rust-coverage\n");

    expect(job).toContain("runs-on: ubuntu-latest\n");

    // The cargo tools installed on a missed key, the npm packages, whose `prepare` builds the
    // release binaries and which the formatter's tests run, every target's crates and the RustSec
    // database the advisories read offline (EN-116, EN-126). The install and the checks are named,
    // the saves reading their outcome (REF-GH-CONTEXTS).
    expect(job).toContain(
      `- if: steps.tools.outputs.cache-hit != 'true'\n        id: install\n        run: cargo install --locked cargo-llvm-cov@\${{ env.CARGO_LLVM_COV_VERSION }} cargo-deny@\${{ env.CARGO_DENY_VERSION }}\n`,
    );

    expect(job).toContain("- run: npm ci\n      - run: cargo fetch --locked\n");

    expect(job).toContain(
      "- run: cargo deny fetch db\n      - id: checks\n        run: target/release/check-all --lane rust\n",
    );

    // The tests under coverage run even after a failing check, the run not cancelled
    // (REF-GH-EXPRESSIONS), so that a red check still compiles them.
    expect(job).toContain(
      `- if: \${{ !cancelled() }}\n        run: target/release/check-all --lane rust-coverage\n`,
    );

    expect(coverage).toBeGreaterThan(job.indexOf("  run: target/release/check-all --lane rust\n"));
    // Saved after the tests, even when one failed, unless their key was hit (REF-GH-CACHE-SAVE).
    expect(job.indexOf("uses: actions/cache/save@")).toBeGreaterThan(coverage);
  });

  it("[F30.AC1] restores and saves in the develop CI the caches the lanes restore, each new key from an empty build", () => {
    const job = jobOf(DEVELOP_CI, "rust");

    // Restored, then saved, the toolchain and cargo tools' cache, then the build's.
    expect(cachesOf(job)).toStrictEqual([
      ...cachesOf(jobOf(CI, "lanes")),
      ...cachesOf(jobOf(CI, "lanes")),
    ]);

    expect(job.match(/uses: actions\/cache\/restore@v\d+/gu)).toHaveLength(2);
    expect(job.match(/uses: actions\/cache\/save@v\d+/gu)).toHaveLength(2);
    expect(job).not.toMatch(/uses: actions\/cache@/u);
    expect(job).not.toContain("restore-keys");
    expect(job).toContain("- id: tools\n        uses: actions/cache/restore@");
    expect(job).toContain("- id: build\n        uses: actions/cache/restore@");
  });

  it("[F30.AC1] saves in the develop CI the tools once installed, the build once checked, never half made", () => {
    const job = jobOf(DEVELOP_CI, "rust");

    // A saved cache never changes (REF-GH-CACHE): the tools saved after a failed install would
    // miss a binary under their full key, every run then skipping the install and failing; a build
    // saved after npm CI's or the checks' failure would lack their compilations for the life of
    // its Cargo.lock. A step after a failure is skipped (REF-GH-EXPRESSIONS), so the checks passed
    // means npm CI, the fetches and the install passed too; a failing test does not stop the save.
    expect(job).toContain(
      `- if: \${{ !cancelled() && steps.tools.outputs.cache-hit != 'true' && steps.install.outcome == 'success' }}\n        uses: actions/cache/save@`,
    );

    expect(job).toContain(
      `- if: \${{ !cancelled() && steps.build.outputs.cache-hit != 'true' && steps.checks.outcome == 'success' }}\n        uses: actions/cache/save@`,
    );
  });

  it("[F30.AC1] lets no run of the develop CI cancel another, nor a failing check pass", () => {
    // A concurrency group cancels a pending run, or with `cancel-in-progress` the running one,
    // whose caches then go unsaved; `continue-on-error` would turn a red check green.
    expect(DEVELOP_CI).toContain("\njobs:\n");
    expect(DEVELOP_CI).not.toMatch(/^\s*concurrency:/mu);
    expect(DEVELOP_CI).not.toContain("cancel-in-progress");
    expect(DEVELOP_CI).not.toContain("continue-on-error");
  });

  it("[F30.AC1] requires no check of the develop CI in main's ruleset", () => {
    const name =
      /^ {4}name: (?<name>.+)$/mu.exec(jobOf(DEVELOP_CI, "rust"))?.groups?.["name"] ?? "";

    // Its run is on develop's head, the review's head commit: a job named as a required check
    // would make that check ambiguous on the review (REF-GH-PROTECTED-BRANCHES).
    expect(name).not.toBe("");
    expect(MAIN_RULESET).toContain("`Rust coverage`");
    expect(MAIN_RULESET).not.toContain(`\`${name}\``);
    expect(jobOf(CI, "lanes")).not.toContain(name);
  });

  it("[F21.AC6] skips no release-please branch, release-please gone (EN-112)", () => {
    const codeql = readFileSync(join(ROOT, ".github", "workflows", "codeql.yml"), "utf8");

    expect(`${CI}\n${codeql}`).not.toContain("release-please");
  });
});

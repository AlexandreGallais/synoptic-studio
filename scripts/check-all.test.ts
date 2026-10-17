import { execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

import {
  BEFORE_LANES,
  DOCS_LANE,
  LANES,
  baseCandidates,
  changedSince,
  commandOf,
  lanesToRun,
  nearestBase,
  readScripts,
  reportOf,
  touchesSite,
} from "./check-all";

/**
 * Order of two script names.
 *
 * @param left - a name
 * @param right - another name
 * @returns negative, zero or positive, as `localeCompare`
 */
function byName(left: string, right: string): number {
  return left.localeCompare(right);
}

/** The npm scripts of the repository, by name. */
const SCRIPTS = readScripts();

/**
 * The release binaries a command writes into `target/release/`, through the npm scripts it runs:
 * `cargo run` copies its crate's binary there again even when it is up to date, and the release
 * build of the workspace every binary (`workspace`).
 *
 * @param command - a shell command
 * @returns the crates whose binaries it writes
 */
function releaseBinariesOf(command: string): readonly string[] {
  const runs = command
    .matchAll(/cargo run --release -q -p (?<crate>[\w-]+)/gu)
    .map((match) => match.groups?.["crate"] ?? "")
    .toArray();
  const builds = command.includes("cargo build --release") ? ["workspace"] : [];
  const nested = command
    .matchAll(/npm run (?:--silent )?(?<script>[\w:]+)/gu)
    .flatMap((match) => releaseBinariesOf(SCRIPTS.get(match.groups?.["script"] ?? "") ?? ""))
    .toArray();

  return [...runs, ...builds, ...nested];
}

/**
 * The release binaries the test files write: a test may start `cargo run --release -p <crate>` as
 * well as a script, in the `test` lane (`tests/declarations.test.ts` runs `declarations`).
 */
const TEST_FILE_BINARIES: readonly string[] = execFileSync("git", ["ls-files", "*.test.ts"], {
  cwd: join(import.meta.dirname, ".."),
  encoding: "utf8",
})
  .split("\n")
  .filter((path) => path !== "")
  .flatMap((path) =>
    readFileSync(join(import.meta.dirname, "..", path), "utf8")
      .matchAll(/"run",\s*"--release",\s*"-q",\s*"-p",\s*"(?<crate>[\w-]+)"/gu)
      .map((match) => match.groups?.["crate"] ?? "")
      .toArray(),
  );

describe("check-all", () => {
  it("runs every check of check, the playground after the site", () => {
    const manifest = readFileSync(join(import.meta.dirname, "..", "package.json"), "utf8");
    const check = /"check": "(?<line>[^"]+)"/u.exec(manifest)?.groups?.["line"] ?? "";
    const listed = check.match(/(?<=npm run )[\w:]+/gu) ?? [];

    expect(LANES.flat().toSorted(byName)).toEqual(
      [...listed, "deps:outdated", "deps:tools", "docs:build", "playground:build"].toSorted(byName),
    );

    expect(LANES.find((lane) => lane.includes("playground:build"))).toEqual([
      "docs:build",
      "playground:build",
    ]);
  });

  it("[F19.AC9] runs the Rust checks in a lane of their own", () => {
    expect(LANES).toContainEqual(["check:rust"]);
  });

  it("[F19.AC9] writes no release binary from two lanes, nor the workspace's during the lanes", () => {
    // EN-069's pre-push: one lane's `cargo run` replaced target/release/heavy-lock while another
    // lane's started it, which failed with `could not execute process … No such file or directory`.
    const written = LANES.map((lane) => [
      ...lane.flatMap((name) => releaseBinariesOf(commandOf(SCRIPTS.get(name) ?? ""))),
      ...(lane.includes("test") ? TEST_FILE_BINARIES : []),
    ]);
    const shared = written
      .flat()
      .filter((crate) => written.filter((lane) => lane.includes(crate)).length > 1);

    expect(TEST_FILE_BINARIES).toContain("declarations");
    expect(written.flat()).not.toContain("workspace");
    expect(new Set(shared)).toEqual(new Set());
  });

  it("[F19.AC9] builds the release binaries once, before the lanes", () => {
    expect(BEFORE_LANES).toBe("rust:build");
    expect(LANES.flat()).not.toContain(BEFORE_LANES);
  });

  it("runs a script that takes the heavy lock without it, check:all holding it", () => {
    expect(commandOf("cargo run --release -q -p heavy-lock -- vitest run --coverage")).toBe(
      "vitest run --coverage",
    );

    expect(commandOf("cargo run --release -q -p heavy-lock -- --lock merge node x.ts")).toBe(
      "--lock merge node x.ts",
    );

    expect(commandOf("tsc --noEmit")).toBe("tsc --noEmit");
    expect(commandOf("")).toBe("");
  });

  it("reports a check by its time, a failed one with its output", () => {
    expect(reportOf({ durationMs: 23_940, isOk: true, name: "lint", output: "x" })).toBe(
      "✓ lint 23.9 s",
    );

    expect(reportOf({ durationMs: 1000, isOk: false, name: "test", output: "boom" })).toBe(
      "✗ test 1.0 s\nboom",
    );
  });

  it("[F19.AC8] builds the docs for a change in docs/, src/, the playground or the build's inputs", () => {
    expect(touchesSite(["docs/adr/README.md"])).toBe(true);
    expect(touchesSite(["scripts/check-all.ts", "src/math/angle/degrees.ts"])).toBe(true);
    expect(touchesSite(["playground/main.ts"])).toBe(true);
    expect(touchesSite(["tools/api-pages/src/main.rs"])).toBe(true);
    expect(touchesSite(["package-lock.json"])).toBe(true);
    expect(touchesSite(["vite.config.ts"])).toBe(true);

    // A code story of the tools: none of the site's inputs.
    expect(touchesSite(["scripts/check-all.ts", "tools/lint/src/lib.rs", "tests/x.test.ts"])).toBe(
      false,
    );

    // A name that only starts like a folder of the site is not in it.
    expect(touchesSite(["docsy.md", "srcmap.ts"])).toBe(false);
    expect(touchesSite([])).toBe(false);
  });

  it("[F19.AC8] skips the docs lane only when the change is known and touches none of its inputs", () => {
    expect(DOCS_LANE).toEqual(["docs:build", "playground:build"]);
    expect(lanesToRun(["tools/lint/src/lib.rs"])).not.toContainEqual(DOCS_LANE);
    expect(lanesToRun(["tools/lint/src/lib.rs"])).toHaveLength(LANES.length - 1);
    expect(lanesToRun(["docs/index.md"])).toEqual(LANES);
    // No base found (main, develop, the CI): every lane.
    expect(lanesToRun(undefined)).toEqual(LANES);
  });

  it("[F19.AC8] compares a story with its feature or develop, a feature with develop, nothing else", () => {
    const remotes = [
      "origin/develop",
      "origin/feature/f19-x",
      "origin/feature/f21-y",
      "origin/main",
    ];

    expect(baseCandidates("chore/en-085-x", remotes)).toEqual([
      "origin/develop",
      "origin/feature/f19-x",
      "origin/feature/f21-y",
    ]);

    expect(baseCandidates("feature/f19-x", remotes)).toEqual(["origin/develop"]);
    expect(baseCandidates("develop", remotes)).toEqual([]);
    expect(baseCandidates("main", remotes)).toEqual([]);
    expect(baseCandidates("HEAD", remotes)).toEqual([]);
  });
});

/**
 * Runs Git in a temporary repository, without the `GIT_*` variables the pre-push hook sets.
 *
 * @param folder - the repository's folder
 * @param options - its arguments
 * @returns its output, trimmed
 */
function gitIn(folder: string, options: readonly string[]): string {
  const environment = Object.fromEntries(
    Object.entries(process.env).filter(([name]) => !name.startsWith("GIT_")),
  );

  return execFileSync(
    "git",
    ["-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false", ...options],
    { cwd: folder, encoding: "utf8", env: environment },
  ).trim();
}

describe("the change check:all reads (AUD-021)", () => {
  it("[F19.AC8] builds the docs for a change to the Rust toolchain or crates", () => {
    expect(touchesSite(["Cargo.lock"])).toBe(true);
    expect(touchesSite(["Cargo.toml"])).toBe(true);
    expect(touchesSite(["rust-toolchain.toml"])).toBe(true);
    // A file named like an input is not that input.
    expect(touchesSite(["package.json.bak"])).toBe(false);
  });

  it("[F19.AC8] counts a page moved out of docs/ under its old path too", () => {
    const folder = mkdtempSync(join(tmpdir(), "check-all-rename-"));

    gitIn(folder, ["init", "-q"]);
    mkdirSync(join(folder, "docs"));
    writeFileSync(join(folder, "docs", "page.md"), "# A page with enough text to be a rename\n");
    gitIn(folder, ["add", "."]);
    gitIn(folder, ["commit", "-q", "-m", "page"]);

    const fork = gitIn(folder, ["rev-parse", "HEAD"]);

    mkdirSync(join(folder, "notes"));
    gitIn(folder, ["mv", "docs/page.md", "notes/page.md"]);
    gitIn(folder, ["commit", "-q", "-m", "move"]);

    const paths = changedSince(folder, fork);

    expect(paths.toSorted(byName)).toEqual(["docs/page.md", "notes/page.md"]);
    expect(touchesSite(paths)).toBe(true);
  });

  it("[F19.AC8] takes the base with the fewest commits ahead, the first on a tie", () => {
    // A story of F19: 5 commits ahead of develop, 2 ahead of its feature branch.
    expect(
      nearestBase([
        { ahead: 5, remote: "origin/develop" },
        { ahead: 2, remote: "origin/feature/f19-x" },
      ]),
    ).toBe("origin/feature/f19-x");

    expect(
      nearestBase([
        { ahead: 3, remote: "origin/develop" },
        { ahead: 3, remote: "origin/feature/f19-x" },
      ]),
    ).toBe("origin/develop");

    expect(nearestBase([])).toBeUndefined();
  });
});

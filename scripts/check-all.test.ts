import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

import {
  BEFORE_LANES,
  DOCS_LANE,
  LANES,
  baseCandidates,
  commandOf,
  lanesToRun,
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
    const written = LANES.map((lane) =>
      lane.flatMap((name) => releaseBinariesOf(commandOf(SCRIPTS.get(name) ?? ""))),
    );
    const shared = written
      .flat()
      .filter((crate) => written.filter((lane) => lane.includes(crate)).length > 1);

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

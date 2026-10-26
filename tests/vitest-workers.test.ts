import { execFileSync } from "node:child_process";
import { join, matchesGlob } from "node:path";

import { describe, expect, it } from "vitest";

import config from "../vite.config";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/** The playground, an npm project of its own whose tests its `ng test` runs (ADR-0104). */
const PLAYGROUND = "playground/";

/** Vitest's configuration, as `vitest run` resolves it. */
const RESOLVED = config({ command: "serve", mode: "test" });

/**
 * Whether a file matches one of the root run's globs.
 *
 * @param file - a path from the repository's root
 * @param include - the globs of the files it runs
 * @returns true when the root's Vitest runs the file
 */
function runs(file: string, include: readonly string[]): boolean {
  return include.some((glob) => matchesGlob(file, glob));
}

describe("Vitest's workers (EN-047)", () => {
  it("[F18.AC20] uses half the machine's cores, so that worktrees testing at once share it", () => {
    expect(RESOLVED.test?.maxWorkers).toBe("50%");
  });

  it("runs each test file in exactly one run, the root's or the playground's, none with a setup file (the rules' tests are Rust's since EN-075)", () => {
    const include = RESOLVED.test?.include ?? [];
    const files = execFileSync("git", ["ls-files", "*.test.ts"], { cwd: ROOT, encoding: "utf8" })
      .split("\n")
      .filter((path) => path !== "");

    // The playground's tests run by its own `ng test` (EN-160, EN-175), never by the root's.
    expect(files.filter((file) => runs(file, include) === file.startsWith(PLAYGROUND))).toEqual([]);
    expect(files.filter((file) => file.startsWith(PLAYGROUND)).length).toBeGreaterThan(0);

    expect(RESOLVED.test?.projects).toBeUndefined();
    expect(RESOLVED.test?.setupFiles).toBeUndefined();
  });
});

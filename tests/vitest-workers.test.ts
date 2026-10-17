import { execFileSync } from "node:child_process";
import { join, matchesGlob } from "node:path";

import { describe, expect, it } from "vitest";

import config from "../vite.config";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/** Vitest's configuration, as `vitest run` resolves it. */
const RESOLVED = config({ command: "serve", mode: "test" });

/**
 * Whether a file matches one of a project's globs.
 *
 * @param file - a path from the repository's root
 * @param include - the project's globs
 * @returns true when Vitest's project runs the file
 */
function runs(file: string, include: readonly string[]): boolean {
  return include.some((glob) => matchesGlob(file, glob));
}

describe("Vitest's workers (EN-047)", () => {
  it("[F18.AC20] uses half the machine's cores, so that worktrees testing at once share it", () => {
    expect(RESOLVED.test?.maxWorkers).toBe("50%");
  });

  it("runs each test file in exactly one project, the RuleTester's setup in the rules' only", () => {
    const projects = (RESOLVED.test?.projects ?? []).flatMap((project) =>
      typeof project === "object" && "test" in project ? [project.test] : [],
    );
    const files = execFileSync("git", ["ls-files", "*.test.ts"], { cwd: ROOT, encoding: "utf8" })
      .split("\n")
      .filter((path) => path !== "");

    expect(
      files.filter(
        (file) => projects.filter((test) => runs(file, test.include ?? [])).length !== 1,
      ),
    ).toEqual([]);

    expect(
      projects.filter((test) => test.setupFiles !== undefined).map((test) => test.name),
    ).toEqual(["rules"]);
  });
});

import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

import { LANES, reportOf } from "./check-all";

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

  it("reports a check by its time, a failed one with its output", () => {
    expect(reportOf({ durationMs: 23_940, isOk: true, name: "lint", output: "x" })).toBe(
      "✓ lint 23.9 s",
    );

    expect(reportOf({ durationMs: 1000, isOk: false, name: "test", output: "boom" })).toBe(
      "✗ test 1.0 s\nboom",
    );
  });
});

import { spawnSync } from "node:child_process";
import { mkdtempSync, readdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/** Time limit: the release build of the tool when it is not built yet, about 30 s (EN-091). */
const BUILD_TIMEOUT = 120_000;

describe("the library's declarations (EN-091)", () => {
  it(
    "are written by Oxc from each module alone: every export carries the types they need",
    { timeout: BUILD_TIMEOUT },
    () => {
      const output = mkdtempSync(join(tmpdir(), "en-091-types-"));
      const result = spawnSync(
        "cargo",
        ["run", "--release", "-q", "-p", "declarations", "--", "src", output],
        { cwd: ROOT, encoding: "utf8" },
      );
      const written = readdirSync(output, { recursive: true }).filter((path) =>
        String(path).endsWith(".d.ts"),
      ).length;

      rmSync(output, { force: true, recursive: true });

      // The tool names each export whose type is neither written nor read from a plain literal.
      expect(result.stderr).toBe("");
      expect(result.status).toBe(0);
      expect(written).toBeGreaterThan(0);
    },
  );
});

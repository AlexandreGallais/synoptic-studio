import { spawnSync } from "node:child_process";
import { mkdtempSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { afterAll, describe, expect, it } from "vitest";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/** A folder of its own: the declarations under `types/`, a consumer beside them. */
const FOLDER = mkdtempSync(join(tmpdir(), "en-091-types-"));

/**
 * What the tool wrote, once for the tests, at import: its release build, when not built yet, takes
 * about 30 s (EN-091), which check:all builds before its lanes.
 */
const WRITTEN = spawnSync(
  "cargo",
  ["run", "--release", "-q", "-p", "declarations", "--", "src", join(FOLDER, "types")],
  { cwd: ROOT, encoding: "utf8" },
);

/**
 * A program using the library through its declarations alone, as a package's user would: each
 * use well typed, and each `@ts-expect-error` above a misuse the declarations must refuse.
 */
const CONSUMER = `import {
  BISECTION_STEPS,
  bisection,
  degreesToRadians,
  directionAt,
  formatDataUrl,
} from "./types/index";

export const steps: 60 = BISECTION_STEPS;
export const half: number = degreesToRadians(180);
export const root: number = bisection((value) => value < 2, 0, 4);
export const right: number = directionAt(0).x;
export const url: string = formatDataUrl("font/ttf", new Uint8Array([1]));

// @ts-expect-error a number is not a string
export const wrong: string = degreesToRadians(180);
// @ts-expect-error the bytes are a Uint8Array
export const text = formatDataUrl("font/ttf", "bytes");
// @ts-expect-error a vector's components are read only
directionAt(0).x = 1;
`;

afterAll(() => {
  rmSync(FOLDER, { force: true, recursive: true });
});

describe("the library's declarations (EN-091)", () => {
  it("[F19.AC6] are written by Oxc from each module alone: every export carries the types they need", () => {
    // The tool names each export whose type is neither written nor read from a plain literal.
    expect(WRITTEN.stderr).toBe("");
    expect(WRITTEN.status).toBe(0);

    expect(
      readdirSync(join(FOLDER, "types"), { recursive: true }).filter((path) =>
        String(path).endsWith(".d.ts"),
      ).length,
    ).toBeGreaterThan(0);
  });

  it("[F19.AC6] mean the library's types: a consumer type-checks, its misuses refused", () => {
    writeFileSync(join(FOLDER, "consumer.ts"), CONSUMER);

    writeFileSync(
      join(FOLDER, "tsconfig.json"),
      JSON.stringify({
        compilerOptions: {
          module: "preserve",
          moduleResolution: "bundler",
          noEmit: true,
          strict: true,
          types: [],
        },
        files: ["consumer.ts"],
      }),
    );

    const result = spawnSync(join(ROOT, "node_modules", ".bin", "tsc"), ["-p", FOLDER], {
      encoding: "utf8",
    });

    // An unused `@ts-expect-error` (TS2578) would mean a declaration lost its type.
    expect(result.stdout).toBe("");
    expect(result.status).toBe(0);
  });
});

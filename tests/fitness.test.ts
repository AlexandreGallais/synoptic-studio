import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Folders whose functions need a property test (ADR-0022). */
const PROPERTY_LAYERS = ["src/math", "src/geometry"];

/** Repository root. */
const ROOT = join(import.meta.dirname, "..");

/** A property in a test file: `test.prop(` or `it.prop(`. */
const PROPERTY = /\b(?:test|it)\.prop\(/u;

/** An exported function of the layers checked, with the test file beside it. */
type CheckedFunction = {
  readonly name: string;
  readonly test: string;
};

/**
 * Exported functions of a folder, each with the path of its own test file.
 *
 * @param folder - folder relative to the root
 * @returns one entry per `export function`, `index.ts` and tests left out
 */
function exportedFunctions(folder: string): readonly CheckedFunction[] {
  return readdirSync(join(ROOT, folder), { recursive: true })
    .map(String)
    .filter(
      (file) => file.endsWith(".ts") && !file.endsWith("index.ts") && !file.endsWith(".test.ts"),
    )
    .flatMap((file) =>
      readFileSync(join(ROOT, folder, file), "utf8")
        .matchAll(/^export function (?<name>\w+)/gmu)
        .map((match) => ({
          name: match.groups?.["name"] ?? "",
          test: join(ROOT, folder, file.replace(/\.ts$/u, ".test.ts")),
        }))
        .toArray(),
    );
}

/**
 * Whether a function's own test file holds a property (ADR-0022): a property written for another
 * function, even one calling it, does not count.
 *
 * @param checked - the function and its test file
 * @returns true when the test file exists and holds `test.prop` or `it.prop`
 */
function hasOwnProperty(checked: CheckedFunction): boolean {
  // A property commented out does not count.
  return (
    existsSync(checked.test) &&
    PROPERTY.test(readFileSync(checked.test, "utf8").replaceAll(/\/\/.*$/gmu, ""))
  );
}

/** Functions checked: every exported function of the layers. */
const CHECKED = PROPERTY_LAYERS.flatMap((layer) => exportedFunctions(layer));

/** Functions of the two layers at EN-020: the check may never read fewer (an empty list passes). */
const FUNCTIONS_AT_EN_020 = 41;

describe("fitness functions", () => {
  it("reads every function of the layers, not an empty list", () => {
    expect(CHECKED.length).toBeGreaterThanOrEqual(FUNCTIONS_AT_EN_020);

    expect(CHECKED.map((checked) => checked.name)).toEqual(
      expect.arrayContaining(["add", "mirrorAcrossAxis", "turningAngles"]),
    );
  });

  it("gives every math and geometry function a property test in its own test file", () => {
    expect(
      CHECKED.filter((checked) => !hasOwnProperty(checked)).map((checked) => checked.name),
    ).toEqual([]);
  });
});

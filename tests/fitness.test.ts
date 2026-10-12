import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Folders whose functions need a property test (ADR-0022). */
const PROPERTY_LAYERS = ["src/math", "src/geometry"];

/** Repository root. */
const ROOT = join(import.meta.dirname, "..");

/**
 * Functions written before this check and still without a property test: each one goes once its
 * property is written (story EN-020). The list may only shrink.
 */
const WITHOUT_PROPERTY_YET = new Set([
  "alignmentFactor",
  "boxAspect",
  "cornerSetbacks",
  "edgeFactors",
  "hasLength",
  "isSoundStretch",
  "mirrorAcrossAxis",
  "stretchedRightHalf",
  "stretchedSideDirections",
]);

/**
 * Text of the TypeScript files of a folder, tests or not.
 *
 * @param folder - folder relative to the root
 * @param isTest - true for test files, false for the others
 * @returns the text of each file, `index.ts` left out
 */
function sources(folder: string, isTest: boolean): readonly string[] {
  return readdirSync(join(ROOT, folder), { recursive: true })
    .map(String)
    .filter(
      (file) =>
        file.endsWith(".ts") && !file.endsWith("index.ts") && file.endsWith(".test.ts") === isTest,
    )
    .map((file) => readFileSync(join(ROOT, folder, file), "utf8"));
}

/** Text of the test files of `src/` that hold a property (`test.prop` or `it.prop`). */
const PROPERTY_TESTS = sources("src", true)
  .filter((text) => /\b(?:test|it)\.prop\(/u.test(text))
  .join("\n");

/** Names of the functions exported by the layers checked. */
const FUNCTIONS = PROPERTY_LAYERS.flatMap((layer) =>
  sources(layer, false).flatMap((text) =>
    text
      .matchAll(/^export function (?<name>\w+)/gmu)
      .map((match) => match.groups?.["name"] ?? "")
      .toArray(),
  ),
);

/**
 * Whether a function is named in a test file holding a property.
 *
 * @param name - function name
 * @returns true when a property test file names it
 */
function hasProperty(name: string): boolean {
  const pattern = new RegExp(String.raw`\b${name}\b`, "u");

  return pattern.test(PROPERTY_TESTS);
}

describe("fitness functions", () => {
  it("gives every math and geometry function a property test, but the listed ones", () => {
    expect(
      FUNCTIONS.filter((name) => !hasProperty(name) && !WITHOUT_PROPERTY_YET.has(name)),
    ).toEqual([]);
  });

  it("lists only functions that still lack a property test", () => {
    expect(
      [...WITHOUT_PROPERTY_YET].filter((name) => hasProperty(name) || !FUNCTIONS.includes(name)),
    ).toEqual([]);
  });
});

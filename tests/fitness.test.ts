import { existsSync, readFileSync, readdirSync } from "node:fs";
import { basename, join } from "node:path";

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
      (file) => file.endsWith(".ts") && basename(file) !== "index.ts" && !file.endsWith(".test.ts"),
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

  it("[F11.AC5] gives every math and geometry function a property test in its own test file", () => {
    expect(
      CHECKED.filter((checked) => !hasOwnProperty(checked)).map((checked) => checked.name),
    ).toEqual([]);
  });
});

/** Exported names of a module, read from its text. */
const EXPORTED = /^export (?:async )?(?:function|const) (?<name>\w+)/gmu;

/**
 * Names exported by the modules of a folder, read from their text.
 *
 * @param folder - path from the repository root
 * @returns each exported function or constant name
 */
function exportedNames(folder: string): readonly string[] {
  return readdirSync(join(ROOT, folder))
    .filter((name) => name.endsWith(".ts") && !name.endsWith(".test.ts"))
    .flatMap((name) =>
      readFileSync(join(ROOT, folder, name), "utf8")
        .matchAll(EXPORTED)
        .map((match) => match.groups?.["name"] ?? "")
        .toArray(),
    );
}

/** Names the shared test modules export: a test file never declares them again (F11). */
const SHARED_HELPERS = new Set(
  ["playground/user-actions", "tests"].flatMap((folder) => exportedNames(folder)),
);

/**
 * Text of every file of `src/` and `playground/` whose name ends as asked.
 *
 * @param ending - end of the file name, such as `.test.ts`
 * @returns path from the repository root, and text, of each file
 */
function sourcesEndingWith(
  ending: string,
): readonly { readonly path: string; readonly text: string }[] {
  return ["src", "playground"].flatMap((root) =>
    readdirSync(join(ROOT, root), { recursive: true })
      .map(String)
      .filter((name) => name.endsWith(ending))
      .map((name) => ({
        path: join(root, name),
        text: readFileSync(join(ROOT, root, name), "utf8"),
      })),
  );
}

/**
 * Number of times a text holds a piece.
 *
 * @param text - text to search
 * @param piece - exact piece of text
 * @returns how many times it appears
 */
function occurrences(text: string, piece: string): number {
  return text.split(piece).length - 1;
}

describe("a code base ready for curves (F11)", () => {
  it("[F11.AC1] declares each shared test helper once, never again in a test file", () => {
    expect(SHARED_HELPERS).toContain("typeValue");
    expect(SHARED_HELPERS).toContain("rectanglePoints");

    expect(
      sourcesEndingWith(".test.ts").flatMap(({ path, text }) =>
        text
          .matchAll(/^(?:function|const) (?<name>\w+)/gmu)
          .map((match) => match.groups?.["name"] ?? "")
          .filter((name) => SHARED_HELPERS.has(name))
          .map((name) => `${path}: ${name}`)
          .toArray(),
      ),
    ).toEqual([]);
  });

  it("[F11.AC2] exports one writer of path data, its separator and close command declared once", () => {
    const modules = sourcesEndingWith(".ts")
      .filter(({ path }) => path.startsWith("src") && !path.endsWith(".test.ts"))
      .map(({ text }) => text);
    const writers = modules.filter((text) => /^export function \w+ToPathData\(/mu.test(text));

    expect(writers).toHaveLength(1);
    expect(writers[0]).toContain("export function contourPiecesToPathData(");
    expect(modules.filter((text) => text.includes("const SEPARATOR ="))).toHaveLength(1);
    expect(modules.filter((text) => text.includes("const CLOSE_PATH ="))).toHaveLength(1);
  });

  it("[F11.AC3] computes each corner's triple and turning angle once for a rounded contour", () => {
    const text = readFileSync(join(ROOT, "src/geometry/corner-radius/rounded-contour.ts"), "utf8");

    expect(occurrences(text, "cornerTriples(")).toBe(1);
    expect(occurrences(text, "turningAngles(")).toBe(1);
    expect(occurrences(text, "turningAngle(")).toBe(0);
  });
});

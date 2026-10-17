import { readFileSync, readdirSync } from "node:fs";
import { basename, join } from "node:path";

import { describe, expect, it } from "vitest";

import { builtOnce } from "./built-once";

/** Repository root. */
const ROOT = join(import.meta.dirname, "..");

/** Folders whose code is read. */
const CODE_FOLDERS = ["src", "playground"];

/** A source file: its name and its lines of code, comments left out. */
type SourceFile = {
  readonly code: readonly string[];
  readonly name: string;
};

/**
 * Every source file of the code folders, tests and barrels left out.
 *
 * @returns each file's name and its lines that are not comments
 */
function sourceFiles(): readonly SourceFile[] {
  return CODE_FOLDERS.flatMap((folder) =>
    readdirSync(join(ROOT, folder), { recursive: true })
      .map(String)
      .filter(
        (file) =>
          file.endsWith(".ts") && !file.endsWith(".test.ts") && basename(file) !== "index.ts",
      )
      .map((file) => ({
        code: readFileSync(join(ROOT, folder, file), "utf8")
          .split("\n")
          .filter((line) => !/^\s*(?:\*|\/\/|\/\*)/u.test(line)),
        name: basename(file),
      })),
  );
}

/** The source files, read at the first search and shared, frozen, by every row (EN-090). */
const readSources: () => readonly SourceFile[] = builtOnce(() => Object.freeze(sourceFiles()));

/**
 * Names of the files whose code matches a pattern.
 *
 * @param pattern - what the code is searched for
 * @returns the file names, sorted
 */
function filesWith(pattern: RegExp): readonly string[] {
  return readSources()
    .filter((file) => file.code.some((line) => pattern.test(line)))
    .map((file) => file.name)
    .toSorted((left, right) => left.localeCompare(right));
}

describe("every calculation named (F13, ADR-0049)", () => {
  it.each([
    {
      files: ["polar-angle.ts", "signed-angle.ts"],
      name: "an angle from atan2",
      pattern: /Math\.atan2\(/u,
    },
    { files: ["norm.ts"], name: "a length from hypot", pattern: /Math\.hypot\(/u },
    { files: ["modulo.ts"], name: "the modulo idiom", pattern: /\(\w+ % \w+\) \+ \w+\) %/u },
  ])("[F13.AC1] writes $name only in its named function", ({ files, pattern }) => {
    expect(filesWith(pattern)).toEqual(files);
  });

  it.each([
    { files: ["no-handles.ts"], name: "NO_HANDLES", pattern: /const NO_HANDLES\b/u },
    { files: ["no-curve-corner.ts"], name: "NO_CURVE_CORNER", pattern: /const NO_CURVE_CORNER\b/u },
    { files: ["is-round-shape.ts"], name: "the round shape's test", pattern: /"kind" in /u },
    { files: ["is-regular-polygon.ts"], name: "the polygon's test", pattern: /"anchor" in /u },
  ])("[F13.AC2] declares $name once, the duplicates merged", ({ files, pattern }) => {
    expect(filesWith(pattern)).toEqual(files);
  });
});

describe("every calculation named, E15's naming pass (NAM-001, ADR-0049)", () => {
  it.each([
    {
      files: ["integer-quotient.ts", "progression-terms-up-to.ts"],
      name: "a quotient rounded down",
      pattern: /Math\.floor\(/u,
    },
    {
      files: ["bit-length.ts"],
      name: "a length in binary digits",
      pattern: /\.toString\((?:2|BINARY)\)/u,
    },
    {
      files: ["byte-to-fraction.ts"],
      name: "a byte as a fraction",
      pattern: /\/ (?:255|BYTE_MAX\w*)\b/u,
    },
  ])("writes $name only in its named function", ({ files, pattern }) => {
    expect(filesWith(pattern)).toEqual(files);
  });

  it.each([
    {
      files: ["byte-maximum.ts"],
      name: "the largest byte",
      pattern: /\b(?:const|let) \w+ = 255;/u,
    },
    {
      files: ["percent.ts"],
      name: "a whole in percent",
      pattern: /\b(?:const|let) (?:PERCENT|WHOLE|\w*_PERCENT) = 100;/u,
    },
  ])("declares $name once, the duplicates merged", ({ files, pattern }) => {
    expect(filesWith(pattern)).toEqual(files);
  });
});

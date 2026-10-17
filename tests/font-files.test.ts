import { execFileSync } from "node:child_process";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

import { interRegularFile, partsDigest } from "./font-fixtures";
import { sharedSansTables } from "./inter-face-fixtures";

/** Repository root. */
const ROOT = join(import.meta.dirname, "..");

/** The only file that reads a test font file: Vitest's global setup. */
const READER = "tests/font-files-setup.ts";

/** The extensions of a font file or a collection. */
const FONT_FILE = /\.(?:otf|ttc|ttf|woff2?)$/u;

/**
 * The files of the repository's TypeScript, tracked or new, that name one of the test font files.
 *
 * @param fonts - the names of the test font files
 * @returns each file that names one, by its path from the root
 */
function filesNaming(fonts: readonly string[]): string[] {
  const files = execFileSync(
    "git",
    ["ls-files", "--cached", "--others", "--exclude-standard", "*.ts"],
    { cwd: ROOT, encoding: "utf8" },
  )
    .split("\n")
    .filter((file) => file !== "");

  return files.filter((file) => {
    const text = readFileSync(join(ROOT, file), "utf8");

    return fonts.some((font) => text.includes(font));
  });
}

describe("the test font files, read once per run (EN-089)", () => {
  it("are named by the global setup alone: no test reads a font file itself", () => {
    const fonts = readdirSync(import.meta.dirname).filter((file) => FONT_FILE.test(file));

    // One font file in tests/, Inter Regular (REF-INTER-REGULAR-FILE): its name is left out here.
    expect(fonts).toHaveLength(1);
    expect(filesNaming(fonts)).toEqual([READER]);
  });

  it("reads Inter Regular 4.1 whole: 411 640 bytes, its SHA-256 that of the release", () => {
    // REF-INTER-REGULAR-FILE: 411 640 bytes, SHA-256 40d692fc…be0c82, read with shasum -a 256.
    const file = interRegularFile();

    expect(file.length).toBe(411_640);

    expect(partsDigest([file])).toBe(
      "40d692fce188e4471e2b3cba937be967878f631ad3ebbbdcd587687c7ebe0c82",
    );
  });

  it("gives a fresh copy at each call: a test that writes into it leaves the next one whole", () => {
    const written = interRegularFile();

    written.fill(0);

    expect(interRegularFile()[0]).toBe(0);
    expect(interRegularFile()[1]).toBe(1);
  });

  it("gives Inter's tables frozen: a test cannot change them for the next", () => {
    const [regular] = sharedSansTables();
    const glyf = regular?.["glyf"];

    expect(Object.isFrozen(regular)).toBe(true);
    expect(Array.isArray(glyf) && Object.isFrozen(glyf)).toBe(true);
    expect(sharedSansTables()[0]).toBe(regular);
  });
});

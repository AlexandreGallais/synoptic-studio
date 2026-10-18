// Writes the metrics of the default font, Inter 4.1 (EN-031, ADR-0052), to
// `src/model/font/inter-metrics.json`, read by the library, the playground and the tests without
// the font files. Inter's static TrueType files are not kept in the repository: download the
// release `Inter-4.1.zip` (`REF-INTER-RELEASE`), unpack its `extras/ttf/` folder, then run
//
//   node scripts/default-font-metrics.ts <folder holding Inter-Regular.ttf and the other faces>
//
// The library's own reader (`parseFontMetrics`) reads each face, loaded through Vite since the
// library's imports name no file extension. The data is a Modified Version of the font under the
// SIL Open Font License, shipped with `inter-license.txt` beside it (`REF-INTER-LICENSE`).

import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { runnerImport } from "vite";

import { MEASURED_BLOCKS } from "../src/model/font/measured-blocks.ts";

import type { FontFace, FontFamily, blockCharacters, parseFontMetrics } from "../src/index.ts";

/** What the script uses of the library, loaded through Vite. */
type Library = {
  readonly blockCharacters: typeof blockCharacters;
  readonly parseFontMetrics: typeof parseFontMetrics;
};

/** Where the data file is written. */
export const DEFAULT_FONT_DATA = join(
  import.meta.dirname,
  "..",
  "src",
  "model",
  "font",
  "inter-metrics.json",
);

/** The release the metrics are read from (`REF-INTER-RELEASE`). */
const INTER = { name: "Inter", version: "4.1" };

/** Inter's static weights and the word naming each in its file names. */
export const INTER_WEIGHTS: readonly { readonly weight: number; readonly word: string }[] = [
  { weight: 100, word: "Thin" },
  { weight: 200, word: "ExtraLight" },
  { weight: 300, word: "Light" },
  { weight: 400, word: "Regular" },
  { weight: 500, word: "Medium" },
  { weight: 600, word: "SemiBold" },
  { weight: 700, word: "Bold" },
  { weight: 800, word: "ExtraBold" },
  { weight: 900, word: "Black" },
];

/** One face of Inter and its file. */
export type InterFace = {
  readonly file: string;
  readonly isItalic: boolean;
  readonly weight: number;
};

/**
 * The file name of a face of Inter 4.1: `Inter-Bold.ttf`, `Inter-BoldItalic.ttf`; the regular
 * italic face is `Inter-Italic.ttf`.
 *
 * @param word - the word naming the weight
 * @param isItalic - true for the italic face, false for the upright one
 * @returns the name the release gives the file
 */
export function interFileName(word: string, isItalic: boolean): string {
  const style = isItalic ? `${word}Italic`.replace("RegularItalic", "Italic") : word;

  return `Inter-${style}.ttf`;
}

/**
 * Every face of Inter 4.1 that ships as a static file: nine weights, upright and italic.
 *
 * @returns each face and its file, upright faces first, by weight
 */
export function interFaces(): readonly InterFace[] {
  return [false, true].flatMap((isItalic) =>
    INTER_WEIGHTS.map(({ weight, word }) => ({
      file: interFileName(word, isItalic),
      isItalic,
      weight,
    })),
  );
}

/**
 * The data file's text: the family as compact JSON, one line.
 *
 * @param family - the family and its faces' metrics
 * @returns the file's text
 */
export function familyJson(family: FontFamily): string {
  return `${JSON.stringify(family)}\n`;
}

if (import.meta.main) {
  const [folder = "."] = process.argv.slice(2);
  const { module: library } = await runnerImport<Library>(
    join(import.meta.dirname, "..", "src", "index.ts"),
    { configFile: false, logLevel: "error" },
  );
  const characters = library.blockCharacters(MEASURED_BLOCKS);
  const faces = interFaces().map((face): FontFace => {
    const metrics = library.parseFontMetrics(readFileSync(join(folder, face.file)), characters);

    if (metrics === undefined) {
      throw new Error(`${face.file} is not a TrueType or OpenType font`);
    }

    // The files are not shipped: no face holds one, and JSON leaves the undefined field out.
    return { file: undefined, isItalic: face.isItalic, metrics, weight: face.weight };
  });
  const text = familyJson({ ...INTER, faces });

  writeFileSync(DEFAULT_FONT_DATA, text);

  process.stdout.write(
    `${DEFAULT_FONT_DATA}: ${String(faces.length)} faces, ${String(text.length)} bytes\n`,
  );
}

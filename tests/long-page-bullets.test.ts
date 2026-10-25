import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Repository root. */
const ROOT = join(import.meta.dirname, "..");

/** Longest bullet, in characters: a longer one is split or turned into a table (EN-060). */
const BULLET_LENGTH = 400;

/** A page passed by EN-060: its file, and the heading its part starts at and the one it ends before. */
type PassedPart = {
  readonly file: string;
  readonly from: string;
  readonly to: string;
};

/** The parts of E15's pages that EN-060 passed; an empty heading means the page's start or end. */
const PASSED_PARTS: readonly PassedPart[] = [
  { file: "docs/derivations/text-box.md", from: "", to: "" },
  { file: "docs/derivations/color-spaces.md", from: "", to: "" },
  { file: "docs/research/notes/0017-e15-text.md", from: "", to: "" },
  { file: "docs/research/notes/0018-f14-text-box.md", from: "", to: "" },
];

/**
 * The lines of a part, from its first heading to the one it ends before.
 *
 * @param part - the page and the headings bounding it
 * @returns its Markdown, one string per line, the closing heading left out
 */
function partLines(part: PassedPart): readonly string[] {
  const lines = readFileSync(join(ROOT, part.file), "utf8").split("\n");
  const start = part.from === "" ? 0 : lines.indexOf(part.from);
  const end = part.to === "" ? lines.length : lines.indexOf(part.to);

  expect(start, `${part.file}: ${part.from}`).toBeGreaterThanOrEqual(0);
  expect(end, `${part.file}: ${part.to}`).toBeGreaterThan(start);

  return lines.slice(start, end);
}

describe("bullets of E15's long pages (EN-060)", () => {
  it.each(PASSED_PARTS)("[F18.AC17] keeps every bullet of $file within 400 characters", (part) => {
    const longBullets = partLines(part).filter(
      (line) => /^\s*(?:[-*]|\d+\.) /u.test(line) && line.length > BULLET_LENGTH,
    );

    expect(longBullets).toStrictEqual([]);
  });
});

// Chunks and freshness of the local source library (docs/tooling/sources.md): a page is split at
// its headings, so that a search returns a section and a reading costs only that section; the
// freshness rule says when a stored page is read again online.

/** One section of a page. */
export type Chunk = {
  /** Heading path of the section, e.g. `Hooks › PostToolUse › Input`. */
  readonly heading: string;
  /** Its position in the page, from 0. */
  readonly index: number;
  /** Its Markdown text, heading line included. */
  readonly text: string;
};

/** What the library knows of a stored page's age. */
export type PageAge = {
  /** When the library last read it online, epoch milliseconds. */
  readonly fetchedAt: number;
  /** When the page says it was last changed, epoch milliseconds, or 0 when it does not say. */
  readonly modifiedAt: number;
  /** Time now, epoch milliseconds. */
  readonly now: number;
};

/** Sections read so far, and whether the line being read is inside a code block. */
type Sections = {
  readonly isFence: boolean;
  readonly sections: readonly Section[];
};

/** A section being read: its heading path and its lines. */
type Section = {
  readonly heading: readonly string[];
  readonly lines: readonly string[];
};

/** The section of the text before the first heading, empty until a line is read. */
const NO_SECTION: Section = { heading: [], lines: [] };

/** One day, in milliseconds. */
export const DAY_MS = 86_400_000;

/** A page changed less than this before it was read is young: it may still be corrected. */
const YOUNG_DAYS = 14;

/** A young page is read again after this many days. */
const YOUNG_REFRESH_DAYS = 1;

/** Any page is read again after this many days. */
const STABLE_REFRESH_DAYS = 90;

/** Longest section kept in one chunk, in characters; a longer one is cut at blank lines. */
const MAX_CHUNK = 4000;

/**
 * Level of a Markdown heading line.
 *
 * @param line - a Markdown line
 * @returns 1 to 6 for `#` to `######` followed by a space, 0 for any other line
 */
function headingLevel(line: string): number {
  return /^#{1,6} /u.exec(line)?.[0].trimEnd().length ?? 0;
}

/**
 * Parts of a section no longer than `MAX_CHUNK`, cut between paragraphs.
 *
 * @param text - Markdown of a section
 * @returns one or more parts
 */
function parts(text: string): readonly string[] {
  return text.split("\n\n").reduce<readonly string[]>((done, paragraph) => {
    const last = done.at(-1);

    return last !== undefined && last.length + paragraph.length < MAX_CHUNK
      ? [...done.slice(0, -1), `${last}\n\n${paragraph}`]
      : [...done, paragraph];
  }, []);
}

/**
 * A section opened by a line.
 *
 * @param path - heading path of the section before
 * @param line - the line opening it
 * @param level - its heading level, 1 to 6
 * @returns the section, its heading path cut at the level and ended by the line's title
 */
function openedSection(path: readonly string[], line: string, level: number): Section {
  return {
    heading: [...path.slice(0, level - 1), line.slice(level + 1)],
    lines: [line],
  };
}

/**
 * Sections once one more line is read: a heading outside a code block opens a section, any other
 * line goes to the last one.
 *
 * @param done - sections so far, and whether the line before was inside a code block
 * @param line - the next Markdown line
 * @returns the sections with that line
 */
function readLine(done: Sections, line: string): Sections {
  const last = done.sections.at(-1) ?? NO_SECTION;
  const level = done.isFence ? 0 : headingLevel(line);
  const isFence = line.startsWith("```") !== done.isFence;

  if (level > 0) {
    return { isFence, sections: [...done.sections, openedSection(last.heading, line, level)] };
  }

  return {
    isFence,
    sections: [
      ...done.sections.slice(0, -1),
      { heading: last.heading, lines: [...last.lines, line] },
    ],
  };
}

/**
 * A page split into sections at its headings.
 *
 * @param markdown - the page as Markdown
 * @returns its chunks in page order, each with its heading path; text before the first heading
 * is a chunk of empty heading
 */
export function pageChunks(markdown: string): readonly Chunk[] {
  const { sections } = markdown
    .split("\n")
    .reduce<Sections>((done, line) => readLine(done, line), { isFence: false, sections: [] });

  return sections
    .flatMap((section) =>
      parts(section.lines.join("\n").trim()).map((text) => ({
        heading: section.heading.join(" › "),
        text,
      })),
    )
    .filter((chunk) => chunk.text !== "")
    .map((chunk, index) => ({ ...chunk, index }));
}

/**
 * Whether a stored page must be read again online before use.
 *
 * A page changed shortly before it was read may still be corrected: it is read again after a day.
 * Any other page is read again after 90 days.
 *
 * @param age - when it was read, when it says it changed, and now
 * @returns true when it is stale
 */
export function isStale(age: PageAge): boolean {
  const isYoung = age.modifiedAt > 0 && age.fetchedAt - age.modifiedAt < YOUNG_DAYS * DAY_MS;
  const limit = (isYoung ? YOUNG_REFRESH_DAYS : STABLE_REFRESH_DAYS) * DAY_MS;

  return age.now - age.fetchedAt > limit;
}

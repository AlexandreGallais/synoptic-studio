import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { afterEach, describe, expect, it } from "vitest";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/** The append-only logs merged by union (ADR-0065, ADR-0085), from the root. */
const LOGS = [
  "docs/process/journal.md",
  "docs/process/improvements.md",
  "docs/process/feedback.md",
  "docs/process/tool-incidents.md",
];

/** A date as the logs write it, at the start of a cell or a heading. */
const DATE = /^\d{4}-\d{2}-\d{2}\b/u;

/** Temporary folders to remove after each test. */
const FOLDERS: string[] = [];

/**
 * Whether a line is a table's separator, such as `| --- | :-: |`.
 *
 * @param line - a line of a log
 * @returns true for a separator
 */
function isSeparator(line: string): boolean {
  return /^\|(?:\s*:?-+:?\s*\|)+\s*$/u.test(line);
}

/**
 * The rows of a log a union merge may repeat: its bullets and its tables' body rows, a header and
 * its separator left out.
 *
 * @param text - the log's content
 * @returns its rows, in order
 */
function rows(text: string): readonly string[] {
  const lines = text.split("\n");

  return lines.filter(
    (line, index) =>
      line.startsWith("- ") ||
      (line.startsWith("|") && !isSeparator(line) && !isSeparator(lines[index + 1] ?? "")),
  );
}

/**
 * The rows written more than once, each named once, compared without their padding: a table
 * repeated with other column widths is still repeated.
 *
 * @param text - the log's content
 * @returns the repeated rows, their spaces collapsed, in order of their first occurrence
 */
function repeatedRows(text: string): readonly string[] {
  const all = rows(text).map((row) => row.replaceAll(/\s+/gu, " "));

  return [...new Set(all.filter((row, index) => all.indexOf(row) !== index))];
}

/**
 * The tool cards written more than once, heading and bullets, each named once by its heading: two
 * cards may share a bullet (`- Instead: none`), never a whole card (AUD-025).
 *
 * @param text - the incidents log's content
 * @returns the headings of the repeated cards, in order of their first occurrence
 */
function repeatedCards(text: string): readonly string[] {
  return repeatedRows(text).filter((row) => row.startsWith("## "));
}

/** Each log's check of its repeats: the incidents log compares whole cards, the others rows. */
const REPEATED: Readonly<Record<string, (text: string) => readonly string[]>> = {
  "docs/process/feedback.md": repeatedRows,
  "docs/process/improvements.md": repeatedRows,
  "docs/process/journal.md": repeatedRows,
  "docs/process/tool-incidents.md": repeatedCards,
};

/**
 * The journal's session headings that do not start with a date.
 *
 * @param text - the journal's content
 * @returns the undated headings
 */
function undatedSessions(text: string): readonly string[] {
  return text.split("\n").filter((line) => line.startsWith("## ") && !DATE.test(line.slice(3)));
}

/**
 * The table rows whose first cell is not a date.
 *
 * @param text - a log's content
 * @returns the rows that do not start with a date, in order
 */
function undatedRows(text: string): readonly string[] {
  return rows(text).filter((row) => row.startsWith("|") && !DATE.test(row.slice(1).trim()));
}

/**
 * The lines of a table log that break its one table: from its first table line to the end, any
 * line that is not a row (a bullet, a paragraph, a blank line ending the table), and any header
 * after the first.
 *
 * @param text - a log's content
 * @returns the stray lines, in order
 */
function strayTableLines(text: string): readonly string[] {
  const lines = text.replace(/\n$/u, "").split("\n");
  const first = lines.findIndex((line) => line.startsWith("|"));
  const table = first === -1 ? [] : lines.slice(first);
  const headers = table.filter((line, index) => isSeparator(table[index + 1] ?? ""));

  return [...table.filter((line) => !line.startsWith("|")), ...headers.slice(1)];
}

/** A journal bullet opening with its tag, `` - `#run` … ``. */
const TAGGED = /^- `#[a-z-]+` /u;

/**
 * The journal's bullets without their tag, which `grep` finds them by.
 *
 * @param text - the journal's content
 * @returns the untagged bullets, in order
 */
function untaggedBullets(text: string): readonly string[] {
  return text.split("\n").filter((line) => line.startsWith("- ") && !TAGGED.test(line));
}

/**
 * The parts of a tool card, each found by the label opening one of its bullets, with an optional
 * parenthesis before its colon (`- Used (found by AUD-024):`).
 */
const CARD_PARTS: readonly { readonly label: RegExp; readonly part: string }[] = [
  { label: /^- (?:Used|Seen)(?: \([^)]*\))?:/u, part: "the use" },
  { label: /^- (?:Failed|Fails|Seen)(?: \([^)]*\))?:/u, part: "the failure" },
  { label: /^- Instead(?: \([^)]*\))?:/u, part: "the workaround" },
];

/**
 * The tool cards that lack a part (ADR-0085): each card, under its `## ` heading, states its use
 * (`- Used`) and its failure (`- Failed`, `- Fails`), or what was seen, both at once (`- Seen`),
 * and what was done instead (`- Instead`).
 *
 * @param text - the incidents log's content
 * @returns each incomplete card's heading and the parts it lacks, in order
 */
function incompleteCards(text: string): readonly string[] {
  const cards = text.split(/^(?=## )/mu).filter((card) => card.startsWith("## "));

  return cards
    .map((card) => {
      const lines = card.split("\n");
      const lacks = CARD_PARTS.filter(({ label }) => lines.every((line) => !label.test(line)));

      return { heading: lines[0] ?? "", lacks: lacks.map(({ part }) => part) };
    })
    .filter(({ lacks }) => lacks.length > 0)
    .map(({ heading, lacks }) => `${heading}: lacks ${lacks.join(", ")}`);
}

/**
 * Each log's check of its shape: the two table logs hold one table, the journal tagged bullets,
 * the incidents log whole cards.
 */
const STRAY: Readonly<Record<string, (text: string) => readonly string[]>> = {
  "docs/process/feedback.md": strayTableLines,
  "docs/process/improvements.md": strayTableLines,
  "docs/process/journal.md": untaggedBullets,
  "docs/process/tool-incidents.md": incompleteCards,
};

/**
 * Each log's check of its dates: the journal dates its sessions, the incidents log its cards, the
 * two other logs their rows.
 */
const UNDATED: Readonly<Record<string, (text: string) => readonly string[]>> = {
  "docs/process/feedback.md": undatedRows,
  "docs/process/improvements.md": undatedRows,
  "docs/process/journal.md": undatedSessions,
  "docs/process/tool-incidents.md": undatedSessions,
};

/**
 * A log's content.
 *
 * @param log - the log's path from the root
 * @returns its text
 */
function logText(log: string): string {
  return readFileSync(join(ROOT, log), "utf8");
}

/**
 * The environment without Git's variables, which a hook running the tests sets (EN-050).
 *
 * @returns the variables of this process, Git's left out
 */
function cleanEnvironment(): NodeJS.ProcessEnv {
  return Object.fromEntries(
    Object.entries(process.env).filter(([name]) => !name.startsWith("GIT_")),
  );
}

/**
 * Runs Git in a folder.
 *
 * @param folder - the repository
 * @param parameters - Git's arguments
 * @returns what Git printed
 */
function git(folder: string, parameters: readonly string[]): string {
  return execFileSync(
    "git",
    ["-c", "user.name=test", "-c", "user.email=test@example.com", ...parameters],
    { cwd: folder, encoding: "utf8", env: cleanEnvironment() },
  );
}

/**
 * Merges two branches that each add a row to the improvement log, in a new repository holding this
 * repository's `.gitattributes`.
 *
 * @returns the log after the merge
 */
function unionMerge(): string {
  const folder = mkdtempSync(join(tmpdir(), "logs-"));
  const log = join(folder, "improvements.md");
  const header = "| Date | Item |\n| --- | --- |\n| 2026-10-16 | first |\n";

  FOLDERS.push(folder);
  git(folder, ["init", "-q", "-b", "main"]);

  writeFileSync(
    join(folder, ".gitattributes"),
    readFileSync(join(ROOT, ".gitattributes"), "utf8").replaceAll("docs/process/", ""),
  );

  writeFileSync(log, header);
  git(folder, ["add", "."]);
  git(folder, ["commit", "-q", "-m", "start"]);
  git(folder, ["switch", "-q", "-c", "story"]);
  writeFileSync(log, `${header}| 2026-10-17 | from the story |\n`);
  git(folder, ["commit", "-q", "-a", "-m", "story"]);
  git(folder, ["switch", "-q", "main"]);
  writeFileSync(log, `${header}| 2026-10-17 | from main |\n`);
  git(folder, ["commit", "-q", "-a", "-m", "main"]);
  git(folder, ["merge", "-q", "--no-edit", "story"]);

  return readFileSync(log, "utf8");
}

afterEach(() => {
  for (const folder of FOLDERS.splice(0)) {
    rmSync(folder, { force: true, recursive: true });
  }
});

describe("append-only logs", () => {
  it.each(LOGS)("[F18.AC4] marks %s for a union merge", (log) => {
    expect(git(ROOT, ["check-attr", "merge", "--", log])).toBe(`${log}: merge: union\n`);
  });

  it("[F18.AC4] keeps both branches' rows on a merge, without conflict", () => {
    const merged = unionMerge();

    expect(merged).toContain("| 2026-10-17 | from the story |\n");
    expect(merged).toContain("| 2026-10-17 | from main |\n");
    expect(merged).not.toContain("<<<<<<<");
  });

  it.each(LOGS)("[F18.AC4] keeps the rows of %s unique", (log) => {
    expect(REPEATED[log]?.(logText(log))).toStrictEqual([]);
  });

  it("[F23.AC1] names a card written twice, not a bullet two cards share", () => {
    // Two cards sharing `- Instead: none`, then the first one again, its spaces changed.
    const log = [
      "# Tool incidents\n",
      "## 2026-10-18 — a\n\n- Used: a\n- Failed: b\n- Instead: none\n",
      "## 2026-10-18 — b\n\n- Used: c\n- Failed: d\n- Instead: none\n",
      "## 2026-10-18 — a\n\n- Used:  a\n- Failed: b\n- Instead: none\n",
    ].join("\n");

    expect(repeatedCards(log)).toStrictEqual(["## 2026-10-18 — a"]);

    expect(repeatedCards(log.split("\n## 2026-10-18 — a\n\n- Used:  a", 1)[0] ?? "")).toStrictEqual(
      [],
    );

    // The same heading over other bullets is two cards.
    expect(
      repeatedCards("## 2026-10-18 — a\n\n- Used: a\n\n## 2026-10-18 — a\n\n- Used: b\n"),
    ).toStrictEqual([]);
  });

  it.each(LOGS)("[F18.AC4] dates the entries of %s", (log) => {
    expect(UNDATED[log]?.(logText(log))).toStrictEqual([]);
  });

  it.each(LOGS)("[F18.AC6] keeps %s in its own shape", (log) => {
    expect(STRAY[log]?.(logText(log))).toStrictEqual([]);
  });

  it("[F18.AC6] names the lines that break a table log or the journal", () => {
    // EN-058's header, EN-055's bullet and a blank line ending the table, as move-rows wrote them.
    const log =
      "# Log\n\n| Date | Item |\n| --- | --- |\n| 2026-10-17 | a |\n| Date | Item |\n| --- | --- |\n\n- A bullet.\n  | 2026-10-17 | b |\n";

    expect(strayTableLines(log)).toStrictEqual([
      "",
      "- A bullet.",
      "  | 2026-10-17 | b |",
      "| Date | Item |",
    ]);

    expect(strayTableLines("# Log\n\nNo table.\n")).toStrictEqual([]);

    expect(
      untaggedBullets("## 2026-10-17 — run\n\n- `#run` a\n- EN-055: b\n- `#Run` c\n"),
    ).toStrictEqual(["- EN-055: b", "- `#Run` c"]);
  });

  it("[F23.AC1] names a card without its date, its use, its failure or its workaround", () => {
    // A whole card, one of what was seen, then three that each lack a part or more; a label in the
    // middle of a bullet, or without its colon, is not one.
    const log = [
      "# Tool incidents\n\nIntro.\n",
      "## 2026-10-18 — whole\n\n- Used: a\n- Failed: b\n- Instead: c\n",
      "## 2026-10-18 — seen\n\n- Seen (REV-004): d\n- Instead: e\n",
      "## 2026-10-18 — no workaround\n\n- Used (found by AUD-024): f\n- Fails: g\n- Instead of h\n",
      "## 2026-10-18 — inline\n\n- It was Used: a, Failed: b, Instead: c\n",
      "## 2026-10-18 — no failure\n\n- Used: i\n- Instead: j\n",
    ].join("\n");

    expect(incompleteCards(log)).toStrictEqual([
      "## 2026-10-18 — no workaround: lacks the workaround",
      "## 2026-10-18 — inline: lacks the use, the failure, the workaround",
      "## 2026-10-18 — no failure: lacks the failure",
    ]);

    expect(incompleteCards("# Tool incidents\n\nNo card yet.\n")).toStrictEqual([]);

    expect(undatedSessions("## Someday — a card\n\n- Used: a\n")).toStrictEqual([
      "## Someday — a card",
    ]);
  });

  it("[F18.AC4] names a table repeated with other padding", () => {
    const log = "| 2026-10-17 | a |\n\n| 2026-10-17   | a      |\n";

    expect(repeatedRows(log)).toStrictEqual(["| 2026-10-17 | a |"]);
  });

  it("[F18.AC4] names a row written twice by a union merge", () => {
    const log =
      "| Date | Item |\n| --- | --- |\n| 2026-10-17 | a |\n| 2026-10-17 | b |\n| 2026-10-17 | a |\n";

    expect(repeatedRows(log)).toStrictEqual(["| 2026-10-17 | a |"]);
  });

  it("[F18.AC4] names a bullet written twice", () => {
    expect(repeatedRows("## 2026-10-17 — run\n\n- one\n- two\n- one\n")).toStrictEqual(["- one"]);
  });

  it("[F18.AC4] names an undated row and an undated session", () => {
    expect(undatedRows("| Date | Item |\n| --- | --- |\n| soon | a |\n")).toStrictEqual([
      "| soon | a |",
    ]);

    expect(undatedSessions("## Run of E17\n")).toStrictEqual(["## Run of E17"]);
  });
});

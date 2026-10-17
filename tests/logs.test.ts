import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { afterEach, describe, expect, it } from "vitest";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/** The append-only logs merged by union (ADR-0065), from the root. */
const LOGS = [
  "docs/process/journal.md",
  "docs/process/improvements.md",
  "docs/process/feedback.md",
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

/** Each log's check of its shape: the two table logs hold one table, the journal tagged bullets. */
const STRAY: Readonly<Record<string, (text: string) => readonly string[]>> = {
  "docs/process/feedback.md": strayTableLines,
  "docs/process/improvements.md": strayTableLines,
  "docs/process/journal.md": untaggedBullets,
};

/** Each log's check of its dates: the journal dates its sessions, the two other logs their rows. */
const UNDATED: Readonly<Record<string, (text: string) => readonly string[]>> = {
  "docs/process/feedback.md": undatedRows,
  "docs/process/improvements.md": undatedRows,
  "docs/process/journal.md": undatedSessions,
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
    expect(repeatedRows(logText(log))).toStrictEqual([]);
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

import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/**
 * The 854 rules the theme files of `eslint/rules/` decided when ESLint left, one per line: the
 * keys of their `rules` objects at `650d1b5d^`, the commit before EN-085 removed them (AUD-021).
 */
const ESLINT_RULES: readonly string[] = readFileSync(
  join(ROOT, "tests", "eslint-rules.txt"),
  "utf8",
)
  .split("\n")
  .filter((line) => line !== "");

/** ADR-0075's section on the rules of `eslint/`, which says what became of each. */
const SECTION =
  /## The rules of `eslint\/`\n(?<body>[\s\S]*?)\n## References/u.exec(
    readFileSync(join(ROOT, "docs", "adr", "0075-the-toolchain-on-oxc.md"), "utf8"),
  )?.groups?.["body"] ?? "";

/**
 * The rows of a table of the section, below a heading opening with a given text.
 *
 * @param heading - the start of the heading's text
 * @returns each row's cells, the header and separator left out
 */
function rowsUnder(heading: string): readonly (readonly string[])[] {
  const table = SECTION.split("\n### ").find((part) => part.startsWith(heading)) ?? "";

  return table
    .split("\n")
    .filter((line) => line.startsWith("| ") && !line.startsWith("| ---"))
    .slice(1)
    .map((line) => line.slice(2, -2).split(" | "));
}

/**
 * The names written in code font in a cell.
 *
 * @param cell - a table cell
 * @returns each name between backticks
 */
function namesIn(cell: string): readonly string[] {
  return cell
    .matchAll(/`(?<name>[^`]+)`/gu)
    .map((match) => match.groups?.["name"] ?? "")
    .toArray();
}

/** The table of the rules carried: theme, oxlint's column, the project's linter's column. */
const CARRIED = rowsUnder("Carried");

/**
 * A rule oxlint carries, from the names of one item of the table's cell.
 *
 * @param names - its name in ESLint and, when it differs, in oxlint
 * @returns both names
 */
function carriedRule(names: Readonly<Record<string, string | undefined>>): {
  readonly eslint: string;
  readonly oxlint: string;
} {
  const eslint = names["eslint"] ?? "";

  return { eslint, oxlint: names["oxlint"] ?? eslint };
}

/** Each rule oxlint carries: its name in ESLint, and in oxlint when it differs (in parentheses). */
const BY_OXLINT = CARRIED.flatMap((cells) =>
  (cells[1] ?? "")
    .matchAll(/`(?<eslint>[^`]+)`(?: \(`(?<oxlint>[^`]+)`\))?/gu)
    .map((match) => carriedRule(match.groups ?? {}))
    .toArray(),
);

/** Each rule the project's linter carries. */
const BY_PROJECT_LINTER = CARRIED.flatMap((cells) => namesIn(cells[2] ?? ""));

/** Each rule the triage dropped. */
const DROPPED = rowsUnder("Dropped").flatMap((cells) => namesIn(cells.at(-1) ?? ""));

/** Each rule ESLint already had off. */
const OFF = rowsUnder("Off in ESLint already").flatMap((cells) => namesIn(cells.at(-1) ?? ""));

/** The names `.oxlintrc.json` gives a setting. */
const OXLINT_CONFIGURED: ReadonlySet<string> = new Set(
  readFileSync(join(ROOT, ".oxlintrc.json"), "utf8")
    .matchAll(/"(?<name>[@\w/-]+)": (?:"|\[)/gu)
    .map((match) => match.groups?.["name"] ?? ""),
);

/**
 * Every Rust file under a folder.
 *
 * @param folder - a folder of the repository
 * @returns the files' paths
 */
function rustFilesUnder(folder: string): readonly string[] {
  return readdirSync(folder, { recursive: true, encoding: "utf8" })
    .filter((path) => path.endsWith(".rs"))
    .map((path) => join(folder, path));
}

/** The names the rules of the project's linter give themselves (`fn name`). */
const PROJECT_LINTER_RULES: ReadonlySet<string> = new Set(
  rustFilesUnder(join(ROOT, "tools", "lint", "src")).flatMap((path) =>
    readFileSync(path, "utf8")
      .matchAll(/fn name\(&self\) -> &'static str \{\s*"(?<name>[^"]+)"/gu)
      .map((match) => match.groups?.["name"] ?? "")
      .toArray(),
  ),
);

describe("what became of each rule of `eslint/` (ADR-0075)", () => {
  it("[F19.AC1] gives each of the 854 rules one fate: carried, dropped or already off", () => {
    const fates = ESLINT_RULES.map((name) => ({
      count: [
        BY_OXLINT.some((rule) => rule.eslint === name),
        BY_PROJECT_LINTER.includes(name),
        DROPPED.includes(name),
        OFF.includes(name),
      ].filter(Boolean).length,
      name,
    }));

    expect(ESLINT_RULES).toHaveLength(854);
    expect(fates.filter((fate) => fate.count !== 1).map((fate) => fate.name)).toEqual([]);
  });

  it("[F19.AC1] names no rule ESLint did not decide", () => {
    const named = [
      ...BY_OXLINT.map((rule) => rule.eslint),
      ...BY_PROJECT_LINTER,
      ...DROPPED,
      ...OFF,
    ];

    expect(named.filter((name) => !ESLINT_RULES.includes(name))).toEqual([]);
  });

  it("[F19.AC1] counts 374 run by oxlint, 205 by the project's linter, 181 dropped, 94 off", () => {
    expect([BY_OXLINT.length, BY_PROJECT_LINTER.length, DROPPED.length, OFF.length]).toEqual([
      374, 205, 181, 94,
    ]);
  });

  it("[F19.AC1] runs each rule carried by oxlint under its name in `.oxlintrc.json`", () => {
    expect(BY_OXLINT.filter((rule) => !OXLINT_CONFIGURED.has(rule.oxlint))).toEqual([]);
  });

  it("[F19.AC1] runs each rule carried by the project's linter as one of its rules", () => {
    expect(BY_PROJECT_LINTER.filter((name) => !PROJECT_LINTER_RULES.has(name))).toEqual([]);
  });
});

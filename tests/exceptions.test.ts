import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/** The register of exceptions (ADR-0066). */
const REGISTER = readFileSync(join(ROOT, "docs", "conventions", "exceptions.md"), "utf8");

/** A place where the code turns a lint rule off: its file and its rules, sorted and joined. */
type Exception = {
  readonly file: string;
  readonly rules: string;
};

/** A row of the register: the place, the function that holds it and the reason. */
type Row = Exception & {
  readonly function: string;
  readonly reason: string;
};

/** The openers of a line and of a block comment, kept apart from the words that follow them. */
const COMMENT = { block: "/*", line: "//" };

/** A directive turning rules off, in a line or a block comment, up to its `--` justification. */
const DIRECTIVE = /(?:\/\/|\/\*)\s*eslint-disable(?:-next-line|-line)?\s+(?<rules>[^\n]*?)\s+--/gu;

/**
 * The rules a directive names, sorted and joined, so that their order does not count.
 *
 * @param rules - the rules as written, comma-separated
 * @returns the rules, sorted, joined by a comma and a space
 */
function sortedRules(rules: string): string {
  return rules
    .split(",")
    .map((rule) => rule.trim().replaceAll("`", ""))
    .toSorted((first, second) => first.localeCompare(second))
    .join(", ");
}

/**
 * The exceptions a file's code holds, one per directive.
 *
 * @param file - the file's path from the root
 * @param code - the file's content
 * @returns one exception per directive
 */
function codeExceptions(file: string, code: string): readonly Exception[] {
  return code
    .matchAll(DIRECTIVE)
    .map((match) => ({ file, rules: sortedRules(match.groups?.["rules"] ?? "") }))
    .toArray();
}

/**
 * The rows of the register's tables whose first cell is a file.
 *
 * @param register - the register's content
 * @returns one row per line
 */
function registerRows(register: string): readonly Row[] {
  return register
    .split("\n")
    .filter((line) => /^\|\s*`[^`]+\.[a-z]+`\s*\|/u.test(line))
    .map((line) => {
      const [file = "", place = "", rules = "", reason = ""] = line
        .split(/(?<!\\)\|/u)
        .slice(1, -1)
        .map((cell) => cell.trim());

      return {
        file: file.replaceAll("`", ""),
        function: place.replaceAll("`", ""),
        reason,
        rules: sortedRules(rules),
      };
    });
}

/**
 * A place as the failures name it.
 *
 * @param place - a directive of the code or a row of the register
 * @returns `file: rules`
 */
function placeKey(place: Exception): string {
  return `${place.file}: ${place.rules}`;
}

/**
 * The places of one list missing from another, each counted: a place held twice in the code needs
 * two rows.
 *
 * @param places - the places looked for
 * @param among - the places they are looked for in
 * @returns `file: rules` for each place of `places` left without its match in `among`
 */
function unmatched(places: readonly Exception[], among: readonly Exception[]): readonly string[] {
  const others = among.map((place) => placeKey(place));

  // The n-th place of a key, from 0, is unmatched when `among` holds that key n times or fewer.
  return places
    .map((place) => placeKey(place))
    .filter(
      (key, index, all) =>
        all.slice(0, index).filter((each) => each === key).length >=
        others.filter((each) => each === key).length,
    );
}

/** Every `.ts` file of the repository, tracked or new, with its content. */
const FILES = execFileSync("git", ["ls-files", "--cached", "--others", "--exclude-standard"], {
  cwd: ROOT,
  encoding: "utf8",
})
  .split("\n")
  .filter((file) => file.endsWith(".ts"))
  .map((file) => ({ code: readFileSync(join(ROOT, file), "utf8"), file }));

/** The exceptions the code holds. */
const CODE = FILES.flatMap(({ code, file }) => codeExceptions(file, code));

/** The rows of the register. */
const ROWS = registerRows(REGISTER);

describe("register of exceptions (docs/conventions/exceptions.md, ADR-0066)", () => {
  it("[F18.AC13] reads a directive's rules and a row's cells", () => {
    // Two rules, written in any order, give one key; a line or a block comment both count. The
    // comment openers are joined at run time, so that this file holds no directive itself.
    expect(
      codeExceptions(
        "a.ts",
        `x; ${COMMENT.line} eslint-disable-line b-rule, a-rule -- why
${COMMENT.block} eslint-disable c-rule -- why */`,
      ),
    ).toEqual([
      { file: "a.ts", rules: "a-rule, b-rule" },
      { file: "a.ts", rules: "c-rule" },
    ]);

    // A comment that only names the word is not a directive.
    expect(
      codeExceptions("a.ts", `${COMMENT.line} needs an eslint-disable comment -- here`),
    ).toEqual([]);

    expect(registerRows("| `a.ts` | `f` | `b-rule`, `a-rule` | why |\n| File | x |")).toEqual([
      { file: "a.ts", function: "f", reason: "why", rules: "a-rule, b-rule" },
    ]);
  });

  it("[F18.AC13] names a directive without its row and a row whose code is gone", () => {
    const code = [
      { file: "a.ts", rules: "x" },
      { file: "a.ts", rules: "x" },
      { file: "b.ts", rules: "y" },
    ];
    const rows = [
      { file: "a.ts", rules: "x" },
      { file: "c.ts", rules: "z" },
    ];

    // a.ts holds two directives of `x` for one row; b.ts has none; c.ts's code is gone.
    expect(unmatched(code, rows)).toEqual(["a.ts: x", "b.ts: y"]);
    expect(unmatched(rows, code)).toEqual(["c.ts: z"]);
  });

  it("[F18.AC13] lists every eslint-disable directive of the code", () => {
    // Not vacuous: the 30 directives of the code and the register are read.
    expect(CODE.length).toBeGreaterThanOrEqual(30);
    expect(unmatched(CODE, ROWS)).toEqual([]);
  });

  it("[F18.AC13] lists no exception the code no longer holds", () => {
    expect(unmatched(ROWS, CODE)).toEqual([]);
  });

  it("[F18.AC13] gives each row a function of its file and a reason", () => {
    const codes = new Map(FILES.map(({ code, file }) => [file, code]));

    expect(
      ROWS.filter(
        (row) => row.reason === "" || codes.get(row.file)?.includes(row.function) !== true,
      ).map((row) => `${row.file}: ${row.function}`),
    ).toEqual([]);
  });
});

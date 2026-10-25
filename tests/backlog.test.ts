import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** A criterion of a done feature, as the records export it for the CI (ADR-0027, EN-154). */
type Criterion = {
  readonly id: string;
  readonly retired?: string;
};

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/**
 * The criteria of the done features, written by `target/release/knowledge export-criteria` from
 * the records, where the backlog lives since EN-154: the CI reads this file, never the records
 * (E49, Decisions, Q3).
 */
const CRITERIA_FILE = join(import.meta.dirname, "data", "criteria.json");

/** The form of a criterion's id, `F48.AC1`. */
const CRITERION_ID = /^F\d{2}\.AC\d+$/u;

/**
 * Folders whose TypeScript tests may verify acceptance criteria (ADR-0027): `tests` for the
 * criteria on the code base itself (F11), the scripts and lint rules for the tooling's (F18); the
 * hooks are Rust since EN-104.
 */
const TEST_ROOTS = ["src", "playground", "tests", "scripts"];

/** The Rust tools, whose tests verify the tooling's criteria (F18). */
const TOOLS = join(ROOT, "tools");

/** This file: it names tags in its own code, so only its tests' titles count. */
const CHECKER = "backlog.test.ts";

/**
 * The criteria of the data file.
 *
 * @returns each criterion, in the file's order
 */
function criteria(): readonly Criterion[] {
  const read: unknown = JSON.parse(readFileSync(CRITERIA_FILE, "utf8"));

  const entries: readonly unknown[] = Array.isArray(read) ? read : [];

  return entries.filter((entry) => isCriterion(entry));
}

/**
 * Whether a value of the data file is a criterion: an object with a text `id`, and a text
 * `retired` when it has one.
 *
 * @param value - an entry of the file
 * @returns `true` for a criterion
 */
function isCriterion(value: unknown): value is Criterion {
  if (typeof value !== "object" || value === null) {
    return false;
  }

  const id: unknown = Reflect.get(value, "id");
  const retired: unknown = Reflect.get(value, "retired");

  return typeof id === "string" && (retired === undefined || typeof retired === "string");
}

/**
 * Titles of the tests of a file, the text a `[F18.AC5]` tag is written in.
 *
 * @param text - a test file
 * @returns each `it("…"` title, one per line
 */
function testTitles(text: string): string {
  return text
    .matchAll(/\bit\(\s*"(?<title>[^"]*)"/gu)
    .map((match) => match.groups?.["title"] ?? "")
    .toArray()
    .join("\n");
}

/**
 * Criterion tags of a Rust file: a test named `f18_ac3_…` reads as `[F18.AC3]`, a `// [F18.AC6]`
 * comment as itself; a `//!` module comment only describes the convention, so it is left out.
 *
 * @param text - a Rust file
 * @returns its lines without module comments, then one tag per test named after a criterion
 */
function rustTags(text: string): string {
  const code = text
    .split("\n")
    .filter((line) => !line.trimStart().startsWith("//!"))
    .join("\n");
  const names = code
    .matchAll(/\bfn f(?<feature>\d{2})_ac(?<criterion>\d+)_/gu)
    .map((match) => `[F${match.groups?.["feature"] ?? ""}.AC${match.groups?.["criterion"] ?? ""}]`)
    .toArray();

  return [code, ...names].join("\n");
}

/**
 * Rust files of the tools that may hold tests: `tests/*.rs` and `src/**\/*.rs` of each crate,
 * its build folder `target` left out.
 *
 * @returns their paths
 */
function rustFiles(): string[] {
  return readdirSync(TOOLS).flatMap((crate) =>
    ["tests", "src"].flatMap((folder) => {
      const root = join(TOOLS, crate, folder);

      return existsSync(root)
        ? readdirSync(root, { recursive: true })
            .map(String)
            .filter((file) => file.endsWith(".rs"))
            .map((file) => join(root, file))
        : [];
    }),
  );
}

/**
 * Concatenated text where `[F01.AC3]` tags are looked for: every `*.test.ts` file under the test
 * roots (only the titles of this file's tests), and the Rust tools' tags.
 *
 * @returns the text of every test that may prove a criterion
 */
function testSources(): string {
  const typescript = TEST_ROOTS.flatMap((root) =>
    readdirSync(join(ROOT, root), { recursive: true })
      .map(String)
      .filter((file) => file.endsWith(".test.ts"))
      .map((file) => {
        const text = readFileSync(join(ROOT, root, file), "utf8");

        return file.endsWith(CHECKER) ? testTitles(text) : text;
      }),
  );
  const rust = rustFiles().map((file) => rustTags(readFileSync(file, "utf8")));

  return [...typescript, ...rust].join("\n");
}

describe("backlog", () => {
  it("traces every criterion of a done feature to a test tagged with its id (ADR-0027)", () => {
    const sources = testSources();
    const untraced = criteria()
      .filter((criterion) => criterion.retired === undefined)
      .map((criterion) => criterion.id)
      .filter((id) => !sources.includes(`[${id}]`));

    expect(untraced).toEqual([]);
  });

  it("reads each criterion of the data file once, a well-formed id, a retired one with its reason", () => {
    const read = criteria();
    const ids = read.map((criterion) => criterion.id);

    expect(read.length).toBeGreaterThan(200);
    expect(isCriterion({ id: "F01.AC1", retired: 3 })).toBe(false);
    expect(isCriterion("F01.AC1")).toBe(false);
    expect(ids.filter((id) => !CRITERION_ID.test(id))).toEqual([]);
    expect(ids.filter((id, index) => ids.indexOf(id) !== index)).toEqual([]);
    expect(read.filter((criterion) => criterion.retired?.trim() === "")).toEqual([]);
  });

  it("reads a Rust test named after a criterion as its tag, never a module comment", () => {
    const text = [
      "//! `[F18.AC7]` marks a test proving the feature's criterion.",
      "fn f18_ac3_a_dead_owner_is_taken_over() {}",
      "    // [F18.AC6] the push is refused.",
    ].join("\n");
    const tags = rustTags(text).match(/\[F\d{2}\.AC\d+\]/gu);

    expect(tags).toEqual(["[F18.AC6]", "[F18.AC3]"]);
  });

  it("reads only the titles of this file's tests, never the tags its code names", () => {
    const text = 'it(\n  "[F18.AC5] keeps the index",\n  () => {});\n// a `[F01.AC3]` tag\n';

    expect(testTitles(text)).toBe("[F18.AC5] keeps the index");
  });

  it("keeps no backlog page: the epics, their order, features and stories are in the records (EN-154, AUD-054)", () => {
    expect(existsSync(join(ROOT, "docs", "backlog"))).toBe(false);
  });
});

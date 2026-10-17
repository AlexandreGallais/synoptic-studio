import { spawnSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

import { builtOnce } from "./built-once";

/** Backlog item: its folder and its front-matter fields. */
type Item = {
  readonly fields: Readonly<Record<string, string>>;
  readonly file: string;
  readonly folder: string;
};

/** Root of the backlog. */
const BACKLOG = join(import.meta.dirname, "..", "docs", "backlog");

/** The Rust tool writing the stories index from the story files (EN-049). */
const INDEX_TOOL = join(import.meta.dirname, "..", "tools", "backlog-index", "Cargo.toml");

/** Time limit of a test building the index tool: a cold build of about 5 s, four times, and a margin. */
const BUILD_TIMEOUT = 60_000;

/** Allowed statuses per folder. */
const STATUSES: Readonly<Record<string, readonly string[]>> = {
  epics: ["draft", "ready", "in-progress", "done"],
  features: ["draft", "ready", "in-progress", "done"],
  stories: ["draft", "ready", "in-progress", "done", "dropped"],
};

/** Identifier pattern per folder. */
const ID_PATTERNS: Readonly<Record<string, RegExp>> = {
  epics: /^E\d{2}$/u,
  features: /^F\d{2}$/u,
  stories: /^(?:US|EN|SP|CHK|AUD|VAL|NAM|REV|RET)-\d{3}$/u,
};

/**
 * Front-matter fields of a Markdown file (`key: value` lines between `---` fences).
 *
 * @param text - file content
 * @returns the fields, empty without front matter
 */
function frontMatter(text: string): Record<string, string> {
  const block = /^---\n(?<body>[\s\S]*?)\n---\n/u.exec(text)?.groups?.["body"] ?? "";

  return Object.fromEntries(
    block.split("\n").map((line) => {
      const [key = "", ...value] = line.split(":");

      return [
        key.trim(),
        value
          .join(":")
          .trim()
          .replace(/^"(?<inner>.*)"$/u, "$<inner>"),
      ];
    }),
  );
}

/**
 * Every backlog item, templates and folder indexes excluded.
 *
 * @returns items of all folders
 */
function readItems(): readonly Item[] {
  return Object.keys(STATUSES).flatMap((folder) =>
    readdirSync(join(BACKLOG, folder))
      .filter((file) => file.endsWith(".md") && file !== "README.md")
      .map((file) => ({
        fields: frontMatter(readFileSync(join(BACKLOG, folder, file), "utf8")),
        file,
        folder,
      })),
  );
}

/** Every backlog item, read at the first call and shared, frozen, by every test of the file (EN-090). */
const items: () => readonly Item[] = builtOnce(() => Object.freeze(readItems()));

/**
 * Identifiers of the items of one folder.
 *
 * @param folder - backlog folder
 * @returns their `id` fields
 */
function idsOf(folder: string): ReadonlySet<string> {
  return new Set(
    items()
      .filter((item) => item.folder === folder)
      .map((item) => item.fields["id"] ?? ""),
  );
}

/**
 * Whether an item has a well-formed id, a title and a status allowed in its folder.
 *
 * @param item - backlog item
 * @returns `true` when its identity fields are valid
 */
function isWellFormed(item: Item): boolean {
  return hasValidId(item) && (item.fields["title"] ?? "") !== "" && hasValidStatus(item);
}

/**
 * Whether an item's id matches the pattern of its folder.
 *
 * @param item - backlog item
 * @returns `true` for a well-formed id
 */
function hasValidId(item: Item): boolean {
  const pattern = ID_PATTERNS[item.folder] ?? /^$/u;

  return pattern.test(item.fields["id"] ?? "");
}

/**
 * Whether an item's status is allowed in its folder.
 *
 * @param item - backlog item
 * @returns `true` for an allowed status
 */
function hasValidStatus(item: Item): boolean {
  const statuses = STATUSES[item.folder] ?? [];

  return statuses.includes(item.fields["status"] ?? "");
}

/** Parent field of each child folder. */
const PARENT_FIELDS: Readonly<Record<string, string>> = { features: "epic", stories: "feature" };

/** Story kinds attached to an epic rather than a feature: naming pass, review and retrospective (ADR-0023, ADR-0049). */
const EPIC_STORY = /^(?:NAM|REV|RET)-/u;

/**
 * Whether a story belongs directly to its epic (review, retrospective).
 *
 * @param item - backlog item
 * @returns `true` for a `NAM`, `REV` or `RET` story
 */
function isEpicStory(item: Item): boolean {
  return item.folder === "stories" && EPIC_STORY.test(item.fields["id"] ?? "");
}

/**
 * Front-matter field naming an item's parent.
 *
 * @param item - backlog item
 * @returns `epic` for features and epic stories, `feature` for other stories, `undefined` for epics
 */
function parentField(item: Item): string | undefined {
  return isEpicStory(item) ? "epic" : PARENT_FIELDS[item.folder];
}

/**
 * Whether an item is attached to an existing parent (features to epics, stories to features).
 *
 * @param item - backlog item
 * @param parents - existing parent ids, by parent field
 * @returns `true` when the item needs no parent or its parent exists
 */
function hasParent(item: Item, parents: Readonly<Record<string, ReadonlySet<string>>>): boolean {
  const field = parentField(item);

  return field === undefined || (parents[field]?.has(item.fields[field] ?? "") ?? false);
}

/**
 * File-name prefix of an item: its ancestors' ids then its own (`E01-F01-US-001-`).
 *
 * @param item - backlog item
 * @returns `E01-`, `E01-F01-` or `E01-F01-US-001-`, ancestors first
 */
function expectedPrefix(item: Item): string {
  const { epic = "", feature = "", id = "" } = item.fields;

  return [epic, feature, id]
    .filter((part) => part !== "")
    .map((part) => `${part}-`)
    .join("");
}

/**
 * Statuses shown in the item tables of a Markdown file (rows `| … [ID](…) | … | status |`).
 *
 * @param path - Markdown file
 * @returns map from item id to the status written in the table
 */
function tableStatuses(path: string): ReadonlyMap<string, string> {
  return new Map(
    readFileSync(path, "utf8")
      .matchAll(/^\|[^\n[]*\[(?<id>[A-Z]+-?\d+)\]\([^)]*\)[^\n]*\|\s*(?<status>[a-z-]+)\s*\|$/gmu)
      .map((match) => [match.groups?.["id"] ?? "", match.groups?.["status"] ?? ""] as const),
  );
}

/**
 * Statuses shown in a folder index (`README.md`).
 *
 * @param folder - backlog folder
 * @returns map from item id to the status written in the index
 */
function indexStatuses(folder: string): ReadonlyMap<string, string> {
  return tableStatuses(join(BACKLOG, folder, "README.md"));
}

/** Folder of the parent of each child folder. */
const PARENT_FOLDERS: Readonly<Record<string, string>> = { features: "epics", stories: "features" };

/**
 * Statuses shown in the table of an item's parent (stories in their feature, features in their epic).
 *
 * @param item - feature or story
 * @returns map from item id to the status written in the parent's table, empty without parent file
 */
function parentStatuses(item: Item): ReadonlyMap<string, string> {
  const folder = isEpicStory(item) ? "epics" : (PARENT_FOLDERS[item.folder] ?? "");
  const prefix = expectedPrefix(item).replace(`${item.fields["id"] ?? ""}-`, "");
  const parent = readdirSync(join(BACKLOG, folder)).find((file) => file.startsWith(prefix));

  return parent === undefined ? new Map() : tableStatuses(join(BACKLOG, folder, parent));
}

/** Story kinds every feature needs: research spike, checkpoint, audit, validation (ADR-0020, ADR-0028). */
const FRAME_PREFIXES = ["SP-", "CHK-", "AUD-", "VAL-"];

/** Story kinds every started epic needs: review, retrospective (ADR-0023). */
const CLOSING_PREFIXES = ["REV-", "RET-"];

/** Story kind every started epic needs before its review: the naming pass (ADR-0049). */
const NAMING_PREFIXES = ["NAM-"];

/** Epics without a naming pass: E01 closed before ADR-0049, F13 is E14's (ADR-0049 §4). */
const BEFORE_NAMING_PASS = new Set(["E01", "E14"]);

/**
 * Whether the stories of one parent include every required kind.
 *
 * @param stories - every story item
 * @param parent - `["feature", "F01"]` or `["epic", "E01"]`
 * @param prefixes - id prefixes that must all be present
 * @returns `true` when each prefix starts the id of one of the parent's stories
 */
function hasKinds(
  stories: readonly Item[],
  parent: readonly [string, string],
  prefixes: readonly string[],
): boolean {
  const [field, value] = parent;
  const ids = stories
    .filter((story) => story.fields[field] === value)
    .map((story) => story.fields["id"] ?? "");

  return prefixes.every((prefix) => ids.some((id) => id.startsWith(prefix)));
}

/**
 * Folders whose TypeScript tests may verify acceptance criteria (ADR-0027): `tests` for the
 * criteria on the code base itself (F11), the scripts, hooks and lint rules for the tooling's (F18).
 */
const TEST_ROOTS = ["src", "playground", "tests", "scripts", ".claude/hooks"];

/** The Rust tools, whose tests verify the tooling's criteria (F18). */
const TOOLS = join(import.meta.dirname, "..", "tools");

/** This file: it names tags in its own code, so only its tests' titles count. */
const CHECKER = "backlog.test.ts";

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
    readdirSync(join(import.meta.dirname, "..", root), { recursive: true })
      .map(String)
      .filter((file) => file.endsWith(".test.ts"))
      .map((file) => {
        const text = readFileSync(join(import.meta.dirname, "..", root, file), "utf8");

        return file === CHECKER ? testTitles(text) : text;
      }),
  );
  const rust = rustFiles().map((file) => rustTags(readFileSync(file, "utf8")));

  return [...typescript, ...rust].join("\n");
}

/**
 * Section "Acceptance criteria" of a feature file.
 *
 * @param item - feature item
 * @returns the section's text, empty when absent
 */
function criteriaSection(item: Item): string {
  const text = readFileSync(join(BACKLOG, item.folder, item.file), "utf8");

  return /^## Acceptance criteria\n(?<body>[\s\S]*?)(?=^## )/mu.exec(text)?.groups?.["body"] ?? "";
}

/**
 * Criterion identifiers of a feature: one per numbered item of its "Acceptance criteria" section.
 *
 * @param item - feature item
 * @returns `F01.AC1`, `F01.AC2`…
 */
function criteriaOf(item: Item): string[] {
  return criteriaSection(item)
    .matchAll(/^(?<number>\d+)\. /gmu)
    .map((match) => `${item.fields["id"] ?? ""}.AC${match.groups?.["number"] ?? ""}`)
    .toArray();
}

describe("backlog", () => {
  it("gives every item a well-formed id, a title and an allowed status", () => {
    expect(
      items()
        .filter((item) => !isWellFormed(item))
        .map((item) => item.file),
    ).toEqual([]);
  });

  it("names every file after its parents and its id", () => {
    expect(
      items()
        .filter((item) => !item.file.startsWith(expectedPrefix(item)))
        .map((item) => item.file),
    ).toEqual([]);
  });

  it("quotes front-matter values containing a colon (YAML)", () => {
    const unquoted = Object.keys(STATUSES).flatMap((folder) =>
      readdirSync(join(BACKLOG, folder))
        .filter((file) => file.endsWith(".md"))
        .filter((file) =>
          /^\w+: [^"\n]*: /mu.test(readFileSync(join(BACKLOG, folder, file), "utf8")),
        ),
    );

    expect(unquoted).toEqual([]);
  });

  it("lists every item in its folder index", () => {
    const unlisted = items().filter(
      (item) => !indexStatuses(item.folder).has(item.fields["id"] ?? ""),
    );

    expect(unlisted.map((item) => item.file)).toEqual([]);
  });

  it("shows in every folder index the status written in each file", () => {
    const mismatches = items().filter(
      (item) => indexStatuses(item.folder).get(item.fields["id"] ?? "") !== item.fields["status"],
    );

    expect(mismatches.map((item) => item.file)).toEqual([]);
  });

  it(
    "[F18.AC5] keeps the stories index as the story files generate it",
    { timeout: BUILD_TIMEOUT },
    () => {
      const result = spawnSync(
        "cargo",
        ["run", "--quiet", "--manifest-path", INDEX_TOOL, "--", BACKLOG, "--check"],
        { encoding: "utf8" },
      );

      // The tool's message names the row that differs and says to run `npm run backlog:index`.
      expect(result.stderr).toBe("");
      expect(result.status).toBe(0);
    },
  );

  it("shows every feature and story in its parent's table, with the status written in its file", () => {
    const mismatches = items().filter(
      (item) =>
        item.folder !== "epics" &&
        parentStatuses(item).get(item.fields["id"] ?? "") !== item.fields["status"],
    );

    expect(mismatches.map((item) => item.file)).toEqual([]);
  });

  it("frames every feature with a spike, a checkpoint, an audit and a validation (ADR-0020, ADR-0028)", () => {
    const stories = items().filter((item) => item.folder === "stories");
    const features = [...new Set(stories.map((story) => story.fields["feature"] ?? ""))].filter(
      (feature) => feature !== "",
    );

    expect(
      features.filter((feature) => !hasKinds(stories, ["feature", feature], FRAME_PREFIXES)),
    ).toEqual([]);
  });

  it("closes every started epic with a review and a retrospective (ADR-0023)", () => {
    const stories = items().filter((item) => item.folder === "stories");
    const epics = [...new Set(stories.map((story) => story.fields["epic"] ?? ""))];
    const unclosed = epics.filter((epic) => !hasKinds(stories, ["epic", epic], CLOSING_PREFIXES));

    expect(unclosed).toEqual([]);
  });

  it("closes every started epic since ADR-0049 with a naming pass", () => {
    const stories = items().filter((item) => item.folder === "stories");
    const epics = [...new Set(stories.map((story) => story.fields["epic"] ?? ""))];
    const unnamed = epics.filter(
      (epic) =>
        !BEFORE_NAMING_PASS.has(epic) && !hasKinds(stories, ["epic", epic], NAMING_PREFIXES),
    );

    expect(unnamed).toEqual([]);
  });

  it("traces every criterion of a done feature to a test tagged with its id (ADR-0027)", () => {
    const sources = testSources();
    const untraced = items()
      .filter((item) => item.folder === "features" && item.fields["status"] === "done")
      .flatMap((item) => criteriaOf(item))
      .filter((criterion) => !sources.includes(`[${criterion}]`));

    expect(untraced).toEqual([]);
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

  it("uses each id once", () => {
    const ids = items().map((item) => item.fields["id"] ?? "");

    expect(ids.filter((id, index) => ids.indexOf(id) !== index)).toEqual([]);
  });

  it("attaches every feature to an existing epic and every story to an existing feature", () => {
    const parents = { epic: idsOf("epics"), feature: idsOf("features") };

    expect(
      items()
        .filter((item) => !hasParent(item, parents))
        .map((item) => item.file),
    ).toEqual([]);
  });
});

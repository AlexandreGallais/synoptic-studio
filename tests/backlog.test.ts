import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Backlog item: its folder and its front-matter fields. */
type Item = {
  readonly fields: Readonly<Record<string, string>>;
  readonly file: string;
  readonly folder: string;
};

/** Root of the backlog. */
const BACKLOG = join(import.meta.dirname, "..", "docs", "backlog");

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
function items(): Item[] {
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

/** Folders whose tests may verify acceptance criteria (ADR-0027); `tests` for the criteria on the code base itself (F11). */
const TEST_ROOTS = ["src", "playground", "tests"];

/**
 * Concatenated content of every test file, where `[F01.AC3]` tags are looked for.
 *
 * @returns text of all `*.test.ts` files under the test roots
 */
function testSources(): string {
  return TEST_ROOTS.flatMap((root) =>
    readdirSync(join(import.meta.dirname, "..", root), { recursive: true })
      .map(String)
      // This file names tags in its own text: it proves no criterion.
      .filter((file) => file.endsWith(".test.ts") && file !== "backlog.test.ts")
      .map((file) => readFileSync(join(import.meta.dirname, "..", root, file), "utf8")),
  ).join("\n");
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

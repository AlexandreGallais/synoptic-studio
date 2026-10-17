// Uses of the local source library (docs/tooling/sources.md): each search and each page read is
// recorded, so that a retrospective sees which sources the agent consulted most, what it searched
// for, and how often the library spared a request online. The library serves every project of the
// machine: each use names the project that asked (EN-101).

import { basename } from "node:path";

import { markdownTable } from "./metrics-tables.ts";

import type { DatabaseSync } from "node:sqlite";

/** One use of the library. */
export type Use = {
  /** Where the page came from — `library`, `online`, `failed` — or, for a search, `found` or `none`. */
  readonly origin: string;
  /** The project that asked, named by its folder (`projectOf`). */
  readonly project: string;
  /** The words searched, empty for a page. */
  readonly query: string;
  /** `search`, `get` or `read`. */
  readonly tool: string;
  /** The page, empty for a search. */
  readonly url: string;
};

/** Rows shown in each ranked table. */
const TOP = 15;

/** The folder of a story's worktree inside a project (EN-052), the project's folder captured. */
const WORKTREE = /^(?<project>.*?)\/\.claude\/worktrees(?:\/|$)/u;

/**
 * The project a folder belongs to, as the library records it: the name of the project's folder,
 * the same for each of its worktrees.
 *
 * @param directory - absolute folder the agent works in, possibly a worktree or below it
 * @returns `synoptic-studio` for `/…/synoptic-studio/.claude/worktrees/en-101`, empty for none
 */
export function projectOf(directory: string): string {
  return basename(WORKTREE.exec(directory)?.groups?.["project"] ?? directory);
}

/**
 * Records one use of the library.
 *
 * @param database - open library
 * @param use - what was searched or read, and by which project
 * @param time - when, epoch milliseconds
 */
export function recordUse(database: DatabaseSync, use: Use, time: number): void {
  database
    .prepare(
      "INSERT INTO project_uses (time, tool, url, query, origin, project) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .run(time, use.tool, use.url, use.query, use.origin, use.project);
}

/**
 * Uses of the library per project since a time.
 *
 * @param database - open library
 * @param since - epoch milliseconds
 * @returns a Markdown table: project, pages consulted, of which online, searches
 */
function projectsTable(database: DatabaseSync, since: number): string {
  const rows = database
    .prepare(
      `SELECT project, SUM(tool != 'search') AS pages,
        SUM(tool != 'search' AND origin = 'online') AS online, SUM(tool = 'search') AS searches
      FROM project_uses WHERE time >= ? GROUP BY project ORDER BY COUNT(*) DESC`,
    )
    .all(since)
    .map((row) => [
      String(row["project"]) === "" ? "(not recorded)" : String(row["project"]),
      String(row["pages"]),
      String(row["online"]),
      String(row["searches"]),
    ]);

  return markdownTable(["Project", "Pages consulted", "Of which online", "Searches"], rows);
}

/**
 * Sources consulted most since a time.
 *
 * @param database - open library
 * @param since - epoch milliseconds
 * @returns a Markdown table: page, title, consultations, of which online
 */
function sourcesTable(database: DatabaseSync, since: number): string {
  const rows = database
    .prepare(
      `SELECT project_uses.url, COALESCE(pages.title, '') AS title, COUNT(*) AS count,
        SUM(origin = 'online') AS online
      FROM project_uses LEFT JOIN pages ON pages.url = project_uses.url
      WHERE tool != 'search' AND time >= ? GROUP BY project_uses.url ORDER BY count DESC LIMIT ?`,
    )
    .all(since, TOP)
    .map((row) => [
      String(row["title"]),
      `<${String(row["url"])}>`,
      String(row["count"]),
      String(row["online"]),
    ]);

  return markdownTable(["Source", "Address", "Consulted", "Of which online"], rows);
}

/**
 * Searches run most since a time.
 *
 * @param database - open library
 * @param since - epoch milliseconds
 * @returns a Markdown table: words, times, found
 */
function searchesTable(database: DatabaseSync, since: number): string {
  const rows = database
    .prepare(
      `SELECT query, COUNT(*) AS count, SUM(origin = 'found') AS found FROM project_uses
      WHERE tool = 'search' AND time >= ? GROUP BY query ORDER BY count DESC LIMIT ?`,
    )
    .all(since, TOP)
    .map((row) => [`\`${String(row["query"])}\``, String(row["count"]), String(row["found"])]);

  return markdownTable(["Words searched", "Times", "Found"], rows);
}

/**
 * Report of the library's uses since a time: what was consulted most, what was searched, and the
 * share of pages served without going online.
 *
 * @param database - open library
 * @param since - epoch milliseconds
 * @returns Markdown sections
 */
export function usageReport(database: DatabaseSync, since: number): string {
  const totals = database
    .prepare(
      `SELECT COUNT(*) AS reads, COALESCE(ROUND(100.0 * SUM(origin = 'library') / COUNT(*)), 0) AS share
      FROM project_uses WHERE tool != 'search' AND time >= ?`,
    )
    .get(since);
  const reads = String(totals?.["reads"] ?? 0);
  const share = String(totals?.["share"] ?? 0);

  return [
    `${reads} pages consulted, ${share} % served by the library without going online.`,
    "### Projects",
    projectsTable(database, since),
    "### Sources consulted most",
    sourcesTable(database, since),
    "### Searches",
    searchesTable(database, since),
  ].join("\n\n");
}

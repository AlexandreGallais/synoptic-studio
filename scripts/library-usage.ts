// Uses of the local source library (docs/tooling/sources.md): each search and each page read is
// recorded, so that a retrospective sees which sources the agent consulted most, what it searched
// for, and how often the library spared a request online.

import { markdownTable } from "./metrics-tables.ts";

import type { DatabaseSync } from "node:sqlite";

/** One use of the library. */
export type Use = {
  /** Where the page came from — `library`, `online`, `failed` — or, for a search, `found` or `none`. */
  readonly origin: string;
  /** The words searched, empty for a page. */
  readonly query: string;
  /** `search`, `get` or `read`. */
  readonly tool: string;
  /** The page, empty for a search. */
  readonly url: string;
};

/** Rows shown in each ranked table. */
const TOP = 15;

/**
 * Records one use of the library.
 *
 * @param database - open library
 * @param use - what was searched or read
 * @param time - when, epoch milliseconds
 */
export function recordUse(database: DatabaseSync, use: Use, time: number): void {
  database
    .prepare("INSERT INTO uses VALUES (?, ?, ?, ?, ?)")
    .run(time, use.tool, use.url, use.query, use.origin);
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
      `SELECT uses.url, COALESCE(pages.title, '') AS title, COUNT(*) AS count,
        SUM(origin = 'online') AS online
      FROM uses LEFT JOIN pages ON pages.url = uses.url
      WHERE tool != 'search' AND time >= ? GROUP BY uses.url ORDER BY count DESC LIMIT ?`,
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
      `SELECT query, COUNT(*) AS count, SUM(origin = 'found') AS found FROM uses
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
      FROM uses WHERE tool != 'search' AND time >= ?`,
    )
    .get(since);
  const reads = String(totals?.["reads"] ?? 0);
  const share = String(totals?.["share"] ?? 0);

  return [
    `${reads} pages consulted, ${share} % served by the library without going online.`,
    "### Sources consulted most",
    sourcesTable(database, since),
    "### Searches",
    searchesTable(database, since),
  ].join("\n\n");
}

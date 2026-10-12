// Storage of the local source library (docs/tooling/sources.md, ADR-0039): one SQLite file shared
// by every project of the machine, a table of pages and a full-text index of their sections
// (FTS5, ranked by BM25, `REF-SQLITE-FTS5`), through Node.js's built-in `node:sqlite`.

import { mkdirSync } from "node:fs";
import { homedir } from "node:os";
import { dirname, join } from "node:path";
import { DatabaseSync } from "node:sqlite";

import type { Chunk } from "./library-chunks.ts";

/** A stored page and what it says about itself. */
export type StoredPage = {
  readonly etag: string;
  readonly fetchedAt: number;
  readonly lastModified: string;
  readonly markdown: string;
  readonly modifiedAt: number;
  readonly title: string;
  readonly url: string;
};

/** One section found by a search. */
export type Hit = {
  readonly heading: string;
  readonly index: number;
  readonly snippet: string;
  readonly title: string;
  readonly url: string;
};

/** Tables of the library: pages, the full-text index of their sections, and their uses. */
const SCHEMA = `
  CREATE TABLE IF NOT EXISTS pages (
    url TEXT PRIMARY KEY, title TEXT NOT NULL, fetched_at INTEGER NOT NULL,
    modified_at INTEGER NOT NULL, etag TEXT NOT NULL, last_modified TEXT NOT NULL,
    markdown TEXT NOT NULL
  );
  CREATE VIRTUAL TABLE IF NOT EXISTS chunks USING fts5(
    url UNINDEXED, position UNINDEXED, heading, text, tokenize = 'porter unicode61'
  );
  CREATE TABLE IF NOT EXISTS uses (
    time INTEGER NOT NULL, tool TEXT NOT NULL, url TEXT NOT NULL, query TEXT NOT NULL,
    origin TEXT NOT NULL
  );
`;

/**
 * Path of the library: `LIBRARY_PATH`, else a file in the user's data folder.
 *
 * @returns absolute path of the SQLite file
 */
export function libraryPath(): string {
  return (
    process.env["LIBRARY_PATH"] ??
    join(homedir(), ".local", "share", "agent-library", "library.sqlite")
  );
}

/**
 * Opens the library, creating its file and tables when missing.
 *
 * @param path - SQLite file, or `:memory:`
 * @returns the open database
 */
export function openLibrary(path: string): DatabaseSync {
  if (path !== ":memory:") {
    mkdirSync(dirname(path), { recursive: true });
  }

  const database = new DatabaseSync(path);

  database.exec(SCHEMA);

  return database;
}

/**
 * Stores a page and indexes its sections, replacing what was stored for its address.
 *
 * @param database - open library
 * @param page - the page read online
 * @param chunks - its sections, in page order
 */
export function savePage(database: DatabaseSync, page: StoredPage, chunks: readonly Chunk[]): void {
  database
    .prepare("INSERT OR REPLACE INTO pages VALUES (?, ?, ?, ?, ?, ?, ?)")
    .run(
      page.url,
      page.title,
      page.fetchedAt,
      page.modifiedAt,
      page.etag,
      page.lastModified,
      page.markdown,
    );

  database.prepare("DELETE FROM chunks WHERE url = ?").run(page.url);

  const insert = database.prepare("INSERT INTO chunks VALUES (?, ?, ?, ?)");

  for (const chunk of chunks) {
    insert.run(page.url, chunk.index, chunk.heading, chunk.text);
  }
}

/**
 * Marks a stored page as read again online without change (HTTP 304).
 *
 * @param database - open library
 * @param url - address of the page
 * @param fetchedAt - when it was checked, epoch milliseconds
 */
export function touchPage(database: DatabaseSync, url: string, fetchedAt: number): void {
  database.prepare("UPDATE pages SET fetched_at = ? WHERE url = ?").run(fetchedAt, url);
}

/**
 * A stored page.
 *
 * @param database - open library
 * @param url - its address
 * @returns the page, or `undefined` when it is not stored
 */
export function findPage(database: DatabaseSync, url: string): StoredPage | undefined {
  const row = database.prepare("SELECT * FROM pages WHERE url = ?").get(url);

  return row === undefined
    ? undefined
    : {
        etag: String(row["etag"]),
        fetchedAt: Number(row["fetched_at"]),
        lastModified: String(row["last_modified"]),
        markdown: String(row["markdown"]),
        modifiedAt: Number(row["modified_at"]),
        title: String(row["title"]),
        url: String(row["url"]),
      };
}

/**
 * Words of a search as an FTS5 query: each word quoted, so that no character of the text is read as
 * query syntax.
 *
 * @param words - the words searched
 * @param operator - `AND` for sections holding every word, `OR` for any
 * @returns the query, e.g. `"bm25" AND "rank"`
 */
export function ftsQuery(words: string, operator: "AND" | "OR"): string {
  return words
    .split(/\s+/u)
    .filter((word) => word !== "")
    .map((word) => `"${word.replaceAll('"', '""')}"`)
    .join(` ${operator} `);
}

/**
 * Sections matching an FTS5 query, best first (BM25).
 *
 * @param database - open library
 * @param query - FTS5 query
 * @param limit - most sections returned
 * @returns the sections, each with its page, heading and a snippet around the words
 */
function matchingSections(database: DatabaseSync, query: string, limit: number): readonly Hit[] {
  return database
    .prepare(
      `SELECT chunks.url, position, heading, pages.title,
        snippet(chunks, 3, '[', ']', '…', 24) AS snippet
      FROM chunks JOIN pages ON pages.url = chunks.url
      WHERE chunks MATCH ? ORDER BY bm25(chunks) LIMIT ?`,
    )
    .all(query, limit)
    .map((row) => ({
      heading: String(row["heading"]),
      index: Number(row["position"]),
      snippet: String(row["snippet"]),
      title: String(row["title"]),
      url: String(row["url"]),
    }));
}

/**
 * Sections best matching some words, best first (BM25): every word if possible, else any.
 *
 * @param database - open library
 * @param words - words searched
 * @param limit - most sections returned
 * @returns the sections, each with its page, heading and a snippet around the words
 */
export function searchLibrary(
  database: DatabaseSync,
  words: string,
  limit: number,
): readonly Hit[] {
  const all = matchingSections(database, ftsQuery(words, "AND"), limit);

  return all.length > 0 ? all : matchingSections(database, ftsQuery(words, "OR"), limit);
}

/**
 * Sections of a stored page.
 *
 * @param database - open library
 * @param url - address of the page
 * @returns its chunks in page order
 */
export function pageSections(database: DatabaseSync, url: string): readonly Chunk[] {
  return database
    .prepare("SELECT position, heading, text FROM chunks WHERE url = ? ORDER BY position")
    .all(url)
    .map((row) => ({
      heading: String(row["heading"]),
      index: Number(row["position"]),
      text: String(row["text"]),
    }));
}

/**
 * Stored pages, last read first.
 *
 * @param database - open library
 * @returns address, title and reading time of each page
 */
export function listPages(
  database: DatabaseSync,
): readonly Pick<StoredPage, "fetchedAt" | "title" | "url">[] {
  return database
    .prepare("SELECT url, title, fetched_at FROM pages ORDER BY fetched_at DESC")
    .all()
    .map((row) => ({
      fetchedAt: Number(row["fetched_at"]),
      title: String(row["title"]),
      url: String(row["url"]),
    }));
}

// Pages of the local source library (docs/tooling/sources.md): a page is read from the library
// when it is fresh, otherwise fetched online (a conditional request when it is stored), converted
// to Markdown, split into sections and stored. What the agent reads is a table of contents, then
// only the sections it needs.

import { isStale, pageChunks } from "./library-chunks.ts";
import { htmlToMarkdown, pageFacts } from "./library-html.ts";
import { findPage, pageSections, savePage, touchPage } from "./library-store.ts";

import type { Chunk } from "./library-chunks.ts";
import type { PageFacts } from "./library-html.ts";
import type { StoredPage } from "./library-store.ts";
import type { DatabaseSync } from "node:sqlite";

/** A page as the library gives it. */
export type LibraryPage = {
  /** Where it came from: the library, or the site just now; `failed` when the site refused it. */
  readonly origin: "failed" | "library" | "online";
  readonly page: StoredPage;
  readonly sections: readonly Chunk[];
};

/** HTTP status of a conditional request answered "not modified". */
const NOT_MODIFIED = 304;

/** Characters of an ISO 8601 instant that give its day (`2026-10-12`). */
const DAY_LENGTH = 10;

/** Agent string sent with each request. */
const USER_AGENT = "synoptic-studio-library/1 (documentation cache of a coding agent)";

/**
 * Epoch milliseconds of a date a page states.
 *
 * @param date - ISO 8601 instant or calendar date, possibly empty
 * @returns the time, a calendar date taken at midnight UTC, or 0 when empty
 */
export function epochOf(date: string): number {
  if (date === "") {
    return 0;
  }

  return /[T ]\d/u.test(date)
    ? Temporal.Instant.from(date).epochMilliseconds
    : Temporal.PlainDate.from(date.slice(0, DAY_LENGTH)).toZonedDateTime("UTC").epochMilliseconds;
}

/**
 * A day written for a person.
 *
 * @param epoch - epoch milliseconds, 0 when unknown
 * @returns `2026-10-12`, or `not stated`
 */
function dayOf(epoch: number): string {
  return epoch === 0
    ? "not stated"
    : Temporal.Instant.fromEpochMilliseconds(epoch).toString().slice(0, DAY_LENGTH);
}

/** A page the library has not stored. */
const NO_PAGE: StoredPage = {
  etag: "",
  fetchedAt: 0,
  lastModified: "",
  markdown: "",
  modifiedAt: 0,
  title: "",
  url: "",
};

/**
 * Value of a response header.
 *
 * @param response - an HTTP response
 * @param name - header name, lowercase
 * @returns its value, or an empty string
 */
function header(response: Response, name: string): string {
  return response.headers.get(name) ?? "";
}

/**
 * Title and last change of a body: from its first `#` heading for Markdown, from its meta tags
 * for HTML.
 *
 * @param body - text of the response
 * @param isMarkdown - whether it is Markdown or plain text
 * @returns its facts
 */
function bodyFacts(body: string, isMarkdown: boolean): PageFacts {
  return isMarkdown
    ? { modified: "", title: /^# (?<title>.+)$/mu.exec(body)?.groups?.["title"] ?? "" }
    : pageFacts(body);
}

/**
 * A page read online, as stored in the library.
 *
 * @param url - its address
 * @param response - the HTTP response
 * @param now - time of the reading, epoch milliseconds
 * @returns the stored page, Markdown kept as is, HTML converted
 */
export async function pageFromResponse(
  url: string,
  response: Response,
  now: number,
): Promise<StoredPage> {
  const body = await response.text();
  const isMarkdown = /markdown|text\/plain/u.test(header(response, "content-type"));
  const facts = bodyFacts(body, isMarkdown);

  return {
    etag: header(response, "etag"),
    fetchedAt: now,
    lastModified: header(response, "last-modified"),
    markdown: isMarkdown ? body.trim() : htmlToMarkdown(body),
    modifiedAt: epochOf(facts.modified),
    title: facts.title === "" ? url : facts.title,
    url,
  };
}

/**
 * Headers of a request for a page: a conditional request when the page is stored with an entity
 * tag or a date.
 *
 * @param stored - what the library holds, `NO_PAGE` when nothing
 * @returns the headers to send
 */
export function requestHeaders(stored: StoredPage): Record<string, string> {
  const entries: readonly (readonly [string, string])[] = [
    ["user-agent", USER_AGENT],
    ["if-none-match", stored.etag],
    ["if-modified-since", stored.lastModified],
  ];

  return Object.fromEntries(entries.filter(([, value]) => value !== ""));
}

/**
 * What the library gives when the site refuses a page: the stored page, if any.
 *
 * @param database - open library
 * @param stored - what the library holds, `NO_PAGE` when nothing
 * @param status - HTTP status of the refusal
 * @returns the stored page and its sections, or an empty page titled with the status
 */
function failedPage(database: DatabaseSync, stored: StoredPage, status: number): LibraryPage {
  return stored.fetchedAt === 0
    ? { origin: "failed", page: { ...stored, title: `HTTP ${String(status)}` }, sections: [] }
    : { origin: "failed", page: stored, sections: pageSections(database, stored.url) };
}

/**
 * A page read online and stored; the stored page kept when the site says it did not change or
 * refuses it.
 *
 * @param database - open library
 * @param url - address of the page
 * @param stored - what the library holds, `NO_PAGE` when nothing
 * @returns the page and its sections
 */
async function fetchedPage(
  database: DatabaseSync,
  url: string,
  stored: StoredPage,
): Promise<LibraryPage> {
  const now = Temporal.Now.instant().epochMilliseconds;
  const response = await fetch(url, { headers: requestHeaders(stored) });

  if (response.status === NOT_MODIFIED) {
    touchPage(database, url, now);

    return {
      origin: "online",
      page: { ...stored, fetchedAt: now },
      sections: pageSections(database, url),
    };
  }

  if (!response.ok) {
    return failedPage(database, { ...stored, url }, response.status);
  }

  const page = await pageFromResponse(url, response, now);
  const sections = pageChunks(page.markdown);

  savePage(database, page, sections);

  return { origin: "online", page, sections };
}

/**
 * A page from the library, fetched first when missing, stale or asked fresh.
 *
 * @param database - open library
 * @param url - address of the page
 * @param isRefresh - true to read it online whatever its age
 * @returns the page and its sections, with where it came from
 */
export async function libraryPage(
  database: DatabaseSync,
  url: string,
  isRefresh: boolean,
): Promise<LibraryPage> {
  const stored = findPage(database, url);
  const now = Temporal.Now.instant().epochMilliseconds;
  const isFresh = stored !== undefined && !isRefresh && !isStale({ ...stored, now });

  return isFresh
    ? { origin: "library", page: stored, sections: pageSections(database, url) }
    : await fetchedPage(database, url, stored ?? NO_PAGE);
}

/**
 * Table of contents of a page, for the agent to choose the sections to read.
 *
 * @param entry - the page and its sections
 * @returns its title, address, dates and one line per section: number, heading, length
 */
export function tableOfContents(entry: LibraryPage): string {
  const { page, sections } = entry;
  const lines = sections.map(
    (section) =>
      `${String(section.index)}. ${section.heading === "" ? "(top)" : section.heading} — ${String(section.text.length)} characters`,
  );

  return [
    `# ${page.title}`,
    `${page.url} — read ${dayOf(page.fetchedAt)} (${entry.origin}), changed ${dayOf(page.modifiedAt)}`,
    "",
    ...lines,
  ].join("\n");
}

import { describe, expect, it } from "vitest";

import { pageChunks } from "./library-chunks";
import {
  findPage,
  ftsQuery,
  listPages,
  openLibrary,
  pageSections,
  savePage,
  searchLibrary,
  touchPage,
} from "./library-store";

/** A small page about the fillet of a corner. */
const PAGE = {
  etag: '"v1"',
  fetchedAt: 1000,
  lastModified: "",
  markdown:
    "# Fillet\n\nA fillet replaces a corner by an arc.\n\n## Setback\n\nThe setback is r / tan(θ/2).",
  modifiedAt: 0,
  title: "Fillets",
  url: "https://example.org/fillet",
};

describe("library store", () => {
  const database = openLibrary(":memory:");

  savePage(database, PAGE, pageChunks(PAGE.markdown));

  it("finds a stored page and its sections", () => {
    expect(findPage(database, PAGE.url)).toEqual(PAGE);
    expect(findPage(database, "https://example.org/none")).toBeUndefined();

    expect(pageSections(database, PAGE.url).map((section) => section.heading)).toEqual([
      "Fillet",
      "Fillet › Setback",
    ]);
  });

  it("finds the section holding every word, stemmed, else any word", () => {
    expect(searchLibrary(database, "setbacks tan", 5).map((hit) => hit.index)).toEqual([1]);

    expect(searchLibrary(database, "corner nowhere", 5).map((hit) => hit.heading)).toEqual([
      "Fillet",
    ]);

    expect(searchLibrary(database, "nothing", 5)).toEqual([]);
  });

  it("quotes each word, so that the text is never read as query syntax", () => {
    expect(ftsQuery(' a"b  OR ', "AND")).toBe('"a""b" AND "OR"');
  });

  it("marks a page read again, and lists pages last read first", () => {
    savePage(database, { ...PAGE, fetchedAt: 500, url: "https://example.org/old" }, []);
    touchPage(database, PAGE.url, 2000);

    expect(listPages(database).map((page) => [page.url, page.fetchedAt])).toEqual([
      [PAGE.url, 2000],
      ["https://example.org/old", 500],
    ]);
  });
});

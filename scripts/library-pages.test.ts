import { afterEach, describe, expect, it, vi } from "vitest";

import { pageChunks } from "./library-chunks";
import { answer } from "./library-mcp";
import {
  epochOf,
  libraryPage,
  pageFromResponse,
  requestHeaders,
  tableOfContents,
} from "./library-pages";
import { openLibrary, savePage } from "./library-store";

/** Markdown served for the tests. */
const MARKDOWN = "# Hooks\n\nIntro.\n\n## Input\n\nThe `duration_ms` field.";

afterEach(() => {
  vi.unstubAllGlobals();
});

describe("epochOf", () => {
  it("reads an instant, a calendar date at midnight UTC, and nothing", () => {
    expect(epochOf("1970-01-02T00:00:00Z")).toBe(86_400_000);
    expect(epochOf("1970-01-03")).toBe(172_800_000);
    expect(epochOf("")).toBe(0);
  });
});

describe("pageFromResponse", () => {
  it("keeps Markdown as served, its title from its first heading", async () => {
    const response = new Response(MARKDOWN, {
      headers: { "content-type": "text/markdown", etag: '"e"' },
    });
    const page = await pageFromResponse("https://x.org/h.md", response, 7);

    expect(page).toEqual({
      etag: '"e"',
      fetchedAt: 7,
      lastModified: "",
      markdown: MARKDOWN,
      modifiedAt: 0,
      title: "Hooks",
      url: "https://x.org/h.md",
    });
  });

  it("converts HTML, and names an untitled page by its address", async () => {
    const response = new Response("<main><p>Body</p></main>", {
      headers: { "content-type": "text/html" },
    });
    const page = await pageFromResponse("https://x.org/p", response, 7);

    expect([page.title, page.markdown]).toEqual(["https://x.org/p", "Body"]);
  });
});

describe("requestHeaders", () => {
  it("asks only for a change when the page is stored with a tag or a date", () => {
    const stored = { ...emptyPage(), etag: '"e"', lastModified: "Mon" };

    expect(Object.keys(requestHeaders(stored))).toEqual([
      "user-agent",
      "if-none-match",
      "if-modified-since",
    ]);

    const missing = emptyPage();

    expect(Object.keys(requestHeaders(missing))).toEqual(["user-agent"]);
  });
});

/**
 * A served Markdown page, as `fetch` would answer it.
 *
 * @returns a response whose body is `MARKDOWN`, typed `text/markdown`
 */
function markdownResponse(): Response {
  return new Response(MARKDOWN, { headers: { "content-type": "text/markdown" } });
}

/**
 * Answer of the MCP server to a tool call.
 *
 * @param database - open library
 * @param name - `library_search`, `library_get` or `library_read`
 * @param input - its arguments
 * @returns the JSON-RPC response, as text
 */
async function toolCall(
  database: ReturnType<typeof openLibrary>,
  name: string,
  input: unknown,
): Promise<string> {
  const response = await answer(database, {
    id: 1,
    jsonrpc: "2.0",
    method: "tools/call",
    params: { arguments: input, name },
  });

  return JSON.stringify(response);
}

/**
 * A page that is not stored.
 *
 * @returns its empty fields
 */
function emptyPage(): Parameters<typeof requestHeaders>[0] {
  return {
    etag: "",
    fetchedAt: 0,
    lastModified: "",
    markdown: "",
    modifiedAt: 0,
    title: "",
    url: "",
  };
}

describe("libraryPage and the MCP server", () => {
  it("fetches a missing page once, then reads it from the library", async () => {
    const database = openLibrary(":memory:");
    const fetchMock = vi.fn<typeof fetch>().mockResolvedValue(markdownResponse());

    vi.stubGlobal("fetch", fetchMock);

    const first = await libraryPage(database, "https://x.org/h.md", false);
    const second = await libraryPage(database, "https://x.org/h.md", false);

    expect([first.origin, second.origin]).toEqual(["online", "library"]);
    expect(fetchMock).toHaveBeenCalledTimes(1);
    expect(tableOfContents(second)).toContain("1. Hooks › Input — ");
  });

  it("keeps the stored page when the site says it did not change, or refuses it", async () => {
    const database = openLibrary(":memory:");

    savePage(
      database,
      { ...emptyPage(), fetchedAt: 1, markdown: MARKDOWN, title: "Hooks", url: "u" },
      pageChunks(MARKDOWN),
    );

    vi.stubGlobal(
      "fetch",
      vi.fn<typeof fetch>().mockResolvedValue(new Response(undefined, { status: 304 })),
    );

    const unchanged = await libraryPage(database, "u", true);

    expect(unchanged.sections).toHaveLength(2);

    vi.stubGlobal(
      "fetch",
      vi.fn<typeof fetch>().mockResolvedValue(new Response(undefined, { status: 404 })),
    );

    const refused = await libraryPage(database, "u", true);
    const missing = await libraryPage(openLibrary(":memory:"), "v", false);

    expect([refused.origin, refused.sections.length]).toEqual(["failed", 2]);
    expect(missing.page.title).toBe("HTTP 404");
  });

  it("answers the protocol and the three tools", async () => {
    const database = openLibrary(":memory:");

    vi.stubGlobal("fetch", vi.fn<typeof fetch>().mockResolvedValue(markdownResponse()));

    expect(
      await answer(database, { jsonrpc: "2.0", method: "notifications/initialized" }),
    ).toBeUndefined();

    expect(await answer(database, { id: 1, method: "initialize" })).toMatchObject({
      result: { capabilities: { tools: {} } },
    });

    expect(await answer(database, { id: 2, method: "nope" })).toMatchObject({
      error: { code: -32_601 },
    });

    expect(await toolCall(database, "library_search", { query: "duration" })).toContain(
      "No section found",
    );

    expect(await toolCall(database, "library_get", { url: "https://x.org/h.md" })).toContain(
      "Hooks › Input",
    );

    expect(
      await toolCall(database, "library_read", { sections: [1], url: "https://x.org/h.md" }),
    ).toContain("duration_ms");

    expect(await toolCall(database, "library_search", { limit: 1, query: "duration" })).toContain(
      "§1",
    );
  });
});

import { describe, expect, it } from "vitest";

import { openLibrary, savePage } from "./library-store";
import { recordUse, usageReport } from "./library-usage";

describe("library uses", () => {
  const database = openLibrary(":memory:");
  const page = {
    etag: "",
    fetchedAt: 1,
    lastModified: "",
    markdown: "# Hooks",
    modifiedAt: 0,
    title: "Hooks",
    url: "https://x.org/hooks",
  };

  savePage(database, page, []);
  recordUse(database, { origin: "online", query: "", tool: "get", url: page.url }, 1000);
  recordUse(database, { origin: "library", query: "", tool: "read", url: page.url }, 2000);
  recordUse(database, { origin: "library", query: "", tool: "read", url: page.url }, 3000);
  recordUse(database, { origin: "found", query: "duration", tool: "search", url: "" }, 3000);
  recordUse(database, { origin: "none", query: "duration", tool: "search", url: "" }, 4000);

  it("counts the pages consulted and the share served without going online", () => {
    // Three consultations, two from the library: 67 %.
    expect(usageReport(database, 0)).toContain("3 pages consulted, 67 % served by the library");
  });

  it("ranks the sources and the searches", () => {
    const report = usageReport(database, 0);

    expect(report).toContain("| Hooks | <https://x.org/hooks> | 3 | 1 |");
    expect(report).toContain("| `duration` | 2 | 1 |");
  });

  it("counts only what happened since a time", () => {
    expect(usageReport(database, 2500)).toContain("1 pages consulted, 100 % served");
  });
});

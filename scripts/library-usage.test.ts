import { describe, expect, it } from "vitest";

import { openLibrary, savePage } from "./library-store";
import { projectOf, recordUse, usageReport } from "./library-usage";

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
  const read = { origin: "library", project: "studio", query: "", tool: "read", url: page.url };

  savePage(database, page, []);
  recordUse(database, { ...read, origin: "online", tool: "get" }, 1000);
  recordUse(database, read, 2000);
  recordUse(database, { ...read, project: "atlas" }, 3000);

  recordUse(
    database,
    { ...read, origin: "found", query: "duration", tool: "search", url: "" },
    3000,
  );

  recordUse(
    database,
    { ...read, origin: "none", query: "duration", tool: "search", url: "" },
    4000,
  );

  it("counts the pages consulted and the share served without going online", () => {
    // Three consultations, two from the library: 67 %.
    expect(usageReport(database, 0)).toContain("3 pages consulted, 67 % served by the library");
  });

  it("ranks the sources and the searches", () => {
    const report = usageReport(database, 0);

    expect(report).toContain("| Hooks | <https://x.org/hooks> | 3 | 1 |");
    expect(report).toContain("| `duration` | 2 | 1 |");
  });

  it("groups the uses by the project that asked", () => {
    const report = usageReport(database, 0);

    // studio: get online, read from the library, two searches; atlas: one read from the library.
    expect(report).toContain("| studio | 2 | 1 | 2 |");
    expect(report).toContain("| atlas | 1 | 0 | 0 |");
  });

  it("counts only what happened since a time", () => {
    expect(usageReport(database, 2500)).toContain("1 pages consulted, 100 % served");
  });
});

describe("the project of a use", () => {
  it("is the name of the project's folder", () => {
    expect(projectOf("/home/a/github/synoptic-studio")).toBe("synoptic-studio");
    expect(projectOf("/home/a/github/atlas/")).toBe("atlas");
  });

  it("is the project's, not the story's, for a worktree of the project", () => {
    expect(projectOf("/home/a/github/SVG editor/.claude/worktrees/en-101")).toBe("SVG editor");

    expect(projectOf("/home/a/github/SVG editor/.claude/worktrees/en-101/scripts")).toBe(
      "SVG editor",
    );
  });

  it("is empty for no folder", () => {
    expect(projectOf("")).toBe("");
  });
});

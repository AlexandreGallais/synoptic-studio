import { describe, expect, it } from "vitest";

import { DAY_MS, isStale, pageChunks } from "./library-chunks";

describe("pageChunks", () => {
  it("splits at headings, keeps the heading path and ignores # lines in code blocks", () => {
    const markdown = [
      "Intro.",
      "# Guide",
      "Text.",
      "## Install",
      "```sh",
      "# not a heading",
      "```",
      "### Linux",
      "Steps.",
      "## Use",
      "Run it.",
    ].join("\n");

    expect(pageChunks(markdown).map((chunk) => [chunk.index, chunk.heading])).toEqual([
      [0, ""],
      [1, "Guide"],
      [2, "Guide › Install"],
      [3, "Guide › Install › Linux"],
      [4, "Guide › Use"],
    ]);

    expect(pageChunks(markdown)[2]?.text).toBe("## Install\n```sh\n# not a heading\n```");
  });

  it("cuts a long section between paragraphs", () => {
    const paragraph = "word ".repeat(500).trim();
    const chunks = pageChunks(`# Long\n\n${paragraph}\n\n${paragraph}`);

    // Each paragraph has 2499 characters: two do not fit under 4000.
    expect(chunks).toHaveLength(2);
    expect(chunks.every((chunk) => chunk.heading === "Long")).toBe(true);
  });
});

describe("isStale", () => {
  const fetchedAt = 100 * DAY_MS;

  it("reads a young page again after a day", () => {
    const young = { fetchedAt, modifiedAt: fetchedAt - 3 * DAY_MS };

    expect(isStale({ ...young, now: fetchedAt + DAY_MS / 2 })).toBe(false);
    expect(isStale({ ...young, now: fetchedAt + 2 * DAY_MS })).toBe(true);
  });

  it("reads any other page again after 90 days", () => {
    const old = { fetchedAt, modifiedAt: fetchedAt - 400 * DAY_MS };

    expect(isStale({ ...old, now: fetchedAt + 30 * DAY_MS })).toBe(false);
    expect(isStale({ ...old, now: fetchedAt + 91 * DAY_MS })).toBe(true);
    expect(isStale({ fetchedAt, modifiedAt: 0, now: fetchedAt + 91 * DAY_MS })).toBe(true);
  });
});

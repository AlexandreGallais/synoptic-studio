// @vitest-environment happy-dom
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import mermaid from "mermaid";
import { describe, expect, it } from "vitest";

/** Repository root. */
const ROOT = join(import.meta.dirname, "..");

/** Longest line of a label, in characters: longer lines overflow their box on narrow pages. */
const LABEL_LINE = 32;

/** Mermaid blocks of every Markdown page: file and source. */
const DIAGRAMS = [
  ...readdirSync(join(ROOT, "docs"), { recursive: true })
    .map(String)
    .filter(
      (file) => file.endsWith(".md") && !file.startsWith("api") && !file.includes(".vitepress"),
    )
    .map((file) => join("docs", file)),
  "README.md",
  "AGENTS.md",
].flatMap((file) =>
  readFileSync(join(ROOT, file), "utf8")
    .matchAll(/^```mermaid\n(?<source>[\s\S]*?)^```$/gmu)
    .map((match) => [file, match.groups?.["source"] ?? ""] as const)
    .toArray(),
);

describe("Mermaid diagrams of the docs", () => {
  it("finds the diagrams", () => {
    expect(DIAGRAMS.length).toBeGreaterThan(3);
  });

  it.each(DIAGRAMS)("parses the diagram of %s", async (...[, source]) => {
    await expect(mermaid.parse(source)).resolves.toBeTruthy();
  });

  it.each(DIAGRAMS)("keeps each label line short in %s, cut with <br/>", (...[, source]) => {
    const lines = source
      .matchAll(/[[({]"?(?<label>[^\n\])}]+?)"?[\])}]/gu)
      .flatMap((match) => (match.groups?.["label"] ?? "").split(/<br\s*\/?>/u))
      .filter((line) => line.length > LABEL_LINE)
      .toArray();

    expect(lines).toEqual([]);
  });
});

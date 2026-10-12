// @vitest-environment happy-dom
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import mermaid from "mermaid";
import { describe, expect, it } from "vitest";

/** Repository root. */
const ROOT = join(import.meta.dirname, "..");

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
});

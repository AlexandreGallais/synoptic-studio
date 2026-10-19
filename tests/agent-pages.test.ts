import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/** The pages the agent reads beside `.claude/`'s, from the root. */
const PAGES = ["AGENTS.md", "CLAUDE.md", "docs/tooling/agent-tools.md"];

/** A command `npm run`, its silent flag allowed, and the script it names. */
const NPM_RUN = /\bnpm run(?: (?:-s|--silent))? (?<script>[a-z][\w:-]*)/gu;

/**
 * The Markdown pages of `.claude/` — its agents, rules and skills — from the root, the worktrees
 * of the main checkout left out.
 *
 * @returns each page's path, `/`-separated
 */
function claudePages(): readonly string[] {
  return readdirSync(join(ROOT, ".claude"), { recursive: true })
    .map((path) => `.claude/${String(path).replaceAll("\\", "/")}`)
    .filter((path) => path.endsWith(".md") && !path.startsWith(".claude/worktrees/"));
}

/**
 * Whether a parsed JSON value is an object.
 *
 * @param value - parsed JSON
 * @returns `true` for an object, never for `null`
 */
function isObject(value: unknown): value is Readonly<Record<string, unknown>> {
  return typeof value === "object" && value !== null;
}

/**
 * The scripts of `package.json`.
 *
 * @returns their names
 */
function scripts(): ReadonlySet<string> {
  const parsed: unknown = JSON.parse(readFileSync(join(ROOT, "package.json"), "utf8"));
  const field = isObject(parsed) ? parsed["scripts"] : undefined;

  return new Set(isObject(field) ? Object.keys(field) : []);
}

/**
 * Each `npm run <script>` of a text that names a script `package.json` lacks: since ADR-0082 the
 * agent's tools have no script, so such a line names a binary of `target/release/` through npm.
 *
 * @param path - the page's path, for the message
 * @param text - its text
 * @param known - the scripts of `package.json`
 * @returns one message per such command, with its line
 */
function npmRunsOfBinaries(
  path: string,
  text: string,
  known: ReadonlySet<string>,
): readonly string[] {
  return text.split("\n").flatMap((line, index) =>
    line
      .matchAll(NPM_RUN)
      .map((match) => match.groups?.["script"] ?? "")
      .filter((script) => !known.has(script))
      .map(
        (script) =>
          `${path}:${String(index + 1)}: \`npm run ${script}\` names no script of package.json: call the tool by its binary, target/release/<binary> (ADR-0082)`,
      )
      .toArray(),
  );
}

describe("the agent's pages (ADR-0082)", () => {
  it("[F33.AC2] finds an `npm run` naming a script package.json lacks, its line named", () => {
    const known = new Set(["fix", "check:all"]);
    const text =
      "Run `npm run fix`, then `npm run -s page REF-A`.\n`npm run --silent check:all`, `npm run replace`, `npm run -s` alone.";

    expect(npmRunsOfBinaries("a.md", text, known)).toEqual([
      "a.md:1: `npm run page` names no script of package.json: call the tool by its binary, target/release/<binary> (ADR-0082)",
      "a.md:2: `npm run replace` names no script of package.json: call the tool by its binary, target/release/<binary> (ADR-0082)",
    ]);
  });

  it("[F33.AC2] names no binary of target/release/ through npm run in AGENTS.md, CLAUDE.md, .claude/ and agent-tools.md", () => {
    const known = scripts();
    const pages = [...PAGES, ...claudePages()];

    expect(pages.length).toBeGreaterThan(PAGES.length);

    expect(
      pages.flatMap((path) =>
        npmRunsOfBinaries(path, readFileSync(join(ROOT, path), "utf8"), known),
      ),
    ).toEqual([]);
  });
});

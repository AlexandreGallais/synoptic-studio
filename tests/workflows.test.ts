import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** The folder of the workflows. */
const WORKFLOWS = join(import.meta.dirname, "..", ".github", "workflows");

/** The file name of each workflow, sorted. */
const FILES: readonly string[] = readdirSync(WORKFLOWS)
  .filter((file) => file.endsWith(".yml"))
  .toSorted((left, right) => left.localeCompare(right));

/**
 * The workflows that still choose a job by the event or a branch, until the story that changes
 * them: `ci.yml` by EN-133, `story-automerge.yml` by EN-134 (ADR-0088).
 */
const EXCEPTIONS: readonly string[] = ["ci.yml", "story-automerge.yml"];

/**
 * The contexts that name the event or a branch (`REF-GH-CONTEXTS`), which a condition may not
 * read (ADR-0088, decision 2); `\b` keeps `github.ref` from matching `github.repository`.
 */
const EVENT_OR_BRANCH =
  /github\.(?:event_name|ref_name|ref|base_ref|head_ref|event\.pull_request\.base\.ref)\b/u;

/** The start of a step's first key. */
const STEP = /^- /u;

/** The key of a condition, a job's or a step's. */
const CONDITION_KEY = "if:";

/**
 * The lines that continue a key's value: those after it, more indented than the key.
 *
 * @param lines - the workflow's lines
 * @param index - the key's line
 * @param column - the key's column
 * @returns the continuation lines, in order
 */
function continuedLines(
  lines: readonly string[],
  index: number,
  column: number,
): readonly string[] {
  const rest = lines.slice(index + 1);
  const end = rest.findIndex((next) => next.length - next.trimStart().length <= column);

  return end === -1 ? rest : rest.slice(0, end);
}

/**
 * The conditions of a workflow that read the event or a branch.
 *
 * @param file - the workflow's file name
 * @returns each one as `file:line: if: condition`, its continuation lines joined, in the file's
 * order
 */
function findingsOf(file: string): readonly string[] {
  const lines = readFileSync(join(WORKFLOWS, file), "utf8").split("\n");

  return lines.flatMap((line, index) => {
    const key = line.trimStart().replace(STEP, "");

    if (!key.startsWith(CONDITION_KEY)) {
      return [];
    }

    const continued = continuedLines(lines, index, line.length - key.length);
    const condition = [key.slice(CONDITION_KEY.length), ...continued]
      .map((part) => part.trim())
      .join(" ")
      .trim();

    if (!EVENT_OR_BRANCH.test(condition)) {
      return [];
    }

    return `${file}:${index + 1}: if: ${condition}`;
  });
}

describe("one workflow per purpose (ADR-0088)", () => {
  it("[F28.AC1] chooses no job nor step by the event or a branch, its trigger the only condition", () => {
    const findings = FILES.filter((file) => !EXCEPTIONS.includes(file)).flatMap((file) =>
      findingsOf(file),
    );

    expect(findings).toStrictEqual([]);
  });

  it("[F28.AC1] lists as exceptions only workflows that still choose by the event or a branch", () => {
    // A file listed that no longer breaks the rule, or no longer exists, leaves the list.
    const kept = EXCEPTIONS.filter(
      (file) => !FILES.includes(file) || findingsOf(file).length === 0,
    );

    expect(kept).toStrictEqual([]);
  });

  it("[F28.AC1] says each workflow's purpose in its first comment and its name", () => {
    const unnamed = FILES.filter((file) => {
      const text = readFileSync(join(WORKFLOWS, file), "utf8");

      return !text.startsWith("# ") || !/^name: \S/mu.test(text);
    });

    expect(unnamed).toStrictEqual([]);
  });
});

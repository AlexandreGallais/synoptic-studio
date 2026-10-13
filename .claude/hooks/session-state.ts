// SessionStart hook (ADR-0021): prints the project state derived from Git and the backlog files,
// so that a new or compacted session starts from facts rather than from memory.

// Temporal runs behind `--harmony-temporal` in Node.js 24: the hook command passes it.

import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync, rmSync, statSync } from "node:fs";
import { join } from "node:path";

/** Repository root (the hook lives in `.claude/hooks/`). */
const ROOT = join(import.meta.dirname, "..", "..");

/** Folder of the feature files. */
const FEATURES = join(ROOT, "docs", "backlog", "features");

/** Age after which a running call's file is left over from an interrupted or denied call. */
const STALE_MS = 86_400_000;

/** Statuses of finished stories. */
const FINISHED = new Set(["done", "dropped"]);

/** A row of a feature's story table. */
type StoryRow = {
  readonly id: string;
  readonly status: string;
  readonly title: string;
};

/**
 * Story rows of a feature file, in delivery order.
 *
 * @param text - feature file content
 * @returns one row per `| [ID](…) | type | title | status |` line
 */
export function storyRows(text: string): StoryRow[] {
  return text
    .matchAll(
      /^\|\s*\[(?<id>[A-Z]+-\d{3})\]\([^)]*\)\s*\|[^|]*\|\s*(?<title>[^|]*?)\s*\|\s*(?<status>[a-z-]+)\s*\|$/gmu,
    )
    .map((match) => ({
      id: group(match, "id"),
      status: group(match, "status"),
      title: group(match, "title"),
    }))
    .toArray();
}

/**
 * Named group of a regular expression match.
 *
 * @param match - result of `matchAll` on a pattern with named groups
 * @param name - identifier written as `(?<name>…)` in the pattern
 * @returns the captured text, or an empty string
 */
function group(match: RegExpExecArray, name: string): string {
  return match.groups?.[name] ?? "";
}

/**
 * State line of a started feature: what is done and what comes next.
 *
 * @param feature - feature file name
 * @param rows - its story rows
 * @returns a line, or `undefined` for a feature not started (no story finished nor in progress) or finished
 */
export function featureLine(feature: string, rows: readonly StoryRow[]): string | undefined {
  const done = rows.filter((row) => FINISHED.has(row.status)).length;
  const next = rows.find((row) => !FINISHED.has(row.status));
  // A first story is `in-progress` on its own branch only: the feature has started.
  const isStarted = done > 0 || rows.some((row) => row.status === "in-progress");

  return !isStarted || next === undefined
    ? undefined
    : `${feature.replace(/\.md$/u, "")}: ${String(done)}/${String(rows.length)} stories finished; next ${next.id} (${next.status}) — ${next.title}`;
}

/**
 * Output of a Git command in the repository.
 *
 * @param gitArguments - subcommand and its options, e.g. `["status", "--porcelain"]`
 * @returns trimmed standard output
 */
function git(gitArguments: readonly string[]): string {
  return execFileSync("git", gitArguments, { cwd: ROOT, encoding: "utf8" }).trim();
}

/**
 * Removes the files of calls that never ended: a call interrupted or denied never reaches the
 * `log-tool-call` hook, which removes its file.
 *
 * @param folder - `metrics/running` of the Git directory
 * @param now - time now, epoch milliseconds
 */
function removeStaleCalls(folder: string, now: number): void {
  const paths = (existsSync(folder) ? readdirSync(folder) : []).map((name) => join(folder, name));

  const stale = paths.filter((file) => now - statSync(file).mtimeMs > STALE_MS);

  for (const path of stale) {
    rmSync(path, { force: true });
  }
}

if (import.meta.main) {
  removeStaleCalls(
    join(git(["rev-parse", "--path-format=absolute", "--git-common-dir"]), "metrics", "running"),
    Temporal.Now.instant().epochMilliseconds,
  );

  const changed = git(["status", "--porcelain"])
    .split("\n")
    .filter((line) => line !== "").length;
  const files = readdirSync(FEATURES).filter((file) => /^E\d{2}-F\d{2}-.*\.md$/u.test(file));
  const features = files
    .map((file) => ({ file, rows: storyRows(readFileSync(join(FEATURES, file), "utf8")) }))
    .map(({ file, rows }) => featureLine(file, rows))
    .filter((line) => line !== undefined);

  process.stdout.write(
    [
      "Project state (SessionStart hook, ADR-0021):",
      `- branch ${git(["branch", "--show-current"])}, ${String(changed)} uncommitted file(s), last commit: ${git(["log", "-1", "--format=%h %s"])}`,
      ...(features.length === 0 ? ["- no feature started"] : features.map((line) => `- ${line}`)),
      "- a story is implemented only once the Product Owner has set it ready.",
      "",
    ].join("\n"),
  );
}

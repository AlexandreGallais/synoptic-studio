// The findings of `epic:check` read from logs and outputs (EN-055): the failed calls, the worktree
// isolation's refusals apart; the calls lost at a force-quit; the checks of a failed check:all,
// each named with its lane and its failing tests.

// Temporal runs behind `--harmony-temporal` in Node.js 24: the npm script passes it.

import { stripVTControlCharacters } from "node:util";

import { text } from "../.claude/hooks/status-json.ts";
import { readRunningCall } from "../.claude/hooks/status-place.ts";

/**
 * The lanes of `check:all`, as `tools/repo-tools/src/check_all.rs` runs them (EN-103): kept here
 * until `epic-check` is ported in turn.
 */
const LANES: readonly (readonly string[])[] = [
  ["lint"],
  ["test"],
  ["docs:build", "playground:build"],
  ["format:check", "lint:secrets", "deps:audit", "deps:unused", "deps:tools", "deps:outdated"],
  ["check:rust"],
];

/** One line of the check's table. */
export type Finding = {
  readonly detail: string;
  readonly isOk: boolean;
  readonly what: string;
};

/** A check of check:all's output and the lines it printed after its own. */
type CheckBlock = {
  readonly isFailed: boolean;
  readonly lines: readonly string[];
  readonly name: string;
};

/** A running call's file older than this is stale: one hour. */
const STALE_MS = 3_600_000;

/** Characters of an error kept to group the failed calls. */
const ERROR_TEXT = 60;

/** Share of failed calls accepted, in percent. */
const FAILED_PERCENT = 2;

/** Percent of a whole. */
const PERCENT = 100;

/** Error lines shown for the failed calls. */
const ERRORS_SHOWN = 5;

/** Characters of a failed check:all's output kept when no failing check is named. */
const OUTPUT_KEPT = 300;

/** Failing tests named per check, the others counted. */
const TESTS_SHOWN = 10;

/** Characters of a lost call's command shown. */
const SUMMARY_SHOWN = 60;

/** Characters of an ISO 8601 instant kept: date, hours and minutes. */
const MINUTE_TEXT = 16;

/**
 * Start of the error Claude Code returns when a worktree subagent's command leaves its worktree:
 * a refusal before the command runs, not a command that failed (549 of them in E15).
 */
const ISOLATION_REFUSAL = "This agent is isolated in the worktree";

/** A failing test as Vitest (`FAIL  file > suite > test`) or `cargo test` (`test x ... FAILED`) names it. */
const FAILING_TEST = /^\s*(?:FAIL\s+(?<vitest>.+?)|test (?<cargo>\S+) \.\.\. FAILED)\s*$/u;

/** A check's line in `check:all`'s output: `✓ lint 23.9 s` or `✗ test 26.2 s`. */
const CHECK_LINE = /^(?<mark>[✓✗]) (?<name>\S+) [\d.]+ s$/u;

/**
 * The output of check:all, cut at each check's line.
 *
 * @param lines - the output's lines, without terminal styles
 * @returns each check with the lines after it, lines before the first check left out
 */
function checkBlocks(lines: readonly string[]): readonly CheckBlock[] {
  const checks = lines
    .map((line, index) => ({ groups: CHECK_LINE.exec(line)?.groups ?? {}, index }))
    .filter(({ groups }) => "name" in groups);
  const ends = [...checks.slice(1).map(({ index }) => index), lines.length];

  return checks.map(({ groups, index }, rank) => ({
    isFailed: groups["mark"] === "✗",
    lines: lines.slice(index + 1, ends[rank]),
    name: groups["name"] ?? "",
  }));
}

/**
 * The failing tests a check printed, each once.
 *
 * @param lines - the check's output
 * @returns the tests' names, in order
 */
function failingTests(lines: readonly string[]): readonly string[] {
  return [
    ...new Set(
      lines
        .map((line) => FAILING_TEST.exec(line)?.groups)
        .filter((test) => test !== undefined)
        .map((test) => test["vitest"] ?? test["cargo"] ?? ""),
    ),
  ];
}

/**
 * The checks that failed in `check:all`'s output, each with its lane and its failing tests.
 *
 * @param output - what `check:all` printed: a line per check, a failed check's output after it
 * @returns lines such as `test (lane test): a.test.ts > adds`, empty when none failed
 */
export function failingChecks(output: string): readonly string[] {
  // Vitest may print a terminal's colors and styles: removed before reading the lines.
  return checkBlocks(stripVTControlCharacters(output).split("\n"))
    .filter((block) => block.isFailed)
    .map((block) => {
      const lane = LANES.find((checks) => checks.includes(block.name)) ?? [block.name];
      const tests = failingTests(block.lines);
      const more = tests.length > TESTS_SHOWN ? [`${String(tests.length - TESTS_SHOWN)} more`] : [];
      const named = [...tests.slice(0, TESTS_SHOWN), ...more];

      return `${block.name} (lane ${lane.join(" → ")})${named.length === 0 ? "" : `: ${named.join(", ")}`}`;
    });
}

/**
 * The calls whose start was written but whose end never was: Claude Code quit, or was forced to,
 * while they ran.
 *
 * @param calls - the running calls' files and how old each is, milliseconds
 * @returns a finding listing those over an hour old as lost, never a failure
 */
export function lostCalls(
  calls: readonly { readonly ageMs: number; readonly record: unknown }[],
): Finding {
  const lost = calls
    .filter((call) => call.ageMs > STALE_MS)
    .map((call) => readRunningCall(call.record))
    .toSorted((left, right) => left.start - right.start)
    .map(
      (call) =>
        `${Temporal.Instant.fromEpochMilliseconds(call.start).toString().slice(0, MINUTE_TEXT)} ${call.tool} ${call.summary.slice(0, SUMMARY_SHOWN)}`,
    );

  return {
    detail: lost.length === 0 ? "none" : `${String(lost.length)} lost: ${lost.join("; ")}`,
    isOk: true,
    what: "calls lost: started, never ended (a force-quit), over an hour old",
  };
}

/**
 * The calls that failed since a day, and their most frequent errors.
 *
 * @param records - parsed lines of the metrics log
 * @param since - first day, `YYYY-MM-DD`
 * @returns a finding: how many failed out of how many, the frequent errors
 */
export function failedCalls(records: readonly unknown[], since: string): Finding {
  const all = records.filter((record) => text(record, "time") >= since);
  const refusals = all.filter((record) => text(record, "error").startsWith(ISOLATION_REFUSAL));
  const recent = all.filter((record) => !text(record, "error").startsWith(ISOLATION_REFUSAL));
  const failed = recent.filter((record) => text(record, "error") !== "");
  const errors = new Map<string, number>();

  for (const record of failed) {
    const key = `${text(record, "tool")}: ${text(record, "error").slice(0, ERROR_TEXT)}`;

    errors.set(key, (errors.get(key) ?? 0) + 1);
  }

  const frequent = errors
    .entries()
    .toArray()
    .toSorted((left, right) => right[1] - left[1])
    .slice(0, ERRORS_SHOWN)
    .map(([error, times]) => `${String(times)} × ${error}`);

  return {
    detail: [
      `${String(failed.length)} of ${String(recent.length)}`,
      ...frequent,
      `apart: ${String(refusals.length)} refused by the worktree isolation`,
    ].join("; "),
    isOk: failed.length * PERCENT <= recent.length * FAILED_PERCENT,
    what: "calls failed during the epic (2 % at most)",
  };
}

/**
 * The finding of check:all: its last line when it passed, else its failing checks.
 *
 * @param checks - whether check:all passed, and its output
 * @param checks.isOk - whether it passed
 * @param checks.out - what it printed
 * @returns the failing checks, their lanes and tests; the output's end when none is named
 */
export function checkAllFinding(checks: { readonly isOk: boolean; readonly out: string }): Finding {
  const failing = failingChecks(checks.out);
  const said = failing.length === 0 ? checks.out.slice(-OUTPUT_KEPT) : failing.join("; ");

  return {
    detail: checks.isOk ? (checks.out.split("\n").at(-1) ?? "") : said,
    isOk: checks.isOk,
    what: "check:all (code, site, playground, dependencies)",
  };
}

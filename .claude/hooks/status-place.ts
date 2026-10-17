// Where the work stands, for the status line (docs/tooling/metrics.md): the epic, feature and
// story of the branch checked out, read from the backlog's file names, and the tool call running.

import { count, text, textOr } from "./status-json.ts";

import type { BackgroundTask } from "./status-tasks.ts";

/** A tool call started and not finished yet, as the `track-tool-call` hook writes it. */
export type RunningCall = {
  /** Who runs it: `main`, or the type of the subagent. */
  readonly agent: string;
  readonly description: string;
  readonly start: number;
  readonly summary: string;
  readonly tool: string;
};

/** Story ids as they appear in branch names. */
const STORY = /\b(?<id>(?:us|en|sp|chk|aud|val|nam|rev|ret)-\d{3})\b/u;

/** Feature branches, `feature/f14-curves`. */
const FEATURE = /^feature\/(?<id>f\d{2})-/u;

/** Characters of a running command shown. */
const COMMAND_SHOWN = 60;

/**
 * Words of a file name's slug.
 *
 * @param slug - kebab-case words, `turn-a-shape`
 * @returns the words, `turn a shape`
 */
function words(slug: string): string {
  return slug.replaceAll("-", " ");
}

/**
 * Backlog item a branch works on.
 *
 * @param branch - branch checked out, e.g. `feat/us-018-turn-a-shape` or `feature/f04-free-rotation`
 * @returns `US-018`, `F04`, or an empty string
 */
function itemOf(branch: string): string {
  const match = STORY.exec(branch) ?? FEATURE.exec(branch);

  return (match?.[1] ?? "").toUpperCase();
}

/**
 * Epic, feature and story of a branch, from the names of the backlog files.
 *
 * @param branch - branch checked out, e.g. `feat/us-018-turn-a-shape`
 * @param files - names of the story and feature files, `E01-F04-US-018-turn-a-shape.md`
 * @returns `E01 › F04 › US-018 turn a shape`, the branch itself when it names no item
 */
export function placeOf(branch: string, files: readonly string[]): string {
  const id = itemOf(branch);
  const file = files.find((name) => id !== "" && name.includes(`-${id}-`)) ?? "";
  const groups =
    /^(?<epic>E\d{2})-(?:(?<feature>F\d{2})-)?(?:[A-Z]+-\d{3}-)?(?<slug>.+)\.md$/u.exec(
      file,
    )?.groups;

  return groups === undefined
    ? branch
    : [groups["epic"], groups["feature"], `${id} ${words(groups["slug"] ?? "")}`]
        .filter((part) => part !== undefined && part !== `${id} `)
        .join(" › ");
}

/**
 * A running call read back from its file.
 *
 * @param record - parsed JSON written by the `track-tool-call` hook
 * @returns its tool, summary, description and start, empty or 0 where the file lacks them
 */
export function readRunningCall(record: unknown): RunningCall {
  return {
    agent: textOr(record, "agent", "main"),
    description: text(record, "description"),
    start: count(record, "start"),
    summary: text(record, "summary"),
    tool: text(record, "tool"),
  };
}

/**
 * Line of the status line for the oldest call still running.
 *
 * @param calls - calls started and not finished yet
 * @param now - time now, epoch milliseconds
 * @param duration - formats the time since the call started
 * @returns `▶ Bash · Run the checks — 12 s`, or an empty string when nothing runs
 */
export function runningLine(
  calls: readonly RunningCall[],
  now: number,
  duration: (milliseconds: number) => string,
): string {
  const oldest = calls.toSorted((left, right) => left.start - right.start)[0];

  return oldest === undefined ? "" : `▶ ${callText(oldest)} — ${duration(now - oldest.start)}`;
}

/**
 * A call in a few words: its tool, and its description or the start of its command.
 *
 * @param call - a call running
 * @returns `Bash · Run the checks`
 */
function callText(call: RunningCall): string {
  const what = call.description === "" ? call.summary.slice(0, COMMAND_SHOWN) : call.description;

  return `${call.tool} · ${what}`;
}

/**
 * One line per background task: what it is, for how long it has run, and, for a subagent, the
 * call it is making now (the latest one started by a subagent of its type); a message whose result
 * is still to come is left out.
 *
 * @param tasks - background tasks still running
 * @param calls - calls running, of the main agent and of the subagents
 * @param context - time now and the duration format
 * @param context.duration - formats the time since a task started
 * @param context.now - time now, epoch milliseconds
 * @returns `↳ agent auditor: Review EN-030 — 4 min · now Bash · List the files`, one per task
 */
export function backgroundLines(
  tasks: readonly BackgroundTask[],
  calls: readonly RunningCall[],
  context: { readonly duration: (milliseconds: number) => string; readonly now: number },
): readonly string[] {
  return tasks
    .filter((task) => task.kind !== "message")
    .map((task) => {
      const type = task.label.split(":", 1)[0] ?? "";
      const current = calls
        .filter((call) => task.kind === "agent" && call.agent === type)
        .toSorted((left, right) => right.start - left.start)[0];
      const now = current === undefined ? "" : ` · now ${callText(current)}`;

      return `↳ ${task.kind} ${task.label} — ${context.duration(context.now - task.start)}${now}`;
    });
}

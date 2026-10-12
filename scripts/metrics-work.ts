// Work units of the metrics report (docs/tooling/metrics.md): what the agent's requests and tool
// calls were spent on — a story, a feature branch, a task commit — with active time and tokens.

/** Something the agent did at a moment: a model request or a tool call. */
export type Moment = {
  readonly branch: string;
  readonly durationMs: number;
  readonly isFailure: boolean;
  readonly isToolCall: boolean;
  readonly outputTokens: number;
  readonly session: string;
  readonly time: number;
  readonly tokens: number;
};

/** A commit, with the tasks its `Refs:` trailer names. */
export type TaskCommit = {
  readonly hash: string;
  readonly subject: string;
  readonly tasks: readonly string[];
  readonly time: number;
};

/** Totals of one unit of work. */
export type WorkStats = {
  readonly activeMs: number;
  readonly calls: number;
  readonly failures: number;
  readonly key: string;
  readonly outputTokens: number;
  readonly tokens: number;
  readonly toolMs: number;
};

/** Characters of a commit hash shown, as `git log --oneline` shows them. */
const SHORT_HASH = 7;

/** Fields of a `git log` record: hash, author date, subject, body. */
const LOG_FIELDS = 4;

/** A pause longer than this is not work: the gap counts for this much at most (5 minutes). */
export const IDLE_MS = 300_000;

/**
 * Unit of work of a Git branch: its story, its feature, or the branch itself.
 *
 * @param branch - branch name, e.g. `feat/us-018-turn-a-shape` or `feature/f04-free-rotation`
 * @returns `US-018`, `F04`, or the branch name (`main`, `develop`…)
 */
export function unitOf(branch: string): string {
  const found = /\b(?:us|en|sp|chk|aud|val|rev|ret)-\d{3}\b|(?<=^feature\/)f\d{2}(?=-)/u.exec(
    branch,
  );

  if (found !== null) {
    return found[0].toUpperCase();
  }

  return branch === "" ? "(no branch)" : branch;
}

/**
 * Active time of a series of moments: the gaps between them, each capped at `IDLE_MS`.
 *
 * @param times - moments, epoch milliseconds, in any order
 * @returns milliseconds of work
 */
export function activeMs(times: readonly number[]): number {
  const sorted = times.toSorted((left, right) => left - right);

  return sorted
    .slice(1)
    .reduce((sum, time, index) => sum + Math.min(time - (sorted[index] ?? time), IDLE_MS), 0);
}

/**
 * Totals of moments grouped by a key.
 *
 * @param moments - the agent's requests and tool calls
 * @param keyOf - group of a moment
 * @returns one entry per group, in no particular order
 */
export function workStats(
  moments: readonly Moment[],
  keyOf: (moment: Moment) => string,
): readonly WorkStats[] {
  return [...Map.groupBy(moments, keyOf)].map(([key, group]) => {
    const calls = group.filter((moment) => moment.isToolCall);

    return {
      activeMs: activeMs(group.map((moment) => moment.time)),
      calls: calls.length,
      failures: calls.filter((moment) => moment.isFailure).length,
      key,
      outputTokens: group.reduce((sum, moment) => sum + moment.outputTokens, 0),
      tokens: group.reduce((sum, moment) => sum + moment.tokens, 0),
      toolMs: calls.reduce((sum, moment) => sum + moment.durationMs, 0),
    };
  });
}

/**
 * Task a moment worked towards: the first commit of its story made at or after it.
 *
 * @param moment - a request or a tool call
 * @param commits - task commits, oldest first
 * @returns `<task ids> <short hash> <subject>`, or the unit when no later commit of it exists
 */
export function taskOf(moment: Moment, commits: readonly TaskCommit[]): string {
  const unit = unitOf(moment.branch);
  const commit = commits.find(
    (candidate) =>
      candidate.time >= moment.time && candidate.tasks.some((task) => task.startsWith(`${unit}.`)),
  );

  return commit === undefined
    ? `${unit} (no task commit)`
    : `${commit.tasks.join(", ")} ${commit.hash.slice(0, SHORT_HASH)} ${commit.subject}`;
}

/**
 * Task ids a commit message names in its `Refs:` lines.
 *
 * @param body - commit message body
 * @returns ids such as `US-018.T2`
 */
export function taskIds(body: string): readonly string[] {
  return body
    .matchAll(/^Refs:(?<ids>.*)$/gmu)
    .flatMap((match) => (match.groups?.["ids"] ?? "").split(","))
    .map((id) => id.trim())
    .filter((id) => /^[A-Z]+-\d{3}\.T\d+$/u.test(id))
    .toArray();
}

/**
 * Task commits of a `git log` listing, each commit once.
 *
 * A commit rebased onto another branch keeps its author date and subject: those two identify it.
 *
 * @param log - records `hash␟author date (ISO 8601)␟subject␟body␞`
 * @returns commits naming at least one task (`US-018.T2`), oldest first
 */
export function taskCommits(log: string): readonly TaskCommit[] {
  const commits = log
    .split("\u{1E}")
    .map((record) => record.trim().split("\u{1F}"))
    .filter((fields) => fields.length === LOG_FIELDS)
    .map(([hash = "", date = "", subject = "", body = ""]) => ({
      hash,
      subject,
      tasks: taskIds(body),
      time: Temporal.Instant.from(date).epochMilliseconds,
    }))
    .filter((commit) => commit.tasks.length > 0);
  const unique = new Map(
    commits.map((commit) => [`${String(commit.time)} ${commit.subject}`, commit]),
  );

  return unique
    .values()
    .toArray()
    .toSorted((left, right) => left.time - right.time);
}

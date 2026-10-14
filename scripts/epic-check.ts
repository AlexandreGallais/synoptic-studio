// The full check of what an epic produced, at the start and at the end of every epic (RET-002, the
// agent's tools in docs/tooling/agent-tools.md): every check of the code and the site, the
// branches and pull requests left, the code scanning alerts on GitHub, the calls that failed
// during the epic and why, calls left running, the backlog of the epic, the private records
// saved. Prints a Markdown table to copy into the retrospective's record. Run by `npm run epic:check -- <epic> <first day>`.

// Temporal runs behind `--harmony-temporal` in Node.js 24: the npm script passes it.

import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

import { text } from "../.claude/hooks/status-json.ts";

/** One line of the check's table. */
export type Finding = {
  readonly detail: string;
  readonly isOk: boolean;
  readonly what: string;
};

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/** A running call's file older than this is stale: one hour. */
const STALE_MS = 3_600_000;

/** Characters of an error kept to group the failed calls. */
const ERROR_TEXT = 60;

/** Share of failed calls accepted, in percent. */
const FAILED_PERCENT = 2;

/** Percent of a whole. */
const PERCENT = 100;

/** Characters of a failed command's error kept. */
const OUTPUT_KEPT = 300;

/** Error lines shown for the failed calls. */
const ERRORS_SHOWN = 5;

/**
 * The calls that failed since a day, and their most frequent errors.
 *
 * @param records - parsed lines of the metrics log
 * @param since - first day, `YYYY-MM-DD`
 * @returns a finding: how many failed out of how many, the frequent errors
 */
export function failedCalls(records: readonly unknown[], since: string): Finding {
  const recent = records.filter((record) => text(record, "time") >= since);
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
    detail: [`${String(failed.length)} of ${String(recent.length)}`, ...frequent].join("; "),
    isOk: failed.length * PERCENT <= recent.length * FAILED_PERCENT,
    what: "calls failed during the epic (2 % at most)",
  };
}

/**
 * Branches left besides `main`, `develop` and the one checked out.
 *
 * @param branches - local branches and those on GitHub, `origin/…`
 * @param current - the branch checked out
 * @returns a finding listing the others
 */
export function leftBranches(branches: readonly string[], current: string): Finding {
  const kept = new Set([
    "main",
    "develop",
    "origin/main",
    "origin/develop",
    "origin/HEAD",
    "origin",
    current,
    `origin/${current}`,
  ]);
  const others = branches.filter((branch) => !kept.has(branch));

  return {
    detail: others.length === 0 ? "only main, develop and the retrospective's" : others.join(", "),
    isOk: others.length === 0,
    what: "branches left",
  };
}

/**
 * Why a command failed: its own output — check:all names the checks that failed — else the error.
 *
 * @param error - what `execFileSync` threw
 * @returns the end of that text
 */
function failureText(error: unknown): string {
  const output: unknown =
    typeof error === "object" && error !== null ? Reflect.get(error, "stdout") : undefined;
  const said = typeof output === "string" && output !== "" ? output : String(error);

  return said.trim().slice(-OUTPUT_KEPT);
}

/**
 * Lines of a command's output, or its error.
 *
 * @param command - the program
 * @param options - its arguments
 * @returns its trimmed standard output and whether it succeeded
 */
function run(
  command: string,
  options: readonly string[],
): { readonly isOk: boolean; readonly out: string } {
  try {
    return {
      isOk: true,
      out: execFileSync(command, options, { cwd: ROOT, encoding: "utf8", stdio: "pipe" }).trim(),
    };
  } catch (error) {
    return { isOk: false, out: failureText(error) };
  }
}

/**
 * The parsed lines of the metrics log.
 *
 * @param folder - `.git/metrics`
 * @returns them, none when the log is missing
 */
function metricsLog(folder: string): readonly unknown[] {
  const log = join(folder, "tool-calls.jsonl");

  return existsSync(log)
    ? readFileSync(log, "utf8")
        .split("\n")
        .filter((line) => line.startsWith("{"))
        .map((line): unknown => JSON.parse(line))
    : [];
}

/**
 * Backlog items of an epic not `done`, its retrospective left out.
 *
 * @param epic - `E14`
 * @returns a finding listing them
 */
function openItems(epic: string): Finding {
  const open = ["features", "stories"].flatMap((folder) =>
    readdirSync(join(ROOT, "docs", "backlog", folder))
      .filter((name) => name.startsWith(`${epic}-`) && !name.includes("-RET-"))
      .filter(
        (name) =>
          !/^status: done$/mu.test(
            readFileSync(join(ROOT, "docs", "backlog", folder, name), "utf8"),
          ),
      )
      .map((name) => name.replace(/\.md$/u, "")),
  );

  return {
    detail: open.length === 0 ? "every feature and story done" : open.join(", "),
    isOk: open.length === 0,
    what: `backlog of ${epic}`,
  };
}

if (import.meta.main) {
  const [epic = "", since = ""] = process.argv.slice(2);
  const common = run("git", ["rev-parse", "--path-format=absolute", "--git-common-dir"]).out;
  const metrics = join(common, "metrics");
  const now = Temporal.Now.instant().epochMilliseconds;
  const running = join(metrics, "running");
  const stale = (existsSync(running) ? readdirSync(running) : []).filter(
    (name) => now - statSync(join(running, name)).mtimeMs > STALE_MS,
  );
  const checks = run("npm", ["run", "--silent", "check:all"]);
  const prs = run("gh", [
    "pr",
    "list",
    "--json",
    "number,title",
    "-q",
    String.raw`.[] | "#\(.number) \(.title)"`,
  ]);
  const issues = run("gh", ["issue", "list", "--json", "number", "-q", "length"]);
  const alerts = run("gh", [
    "api",
    "repos/{owner}/{repo}/code-scanning/alerts?state=open&per_page=100",
    "-q",
    String.raw`.[] | "\(.rule.id) \(.most_recent_instance.location.path) on \(.most_recent_instance.ref)"`,
  ]);
  const records = run("git", ["-C", join(ROOT, "records"), "status", "--short"]);
  const recordsLog = run("git", ["-C", join(ROOT, "records"), "log", "-1", "--format=%cs %s"]);
  const findings: readonly Finding[] = [
    {
      detail: checks.out.split("\n").at(-1) ?? "",
      isOk: checks.isOk,
      what: "check:all (code, site, playground, dependencies)",
    },
    leftBranches(
      run("git", ["branch", "-a", "--format=%(refname:short)"]).out.split("\n"),
      run("git", ["branch", "--show-current"]).out,
    ),
    {
      detail: prs.out === "" ? "none" : prs.out.replaceAll("\n", "; "),
      isOk: true,
      what: "pull requests open",
    },
    { detail: issues.out, isOk: issues.out === "0", what: "issues open" },
    {
      detail: alerts.out === "" ? "none" : alerts.out.replaceAll("\n", "; "),
      isOk: alerts.isOk && alerts.out === "",
      what: "code scanning alerts open (GitHub, Security)",
    },
    failedCalls(metricsLog(metrics), since),
    {
      detail: stale.length === 0 ? "none" : stale.join(", "),
      isOk: stale.length === 0,
      what: "calls left running over an hour",
    },
    openItems(epic),
    {
      detail: `${records.out === "" ? "nothing unsaved" : "unsaved changes"}; last save ${recordsLog.out}`,
      isOk: records.isOk && records.out === "",
      what: "private records",
    },
    {
      detail: existsSync(join(metrics, "golden.json")) ? "present" : "none written yet",
      isOk: true,
      what: "golden file of refactors",
    },
  ];
  const rows = findings.map(
    (finding) =>
      `| ${finding.what} | ${finding.isOk ? "ok" : "**to look at**"} | ${finding.detail.replaceAll("|", String.raw`\|`)} |`,
  );

  process.stdout.write(
    `${["| Check | Result | Detail |", "| --- | --- | --- |", ...rows].join("\n")}\n`,
  );

  process.exitCode = findings.every((finding) => finding.isOk) ? 0 : 1;
}

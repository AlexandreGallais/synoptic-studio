// The full check of what an epic produced, at the start and at the end of every epic (RET-002, the
// agent's tools in docs/tooling/agent-tools.md): every check of the code and the site, the
// branches and pull requests left, the code scanning alerts on GitHub, the calls that failed
// during the epic and why (the worktree isolation's refusals apart), calls lost at a force-quit,
// the backlog of the epic, the private records saved. A failed check:all is named by its failing
// checks, their lanes and their failing tests (EN-055). Prints a Markdown table to copy into the
// retrospective's record. Run by `npm run epic:check -- <epic> <first day>`.

// Temporal runs behind `--harmony-temporal` in Node.js 24: the npm script passes it.

import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

import { checkAllFinding, failedCalls, lostCalls } from "./epic-check-findings.ts";

import type { Finding } from "./epic-check-findings.ts";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

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
 * @returns that text, whole: `check:all`'s is read for its failing tests
 */
function failureText(error: unknown): string {
  const output: unknown =
    typeof error === "object" && error !== null ? Reflect.get(error, "stdout") : undefined;
  const said = typeof output === "string" && output !== "" ? output : String(error);

  return said.trim();
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
  const runningCalls = (existsSync(running) ? readdirSync(running) : []).map((name) => {
    const record: unknown = JSON.parse(readFileSync(join(running, name), "utf8"));

    return { ageMs: now - statSync(join(running, name)).mtimeMs, record };
  });
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
    checkAllFinding(checks),
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
    lostCalls(runningCalls),
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

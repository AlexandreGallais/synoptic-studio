import { spawnSync } from "node:child_process";
import { chmodSync, existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { afterEach, describe, expect, it } from "vitest";

/** The folder of the workflows. */
const WORKFLOWS = join(import.meta.dirname, "..", ".github", "workflows");

/** Text of the story auto-merge workflow, its comments left out. */
const WORKFLOW = readFileSync(join(WORKFLOWS, "story-automerge.yml"), "utf8")
  .split("\n")
  .filter((line) => !line.trimStart().startsWith("#"))
  .join("\n");

/** Indentation of a step's script lines under `run: |`. */
const SCRIPT_INDENT = " ".repeat(10);

/** Temporary folders to remove after each test. */
const FOLDERS: string[] = [];

/**
 * The script of a step, as GitHub runs it.
 *
 * @param name - the start of the step's name
 * @returns the script's lines without their indentation, or an empty text when the step is missing
 */
function scriptOf(name: string): string {
  const step = WORKFLOW.slice(WORKFLOW.indexOf(`- name: ${name}`));
  const lines = step.slice(step.indexOf("run: |\n") + "run: |\n".length).split("\n");
  const end = lines.findIndex((line) => !line.startsWith(SCRIPT_INDENT));

  return lines
    .slice(0, end)
    .map((line) => line.slice(SCRIPT_INDENT.length))
    .join("\n");
}

/**
 * Whether the check lets a pull request through, its script run by Bash as GitHub's runner does.
 *
 * @param base - the branch the pull request goes into
 * @param branch - the pull request's branch
 * @returns whether the script exits 0
 */
function passes(base: string, branch: string): boolean {
  const run = spawnSync("bash", ["-e", "-c", scriptOf("Check the branch")], {
    env: { BASE: base, BRANCH: branch, PATH: process.env["PATH"] ?? "" },
  });

  return run.status === 0;
}

/** What a run of the merge step did: its exit status, its output and the `gh` commands it ran. */
type MergeRun = {
  readonly status: number | null;
  readonly output: string;
  readonly calls: readonly string[];
};

/**
 * What the stub `gh` answers: the exit status of each `gh pr merge`, the state `gh pr view`
 * prints at each call, in turn, nothing once they run out, and the exit status of
 * `gh workflow run`.
 */
type GhStub = {
  readonly mergeStatus: number;
  readonly states: readonly string[];
  readonly dispatchStatus: number;
};

/** The pull request the merge step is given. */
const PR_URL = "https://github.com/o/r/pull/7";

/**
 * Runs the merge step's script by Bash, `gh` and `sleep` replaced by stubs: `gh pr merge` exits
 * with the status given, `gh pr view` prints the next state of the list, `gh workflow run` exits
 * with its status, printing GitHub's refusal when it fails, each call is logged.
 *
 * @param base - the branch the pull request goes into
 * @param stub - what the stub `gh` answers
 * @returns the run's status, output and `gh` calls
 */
function merge(base: string, stub: GhStub): MergeRun {
  const { dispatchStatus, mergeStatus, states } = stub;
  const folder = mkdtempSync(join(tmpdir(), "story-automerge-"));
  const log = join(folder, "calls");

  FOLDERS.push(folder);

  // The stub keeps its log and its count of `gh pr view` calls beside itself.
  writeFileSync(
    join(folder, "gh"),
    [
      "#!/bin/bash",
      'folder=$(dirname "$0")',
      'echo "gh $*" >> "$folder/calls"',
      `if [ "$1 $2" = "pr merge" ]; then exit ${String(mergeStatus)}; fi`,
      'if [ "$1 $2" = "pr view" ]; then',
      '  count=$(( $(cat "$folder/views" 2>/dev/null || echo 0) + 1 ))',
      '  echo "$count" > "$folder/views"',
      `  echo "${states.join(" ")}" | cut -d " " -f "$count"`,
      "fi",
      'if [ "$1 $2" = "workflow run" ]; then',
      `  [ ${String(dispatchStatus)} -eq 0 ] || echo "could not find any workflows named $3" >&2`,
      `  exit ${String(dispatchStatus)}`,
      "fi",
      "",
    ].join("\n"),
  );

  writeFileSync(join(folder, "sleep"), "#!/bin/sh\nexit 0\n");
  chmodSync(join(folder, "gh"), 0o755);
  chmodSync(join(folder, "sleep"), 0o755);

  // The step's variables, named as GitHub's runner and the workflow's `env:` name them.
  const environment: NodeJS.ProcessEnv = Object.fromEntries([
    ["BASE", base],
    ["GITHUB_REPOSITORY", "o/r"],
    ["PATH", `${folder}:${process.env["PATH"] ?? ""}`],
    ["PR_URL", PR_URL],
  ]);
  const run = spawnSync("bash", ["-e", "-c", scriptOf("Merge")], {
    encoding: "utf8",
    env: environment,
  });
  const calls = existsSync(log)
    ? readFileSync(log, "utf8")
        .split("\n")
        .filter((line) => line !== "")
    : [];

  return { calls, output: `${run.stdout}${run.stderr}`, status: run.status };
}

afterEach(() => {
  for (const folder of FOLDERS.splice(0)) {
    rmSync(folder, { force: true, recursive: true });
  }
});

describe("story auto-merge (ADR-0086)", () => {
  it("[F22.AC4] merges into a feature branch or develop, always by a merge commit", () => {
    expect(WORKFLOW).toContain("startsWith(github.event.pull_request.base.ref, 'feature/')");
    expect(WORKFLOW).toContain("github.event.pull_request.base.ref == 'develop'");
    expect(WORKFLOW).toContain(`gh pr merge "$PR_URL" --auto --merge`);
    expect(WORKFLOW).not.toMatch(/--rebase|\$METHOD/u);
  });

  it("[F22.AC4] merges only a labelled, ready pull request of the owner", () => {
    expect(WORKFLOW).toContain("contains(github.event.pull_request.labels.*.name, 'autonomous')");
    expect(WORKFLOW).toContain("github.event.pull_request.author_association == 'OWNER'");
    expect(WORKFLOW).toContain("!github.event.pull_request.draft");
  });

  it("[F22.AC4] lets a story into its feature, and a feature or a review into develop", () => {
    const feature = "feature/f22-release-by-a-button";

    expect(passes(feature, "chore/en-122-one-story-at-a-time-into-develop")).toBe(true);
    expect(passes(feature, "feat/us-061-font-list")).toBe(true);
    expect(passes(feature, "feat/val-023-validate-f22")).toBe(true);
    expect(passes(feature, "docs/aud-024-audit-f22")).toBe(true);
    expect(passes("develop", feature)).toBe(true);
    expect(passes("develop", "docs/rev-006-review")).toBe(true);
  });

  it("[F22.AC4] still refuses a branch that is neither a story's nor a feature's", () => {
    const feature = "feature/f22-release-by-a-button";

    // A story goes into its feature, never straight into develop; a review into develop only.
    expect(passes("develop", "chore/en-122-one-story-at-a-time-into-develop")).toBe(false);
    expect(passes(feature, "docs/rev-006-review")).toBe(false);
    expect(passes(feature, "feature/f21-other")).toBe(false);
    expect(passes(feature, "chore/tidy-up")).toBe(false);
    expect(passes(feature, "chore/en-12-short")).toBe(false);
    expect(passes("develop", "docs/f21-refinement")).toBe(false);
    // No epic has a closing since ADR-0086: a naming pass or a retrospective is no longer merged.
    expect(passes("develop", "refactor/nam-004-naming-pass")).toBe(false);
    expect(passes("develop", "docs/ret-006-retrospective")).toBe(false);
    expect(passes("develop", "feature/f2-short")).toBe(false);
    expect(passes("develop", "dependabot/npm_and_yarn/vite-8.3.4")).toBe(false);
  });
});

describe("the features' auto-merge starts the develop CI (ADR-0088, decision 5)", () => {
  /** The merge step's two calls of an attempt that merges. */
  const attempt = [
    `gh pr merge ${PR_URL} --auto --merge`,
    `gh pr view ${PR_URL} --json state --jq .state`,
  ];

  /** The dispatch of the develop CI on develop (REF-GH-CLI-WORKFLOW-RUN). */
  const dispatch = "gh workflow run develop-ci.yml --ref develop --repo o/r";

  it("[F30.AC1] dispatches the develop CI once its merge into develop reads MERGED", () => {
    // Mergeable at once, merged by `--auto` (REF-GH-CLI-MERGE): one attempt, then the dispatch.
    const run = merge("develop", { dispatchStatus: 0, mergeStatus: 0, states: ["MERGED"] });

    expect(run.status).toBe(0);
    expect(run.calls).toStrictEqual([...attempt, dispatch]);
    expect(run.output).not.toContain("::warning");
  });

  it("[F30.AC1] warns naming the cause and passes when the dispatch is refused, the merge done", () => {
    // GitHub dispatches a workflow only once its file is on the default branch, main
    // (REF-GH-WORKFLOW-DISPATCH): until then `gh workflow run` fails after the merge.
    const run = merge("develop", { dispatchStatus: 1, mergeStatus: 0, states: ["MERGED"] });

    expect(run.status).toBe(0);
    expect(run.calls).toStrictEqual([...attempt, dispatch]);
    expect(run.output).toContain("could not find any workflows named develop-ci.yml");

    expect(run.output).toMatch(
      /^::warning::The develop CI was not dispatched: .*develop-ci\.yml is not yet on main, the default branch/mu,
    );
  });

  it("[F30.AC1] still fails, dispatching nothing, when the merge is not done and the dispatch would be refused", () => {
    const run = merge("develop", {
      dispatchStatus: 1,
      mergeStatus: 0,
      states: ["OPEN", "OPEN", "OPEN", "OPEN", "OPEN"],
    });

    expect(run.status).toBe(1);
    expect(run.calls).not.toContain(dispatch);
    expect(run.output).not.toContain("::warning");
  });

  it("[F30.AC1] waits for the merge `--auto` only enabled, dispatching once", () => {
    // OPEN twice, the auto-merge enabled but not done: the third attempt reads MERGED, 3 × 2 calls.
    const run = merge("develop", {
      dispatchStatus: 0,
      mergeStatus: 0,
      states: ["OPEN", "OPEN", "MERGED"],
    });

    expect(run.status).toBe(0);
    expect(run.calls).toStrictEqual([...attempt, ...attempt, ...attempt, dispatch]);
  });

  it("[F30.AC1] reads the state after a failing gh pr merge, the pull request merged meanwhile", () => {
    // `gh pr merge` refuses a pull request already merged: the state read decides.
    const run = merge("develop", { dispatchStatus: 0, mergeStatus: 1, states: ["OPEN", "MERGED"] });

    expect(run.status).toBe(0);
    expect(run.calls).toStrictEqual([...attempt, ...attempt, dispatch]);
  });

  it("[F30.AC1] fails naming the pull request, its auto-merge disabled, when it is not merged into develop after its retries", () => {
    // Five attempts, none reading MERGED, the last state empty: 5 × 2 calls, no dispatch, then the
    // auto-merge `--auto` enabled is disabled: GitHub's later merge, by the GITHUB_TOKEN's actor,
    // would start no develop CI (REF-GH-TOKEN-TRIGGERS); a person's merge pushes and starts it.
    const run = merge("develop", {
      dispatchStatus: 0,
      mergeStatus: 0,
      states: ["OPEN", "OPEN", "OPEN", "OPEN"],
    });

    expect(run.status).toBe(1);

    expect(run.calls).toStrictEqual([
      ...Array.from({ length: 5 }, () => attempt).flat(),
      `gh pr merge ${PR_URL} --disable-auto`,
    ]);

    expect(run.output).toContain(`${PR_URL} is not merged after 5 attempts`);
    expect(run.output).toContain("auto-merge disabled");
  });

  it("[F30.AC1] still fails, naming the pull request, when its auto-merge cannot be disabled", () => {
    // Every `gh pr merge` fails, `--disable-auto` included: the step still fails, and says so.
    const run = merge("develop", {
      dispatchStatus: 0,
      mergeStatus: 1,
      states: ["OPEN", "OPEN", "OPEN", "OPEN", "OPEN"],
    });

    expect(run.status).toBe(1);
    expect(run.calls.at(-1)).toBe(`gh pr merge ${PR_URL} --disable-auto`);
    expect(run.output).toContain(`The auto-merge of ${PR_URL} could not be disabled.`);
    expect(run.output).toContain(`${PR_URL} is not merged after 5 attempts`);
  });

  it("[F30.AC1] keeps the auto-merge of a story not merged into its feature branch after its retries", () => {
    // No develop CI follows a merge into a feature branch: GitHub's later merge is kept.
    const run = merge("feature/f30-a-ci-on-develop-that-blocks-nothing", {
      dispatchStatus: 0,
      mergeStatus: 0,
      states: ["OPEN", "OPEN", "OPEN", "OPEN", "OPEN"],
    });

    expect(run.status).toBe(1);
    expect(run.calls).toStrictEqual(Array.from({ length: 5 }, () => attempt).flat());
    expect(run.output).toContain(`${PR_URL} is not merged after 5 attempts`);
  });

  it("[F30.AC1] dispatches nothing after a merge into a feature branch", () => {
    const run = merge("feature/f30-a-ci-on-develop-that-blocks-nothing", {
      dispatchStatus: 0,
      mergeStatus: 0,
      states: ["MERGED"],
    });

    expect(run.status).toBe(0);
    expect(run.calls).toStrictEqual(attempt);
  });

  it("[F30.AC1] dispatches with actions: write added and nothing else, a workflow that takes it", () => {
    const file = join(WORKFLOWS, "develop-ci.yml");

    expect(WORKFLOW).toContain(
      "\npermissions:\n  contents: write\n  pull-requests: write\n  actions: write\n\njobs:\n",
    );

    expect(WORKFLOW.match(/gh workflow run /gu)).toHaveLength(1);

    // A dispatch runs only a workflow with that trigger (REF-GH-CLI-WORKFLOW-RUN).
    expect(existsSync(file) ? readFileSync(file, "utf8") : "").toContain(
      "\n  workflow_dispatch:\n",
    );
  });

  it("[F30.AC1] starts only on a pull request's label or opening, so a person's merge dispatches nothing", () => {
    // A merge by a person is pushed to develop, which starts the develop CI once by itself.
    expect(WORKFLOW).toContain(
      "\non:\n  pull_request:\n    types: [labeled, opened, reopened, ready_for_review]\n\n",
    );
  });
});

import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/** The folder of the workflows. */
const WORKFLOWS = join(ROOT, ".github", "workflows");

/** The file name of each workflow, sorted. */
const FILES: readonly string[] = readdirSync(WORKFLOWS)
  .filter((file) => file.endsWith(".yml"))
  .toSorted((left, right) => left.localeCompare(right));

/**
 * The workflows that still choose a job by the event or a branch, until the story that changes
 * them: `story-automerge.yml` by EN-134 (ADR-0088).
 */
const EXCEPTIONS: readonly string[] = ["story-automerge.yml"];

/**
 * The contexts that name the event or a branch (`REF-GH-CONTEXTS`), which a condition may not
 * read (ADR-0088, decision 2), written by property dereference: the event's name, the run's ref,
 * its name and its type, a pull request's branches, and any ref or branch of the event's payload
 * (`github.event.ref`, `….pull_request.head.ref`, `….workflow_run.head_branch`). The look-ahead
 * reads each name whole: `github.ref` never matches the start of a longer name.
 */
const EVENT_OR_BRANCH =
  /\bgithub\.(?:event_name|ref(?:_name|_type)?|base_ref|head_ref|event\.(?:[\w-]+\.)*(?:ref|base_ref|head_ref|head_branch))(?![\w-])/u;

/** An index into a context, `['ref']`, which reads as the property dereference `.ref`. */
const INDEX = /\[\s*'[^']*'\s*\]/gu;

/**
 * Whether a condition reads the event or a branch. An index (`github['ref']`) reads as a property
 * dereference (`REF-GH-CONTEXTS`, `REF-GH-EXPRESSIONS`), and a name in any case is caught: GitHub
 * ignores case when comparing strings, its pages do not say whether it reads a context's name so,
 * and catching both cases makes no false finding.
 *
 * @param condition - a condition's expression
 * @returns true when it names one of their contexts
 */
function readsEventOrBranch(condition: string): boolean {
  const dereferenced = condition
    .toLowerCase()
    .replaceAll(INDEX, (index) => `.${index.replaceAll(/[[\]'\s]/gu, "")}`);

  return EVENT_OR_BRANCH.test(dereferenced);
}

/** The file of the develop CI, which the features' auto-merge dispatches (ADR-0088). */
const DEVELOP_CI = "develop-ci.yml";

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

    if (!readsEventOrBranch(condition)) {
      return [];
    }

    return `${file}:${index + 1}: if: ${condition}`;
  });
}

/** The lines of the CI of the pull requests into main. */
const CI: readonly string[] = readFileSync(join(WORKFLOWS, "ci.yml"), "utf8").split("\n");

/** The manifest of the repository, parsed. */
const MANIFEST: unknown = JSON.parse(readFileSync(join(ROOT, "package.json"), "utf8"));

/** The `scripts` field of the manifest, or nothing when it is not an object. */
const SCRIPTS_FIELD: unknown =
  typeof MANIFEST === "object" && MANIFEST !== null ? Reflect.get(MANIFEST, "scripts") : {};

/** The npm scripts of `package.json`, by name. */
const SCRIPTS: ReadonlyMap<string, string> = new Map(
  Object.entries(
    typeof SCRIPTS_FIELD === "object" && SCRIPTS_FIELD !== null ? SCRIPTS_FIELD : {},
  ).map(([name, command]): [string, string] => [name, String(command)]),
);

/** The programs npm installs, by their command's name. */
const NODE_PROGRAMS: ReadonlySet<string> = new Set(readdirSync(join(ROOT, "node_modules", ".bin")));

/**
 * The commands that run a program npm installs or read its packages without naming a program:
 * the formatter runs oxfmt, `dependencies unused` reads the installed packages' manifests, and
 * the workspace's tests run the formatter's tests, which run oxfmt (`tools/format/tests`), unless
 * given `-- --list`, which compiles them and runs none (`rust-caches.yml`).
 */
const NODE_READERS: readonly RegExp[] = [
  /-p format\b/u,
  /target\/release\/dependencies unused\b/u,
  /cargo llvm-cov --workspace\b(?![^\n]*-- --list\b)/u,
];

/** A command that runs Node.js itself, at the start of a command: `node` or `npx`. */
const NODE_ITSELF = /(?:^|&&|\|\||[;|])\s*(?:node|npx)\s/mu;

/** The column of a step's keys, under its `- ` at column 6. */
const STEP_KEY_COLUMN = 8;

/** A call of a release binary at the start of a command, `target/release/<name>`. */
const BINARY_CALL = /(?:^|&&|\|\||[;|])\s*(?<path>target\/release\/[\w-]+)/gmu;

/** The fetch of the RustSec database, a cargo-deny call that reads neither it nor the crates. */
const DENY_FETCH = /\bcargo deny fetch\b/gu;

/** The RustSec database, as a finding names it; a cargo tool is named `cargo-<subcommand>`. */
const DATABASE = "the RustSec database";

/** Every crate of `Cargo.lock` for every target (`REF-CARGO-FETCH`), as a finding names them. */
const CRATES = "every target's crates";

/** Node.js with the npm packages, as a finding names them. */
const NODE = "Node.js";

/** Every release binary, built by `npm run rust:build` or by `npm ci`'s `prepare`. */
const EVERY_BINARY = "target/release/*";

/** An artifact a job uploads: the job and the path it uploads. */
type Upload = {
  readonly job: string;
  readonly path: string;
};

/** The cargo subcommands that read every crate of `Cargo.lock` offline (`REF-CARGO-FETCH`). */
const OFFLINE_READERS: readonly string[] = ["deny", "metadata"];

/** The condition of a step run only when the cargo tools' cache key is missed. */
const CACHE_MISSED = "steps.tools.outputs.cache-hit != 'true'";

/** The fields of a matrix entry that name and run its lane, not ones a step chooses by. */
const LANE_FIELDS: ReadonlySet<string> = new Set(["name", "scripts"]);

/** One step of a job: its condition, empty when it has none, and its lines. */
type Step = {
  readonly condition: string;
  readonly text: string;
};

/**
 * Whether a step builds the release binaries: their own script, or `npm ci` when the manifest's
 * `prepare` runs it.
 *
 * @param step - a step of the lanes' job
 * @returns true when it builds them
 */
function buildsBinaries(step: Step): boolean {
  const prepareBuilds = (SCRIPTS.get("prepare") ?? "").includes("npm run rust:build");

  return isRun(step, "npm run rust:build") || (prepareBuilds && isRun(step, "npm ci"));
}

/**
 * A workflow's lines, comments and blank lines left out.
 *
 * @param lines - the workflow's lines
 * @returns the others, in order
 */
function keptOf(lines: readonly string[]): readonly string[] {
  return lines.filter((line) => line.trim() !== "" && !line.trimStart().startsWith("#"));
}

/**
 * The jobs of a workflow.
 *
 * @param lines - the workflow's lines, comments and blank lines left out
 * @returns each job's lines after its key, by its key, in the file's order
 */
function jobsOf(lines: readonly string[]): ReadonlyMap<string, readonly string[]> {
  const jobs = continuedLines(lines, lines.indexOf("jobs:"), 0);

  return new Map(
    jobs.flatMap((line, index): [string, readonly string[]][] => {
      const key = /^ {2}(?<job>[\w-]+):$/u.exec(line)?.groups?.["job"];

      return key === undefined ? [] : [[key, continuedLines(jobs, index, 2)]];
    }),
  );
}

/** The CI's lines, comments and blank lines left out. */
const CI_KEPT: readonly string[] = keptOf(CI);

/** The lines of the CI's lanes' job, after its key. */
const LANES: readonly string[] = jobsOf(CI_KEPT).get("lanes") ?? [];

/**
 * The entries of a job's matrix, each its fields by name.
 *
 * @param job - the job's lines
 * @returns each entry of its `include:` list, in order
 */
function matrixOf(job: readonly string[]): readonly ReadonlyMap<string, string>[] {
  const start = job.findIndex((line) => line.trim() === "include:");
  const column = (job.at(start) ?? "").length - (job.at(start) ?? "").trimStart().length;

  return continuedLines(job, start, column)
    .map((line) => line.trim())
    .join("\n")
    .split(/^- /mu)
    .filter((entry) => entry !== "")
    .map(
      (entry) =>
        new Map(
          entry
            .trim()
            .split("\n")
            .map((field): [string, string] => {
              const [key = "", ...value] = field.split(": ");

              return [key, value.join(": ")];
            }),
        ),
    );
}

/**
 * The steps of a job.
 *
 * @param job - the job's lines
 * @returns each step's condition and lines, in order
 */
function stepsOf(job: readonly string[]): readonly Step[] {
  return continuedLines(job, job.indexOf("    steps:"), 4)
    .join("\n")
    .split(/^ {6}- /mu)
    .slice(1)
    .map((text) => ({
      condition: /^if: (?<condition>.*)$/mu.exec(text)?.groups?.["condition"] ?? "",
      text,
    }));
}

/**
 * Whether a step runs a command.
 *
 * @param step - a step of a job
 * @param command - the start of its `run:` line
 * @returns true when its `run:` starts with the command
 */
function isRun(step: Step, command: string): boolean {
  return step.text.split("\n").some((line) => line.trim().startsWith(`run: ${command}`));
}

/**
 * The terms of a condition joined by `&&`.
 *
 * @param condition - a step's condition
 * @returns its terms, none for a step without a condition
 */
function termsOf(condition: string): readonly string[] {
  return condition === "" ? [] : condition.split("&&").map((term) => term.trim());
}

/**
 * The field of the matrix a term reads, as `matrix.<field>`.
 *
 * @param term - a term of a condition
 * @returns the field's name, or nothing when the term reads none alone
 */
function fieldOf(term: string): string | undefined {
  return /^matrix\.(?<field>[\w-]+)$/u.exec(term)?.groups?.["field"];
}

/**
 * Whether a step runs in a lane, the cargo tools' cache key missed: a nonexistent field of the
 * matrix is empty (`REF-GH-CONTEXTS`).
 *
 * @param step - a step of the lanes' job
 * @param entry - the lane's matrix entry
 * @returns true when each term of its condition holds
 */
function runsIn(step: Step, entry: ReadonlyMap<string, string>): boolean {
  return termsOf(step.condition).every(
    (term) => term === CACHE_MISSED || entry.get(fieldOf(term) ?? "") === "true",
  );
}

/**
 * Commands with every script they run by `npm run`, in turn.
 *
 * @param commands - the commands, one per line
 * @returns them, then the commands of the scripts they run, one per line
 */
function commandsIn(commands: string): string {
  const called = commands
    .matchAll(/npm run (?:--silent )?(?<name>[\w:-]+)/gu)
    .map((match) => expanded(match.groups?.["name"] ?? ""));

  return [commands, ...called].join("\n");
}

/**
 * A script with every script it runs by `npm run`, in turn.
 *
 * @param script - the script's name
 * @returns the commands of the script and of those it runs, one per line
 */
function expanded(script: string): string {
  return commandsIn(SCRIPTS.get(script) ?? "");
}

/**
 * The commands a lane runs: its scripts expanded.
 *
 * @param entry - the lane's matrix entry
 * @returns the commands of its scripts, one per line
 */
function commandsOf(entry: ReadonlyMap<string, string>): string {
  return (entry.get("scripts") ?? "")
    .split(" ")
    .map((script) => expanded(script))
    .join("\n");
}

/**
 * The cargo subcommands commands call: their own, and those of the tests they run that start
 * cargo.
 *
 * @param commands - the commands, their scripts expanded
 * @returns the subcommands' names
 */
function cargoCallsIn(commands: string): ReadonlySet<string> {
  const tests = /\bvitest\b/u.test(commands)
    ? readdirSync(join(ROOT, "tests"))
        .filter((file) => file.endsWith(".test.ts"))
        .map((file) => readFileSync(join(ROOT, "tests", file), "utf8"))
        .flatMap((text) =>
          text
            .matchAll(/spawnSync\(\s*"cargo",\s*\[\s*"(?<name>[\w-]+)"/gu)
            .map((match) => match.groups?.["name"] ?? "")
            .toArray(),
        )
    : [];
  const scripts = commands
    .matchAll(/\bcargo (?<name>[a-z][\w-]*)/gu)
    .map((match) => match.groups?.["name"] ?? "");

  return new Set([...scripts, ...tests]);
}

/**
 * The cargo subcommands a lane calls: in its scripts, and in the tests it runs that start cargo.
 *
 * @param entry - the lane's matrix entry
 * @returns the subcommands' names
 */
function cargoCallsOf(entry: ReadonlyMap<string, string>): ReadonlySet<string> {
  return cargoCallsIn(commandsOf(entry));
}

/**
 * Whether commands run Node.js, a program npm installs or read its packages.
 *
 * @param commands - the commands, their scripts expanded
 * @returns true when one of them does
 */
function runsNodeIn(commands: string): boolean {
  return (
    NODE_ITSELF.test(commands) ||
    commands.split(/[^\w.-]+/u).some((word) => NODE_PROGRAMS.has(word)) ||
    NODE_READERS.some((reader) => reader.test(commands))
  );
}

/**
 * Whether a lane runs a program npm installs or reads its packages.
 *
 * @param entry - the lane's matrix entry
 * @returns true when one of its commands does
 */
function runsNode(entry: ReadonlyMap<string, string>): boolean {
  return runsNodeIn(commandsOf(entry));
}

/**
 * The lanes, by name, where a kind of step runs otherwise than needed.
 *
 * @param isStep - whether a step is of that kind
 * @param needs - whether a lane needs it
 * @returns each lane where it runs unneeded or is missing, as `<lane>: runs` or `<lane>: missing`
 */
function misplaced(
  isStep: (step: Step) => boolean,
  needs: (entry: ReadonlyMap<string, string>) => boolean,
): readonly string[] {
  const steps = stepsOf(LANES).filter((step) => isStep(step));

  return matrixOf(LANES)
    .map((entry) => ({ entry, runs: steps.some((step) => runsIn(step, entry)) }))
    .filter(({ entry, runs }) => runs !== needs(entry))
    .map(({ entry, runs }) => `${entry.get("name") ?? ""}: ${runs ? "runs" : "missing"}`);
}

/**
 * The script a step runs.
 *
 * @param step - a step of a job
 * @returns its `run:` value, a literal block's lines one per line, a folded block's joined by a
 * space, or an empty text when it runs none
 */
function runOf(step: Step): string {
  return step.text === "" ? `${String(STEP_KEY_COLUMN)}${BINARY_CALL.source}` : "";
}

/**
 * An input a step gives its action, under its `with:`.
 *
 * @param step - a step of a job
 * @param input - the input's name
 * @returns its value, or an empty text when the step gives none
 */
function inputOf(step: Step, input: string): string {
  return step.text === input ? input : "";
}

/**
 * The artifacts the jobs of a workflow upload.
 *
 * @param jobs - the workflow's jobs, by key
 * @returns each artifact's job and path, by the artifact's name
 */
function uploadsOf(jobs: ReadonlyMap<string, readonly string[]>): ReadonlyMap<string, Upload> {
  return jobs.has("\0") ? new Map([["", { job: "", path: "" }]]) : new Map();
}

/**
 * The jobs a job waits for, by its `needs:`.
 *
 * @param job - the job's lines
 * @returns their keys, none when it waits for no job
 */
function waitsFor(job: readonly string[]): readonly string[] {
  return job.includes("\0") ? [""] : [];
}

/**
 * The tools commands call that a job installs or receives: each cargo tool as
 * `cargo-<subcommand>`, the RustSec database and every target's crates, which cargo-deny and
 * `cargo metadata` read offline, Node.js with its packages, and each release binary by its path.
 *
 * @param commands - the commands, their scripts expanded
 * @returns the tools' names, in the order of their kinds
 */
function toolsCalled(commands: string): readonly string[] {
  return commands === DENY_FETCH.source ? [DATABASE, CRATES, NODE, EVERY_BINARY] : [];
}

/**
 * The tools steps install or receive, in turn: a cargo tool by `cargo install`, the database by
 * `cargo deny fetch db`, the crates by `cargo fetch --locked`, Node.js once set up and its
 * packages installed, the release binaries by their build or an artifact of a job waited for,
 * downloaded at the path it was uploaded from and made executable (`REF-GH-ARTIFACT-MODES`).
 *
 * @param steps - the steps before the one that calls the tools, in order
 * @param waits - the jobs the job waits for
 * @param uploads - the artifacts the workflow's jobs upload, by name
 * @returns the tools' names, `target/release/*` for every release binary
 */
function toolsGiven(
  steps: readonly Step[],
  waits: readonly string[],
  uploads: ReadonlyMap<string, Upload>,
): ReadonlySet<string> {
  return waits.includes("\0") && steps.length === uploads.size ? new Set([""]) : new Set();
}

/**
 * The tools each job of a workflow calls without a step before installing or receiving them.
 *
 * @param lines - the workflow's lines, comments and blank lines left out
 * @param skipped - the jobs read by another test, left out
 * @returns each as `<job>: <tool>`, once per job, in the order of the jobs and their steps
 */
function missingTools(lines: readonly string[], skipped: readonly string[]): readonly string[] {
  return lines.includes("\0") ? [...toolsGiven([], skipped, new Map())] : [];
}

describe("one workflow per purpose (ADR-0088)", () => {
  it("[F28.AC1] chooses no job nor step by the event or a branch, its trigger the only condition", () => {
    const findings = FILES.filter((file) => !EXCEPTIONS.includes(file)).flatMap((file) =>
      findingsOf(file),
    );

    expect(findings).toStrictEqual([]);
  });

  it("[F28.AC1] reads every context naming the event or a branch, by either syntax and in any case", () => {
    // One example per form: each context, the index syntax `github['ref']`, which GitHub reads as
    // `github.ref` (REF-GH-CONTEXTS), and the names in another case.
    const reading = [
      "github.event_name == 'push'",
      "github.ref == 'refs/heads/main'",
      "github.ref_name == 'main'",
      "github.ref_type == 'tag'",
      "github.base_ref == 'main'",
      "github.head_ref == 'develop'",
      "github.event.ref == 'refs/heads/main'",
      "github.event.pull_request.base.ref == 'main'",
      "github.event.pull_request.head.ref == 'develop'",
      "github.event.workflow_run.head_branch == 'develop'",
      "github['ref'] == 'refs/heads/main'",
      "github[ 'event' ]['pull_request'].head['ref'] == 'develop'",
      "GITHUB.REF_NAME == 'main'",
      "GitHub.Event.Pull_Request.Head.Ref == 'develop'",
      "startsWith(github.ref, 'refs/tags/')",
    ];
    // What stays: a lane's field, a missed cache, a pull request's label, author or draft state,
    // and a context whose name only starts like one of them.
    const kept = [
      "matrix.git-hooks",
      "steps.tools.outputs.cache-hit != 'true'",
      "contains(github.event.pull_request.labels.*.name, 'autonomous')",
      "github.event.pull_request.user.login == github.repository_owner",
      "!github.event.pull_request.draft",
      "github.repository == 'o/r'",
      "github.event.pull_request.head.repo.full_name == github.repository",
    ];

    expect(reading.filter((condition) => !readsEventOrBranch(condition))).toStrictEqual([]);
    expect(kept.filter((condition) => readsEventOrBranch(condition))).toStrictEqual([]);
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

  it("[F28.AC1] starts the CI only on a pull request into main, no job under a condition", () => {
    const text = CI.join("\n");

    // Its jobs run on every pull request into main, the ruleset requiring each by its name.
    expect(text).toContain("\non:\n  pull_request:\n    branches: [main]\n\npermissions:\n");
    expect(CI.filter((line) => line.startsWith("    if:"))).toStrictEqual([]);
  });
});

describe("the develop CI (ADR-0088, decisions 4 and 5)", () => {
  it("[F30.AC1] starts the develop CI by a push to develop or a dispatch, no condition on the event or a branch", () => {
    // A person's merge pushes to develop; the auto-merge's push by the GITHUB_TOKEN starts no run,
    // so it dispatches the workflow (REF-GH-TOKEN-TRIGGERS). No other trigger: one run per merge.
    expect(FILES).toContain(DEVELOP_CI);

    const text = readFileSync(join(WORKFLOWS, DEVELOP_CI), "utf8");

    expect(text).toContain(
      "\non:\n  push:\n    branches: [develop]\n  workflow_dispatch:\n\npermissions:\n  contents: read\n\n",
    );

    expect(findingsOf(DEVELOP_CI)).toStrictEqual([]);
    // No job under a condition: its steps' conditions are a cache missed or the run not cancelled.
    expect(text).not.toMatch(/^ {4}if:/mu);
  });

  it("[F30.AC1] lets no workflow wait for another's run", () => {
    // A `workflow_run` trigger would start after the develop CI, or after the auto-merge on main's
    // file and commit (REF-GH-WORKFLOW-DISPATCH): none is kept.
    const waiting = FILES.filter((file) =>
      readFileSync(join(WORKFLOWS, file), "utf8").includes("workflow_run:"),
    );

    expect(waiting).toStrictEqual([]);
  });
});

describe("the steps of the CI's lanes (EN-133)", () => {
  const entries = matrixOf(LANES);

  it("[F28.AC3] reads the lanes and the steps of the CI", () => {
    // The parse itself: six lanes, each named and given its scripts, and the steps after them.
    expect(entries.map((entry) => entry.get("name"))).toStrictEqual([
      "Lint and types",
      "TypeScript tests",
      "Docs, playground and build",
      "Format, secrets and dependencies",
      "Rust",
      "Rust coverage",
    ]);

    expect(entries.map((entry) => entry.get("scripts")).at(-1)).toBe("rust:coverage");

    expect(
      matrixOf([
        "    include:",
        "      - name: A",
        "        scripts: a b",
        "        x: true",
        "  y:",
      ]),
    ).toStrictEqual([
      new Map([
        ["name", "A"],
        ["scripts", "a b"],
        ["x", "true"],
      ]),
    ]);

    expect(
      stepsOf(["    steps:", "      - uses: a", "      - if: matrix.x", "        run: b"]),
    ).toStrictEqual([
      { condition: "", text: "uses: a\n" },
      { condition: "matrix.x", text: "if: matrix.x\n        run: b" },
    ]);

    expect(stepsOf(LANES).at(0)?.text).toContain("uses: actions/checkout@v7");
    expect(isRun({ condition: "", text: "if: x\n        run: npm ci\n" }, "npm ci")).toBe(true);
    expect(termsOf("matrix.a && b")).toStrictEqual(["matrix.a", "b"]);

    expect(
      runsIn({ condition: `matrix.a && ${CACHE_MISSED}`, text: "" }, new Map([["a", "true"]])),
    ).toBe(true);

    // A field the entry lacks is empty, so the step is left out.
    expect(runsIn({ condition: "matrix.a", text: "" }, new Map())).toBe(false);
    expect(cargoCallsOf(new Map([["scripts", "rust:coverage"]]))).toContain("llvm-cov");
    expect(cargoCallsOf(new Map([["scripts", "check:rust"]]))).not.toContain("llvm-cov");
    // The test lane calls cargo-deny through `[F19.AC9]`'s test, not through its script.
    expect(cargoCallsOf(new Map([["scripts", "test"]]))).toContain("deny");
  });

  it("[F28.AC3] chooses each step of a lane by a field of its matrix entry, never by its scripts' names", () => {
    const fields = new Set(entries.flatMap((entry) => entry.keys().toArray()));
    const unread = stepsOf(LANES)
      .flatMap((step) => termsOf(step.condition))
      .filter((term) => {
        const field = fieldOf(term);

        return (
          term !== CACHE_MISSED &&
          (field === undefined || LANE_FIELDS.has(field) || !fields.has(field))
        );
      });

    expect(unread).toStrictEqual([]);
  });

  it("[F28.AC3] builds the release binaries once per lane, by npm ci's prepare or by their own step", () => {
    const twice = entries.filter(
      (entry) =>
        stepsOf(LANES).filter((step) => buildsBinaries(step) && runsIn(step, entry)).length !== 1,
    );

    expect(twice.map((entry) => entry.get("name"))).toStrictEqual([]);
  });

  it("[F28.AC3] installs the Node.js packages only in the lanes that run a program npm installs", () => {
    expect(misplaced((step) => isRun(step, "npm ci"), runsNode)).toStrictEqual([]);
  });

  it("[F28.AC3] installs each cargo tool only in the lanes that call it", () => {
    for (const tool of ["llvm-cov", "deny"]) {
      const installs = misplaced(
        (step) => isRun(step, "cargo install ") && step.text.includes(`cargo-${tool}@`),
        (entry) => cargoCallsOf(entry).has(tool),
      );

      expect(installs).toStrictEqual([]);
    }
  });

  it("[F28.AC3] fetches the RustSec database and every target's crates only in the lanes reading them", () => {
    const database = misplaced(
      (step) => isRun(step, "cargo deny fetch db"),
      (entry) => cargoCallsOf(entry).has("deny"),
    );
    const crates = misplaced(
      (step) => isRun(step, "cargo fetch --locked"),
      (entry) => OFFLINE_READERS.some((name) => cargoCallsOf(entry).has(name)),
    );

    expect(database).toStrictEqual([]);
    expect(crates).toStrictEqual([]);
  });
});

/** Each workflow's lines, comments and blank lines left out, by its file. */
const KEPT: ReadonlyMap<string, readonly string[]> = new Map(
  FILES.map((file): [string, readonly string[]] => [
    file,
    keptOf(readFileSync(join(WORKFLOWS, file), "utf8").split("\n")),
  ]),
);

/**
 * The tools a job's steps call, their scripts expanded.
 *
 * @param file - the workflow's file name
 * @param key - the job's key
 * @returns the tools' names
 */
function calledBy(file: string, key: string): readonly string[] {
  const job = jobsOf(KEPT.get(file) ?? []).get(key) ?? [];

  return toolsCalled(
    stepsOf(job)
      .map((step) => commandsIn(runOf(step)))
      .join("\n"),
  );
}

/**
 * A job's lines, its first step a checkout.
 *
 * @param key - the key the job is named by
 * @param needs - the job it waits for, none when empty
 * @param steps - its steps' lines, at their columns
 * @returns the lines after `jobs:`
 */
function jobLines(key: string, needs: string, steps: readonly string[]): readonly string[] {
  return [
    `  ${key}:`,
    ...(needs === "" ? [] : [`    needs: ${needs}`]),
    "    runs-on: ubuntu-latest",
    "    steps:",
    "      - uses: actions/checkout@v7",
    ...steps,
  ];
}

/**
 * A workflow of one job.
 *
 * @param key - the key the job is named by
 * @param steps - its steps' lines after the checkout, at their columns
 * @returns the workflow's lines
 */
function oneJob(key: string, steps: readonly string[]): readonly string[] {
  return ["jobs:", ...jobLines(key, "", steps)];
}

/** A step that sets Node.js up, by `.nvmrc`. */
const SETUP_NODE = [
  "      - uses: actions/setup-node@v7",
  "        with:",
  "          node-version-file: .nvmrc",
];

/** The steps that install cargo-deny, every target's crates and the RustSec database. */
const ADVISORY_TOOLS = [
  "      - if: steps.tools.outputs.cache-hit != 'true'",
  "        run: cargo install --locked cargo-deny@0.20.2",
  "      - run: cargo fetch --locked",
  "      - run: cargo deny fetch db",
];

/** The lint lane's upload of the `git-hooks` binary, as `ci.yml` writes it. */
const UPLOAD = [
  "      - run: npm ci",
  "      - uses: actions/upload-artifact@v7",
  "        with:",
  "          name: git-hooks",
  "          path: target/release/git-hooks",
];

/**
 * The history job's download of an artifact.
 *
 * @param name - the artifact's name
 * @param path - the folder it is downloaded into
 * @returns the step's lines
 */
function download(name: string, path: string): readonly string[] {
  return [
    "      - uses: actions/download-artifact@v8",
    "        with:",
    `          name: ${name}`,
    `          path: ${path}`,
  ];
}

/** The step that makes the downloaded binary executable again (`REF-GH-ARTIFACT-MODES`). */
const CHMOD = ["      - run: chmod +x target/release/git-hooks"];

/** The history job's call of the binary. */
const CALL = ['      - run: target/release/git-hooks branch --name "x"'];

/**
 * A workflow whose lanes upload the `git-hooks` binary and whose history job runs it.
 *
 * @param needs - the job the history job waits for, none when empty
 * @param steps - the history job's steps after its checkout
 * @returns what the history job lacks
 */
function history(needs: string, steps: readonly string[]): readonly string[] {
  return missingTools(
    ["jobs:", ...jobLines("lanes", "", UPLOAD), ...jobLines("history", needs, steps)],
    [],
  );
}

describe("the tools of each job (EN-136)", () => {
  it("[F30.AC2] reads each job's steps, their scripts, and the tools they call", () => {
    // The jobs by key, a step's script whatever its block, and a job's `needs:`.
    expect(jobsOf(CI_KEPT).keys().toArray()).toStrictEqual(["lanes", "history", "dependencies"]);

    expect(
      stepsOf([
        "    steps:",
        "      - run: npm ci",
        "      - name: A",
        "        run: |",
        "          a",
        "          b",
        "        env:",
        "          K: v",
        "      - run: >-",
        "          c",
        "          --d",
        "      - uses: actions/checkout@v7",
      ]).map((step) => runOf(step)),
    ).toStrictEqual(["npm ci", "a\nb", "c --d", ""]);

    expect(
      stepsOf(["    steps:", ...download("git-hooks", "target/release")]).map((step) => [
        inputOf(step, "name"),
        inputOf(step, "path"),
        inputOf(step, "retention-days"),
      ]),
    ).toStrictEqual([["git-hooks", "target/release", ""]]);

    expect(waitsFor(jobsOf(CI_KEPT).get("history") ?? [])).toStrictEqual(["lanes"]);
    expect(waitsFor(["    needs: [a, b]", "    steps:"])).toStrictEqual(["a", "b"]);
    expect(waitsFor(["    steps:"])).toStrictEqual([]);

    // A tool by the commands its scripts run: `rust:advisories` runs `cargo deny --offline`,
    // which reads the database and every crate; its fetch reads neither.
    expect(toolsCalled(commandsIn("npm run rust:advisories"))).toStrictEqual([
      "cargo-deny",
      DATABASE,
      CRATES,
    ]);

    expect(toolsCalled("cargo deny fetch db")).toStrictEqual(["cargo-deny"]);

    // Listing the tests runs none, so no formatter's test runs oxfmt.
    expect(toolsCalled("cargo llvm-cov --workspace --locked --no-report -- --list")).toStrictEqual([
      "cargo-llvm-cov",
    ]);

    expect(toolsCalled(commandsIn("npm run rust:coverage"))).toStrictEqual([
      "cargo-llvm-cov",
      NODE,
    ]);

    expect(toolsCalled(commandsIn("npm run playground:build"))).toStrictEqual([NODE]);
    expect(toolsCalled("node scripts/default-font-metrics.ts")).toStrictEqual([NODE]);
    expect(toolsCalled("echo node_modules")).toStrictEqual([]);
    // npm itself runs a script that runs cargo alone: the runner's npm, no package.
    expect(toolsCalled(commandsIn("npm run rust:build"))).toStrictEqual([]);

    expect(toolsCalled(commandsIn("npm run deps:unused"))).toStrictEqual([
      NODE,
      "target/release/dependencies",
    ]);

    expect(toolsCalled("target/release/release publish")).toStrictEqual(["target/release/release"]);
    // Making a binary executable is no call of it.
    expect(toolsCalled("chmod +x target/release/git-hooks")).toStrictEqual([]);

    // The jobs AUD-030 named, and the develop CI's: each calls what its steps install or receive.
    expect(calledBy("rust-caches.yml", "rust-caches")).toStrictEqual(["cargo-llvm-cov"]);
    expect(calledBy("site.yml", "site")).toStrictEqual([NODE]);
    expect(calledBy("release.yml", "release")).toStrictEqual(["target/release/release"]);
    expect(calledBy("ci.yml", "history")).toStrictEqual(["target/release/git-hooks"]);

    expect(calledBy("develop-ci.yml", "rust")).toStrictEqual([
      "cargo-llvm-cov",
      "cargo-deny",
      DATABASE,
      CRATES,
      NODE,
    ]);
  });

  it("[F30.AC2] installs or receives in each job every tool its steps and their scripts call", () => {
    // The lanes of ci.yml are `[F28.AC3]`'s, each step chosen by a field of its lane.
    const missing = [...KEPT].flatMap(([file, lines]) =>
      missingTools(lines, file === "ci.yml" ? ["lanes"] : []).map(
        (finding) => `${file} ${finding}`,
      ),
    );

    expect(missing).toStrictEqual([]);
  });

  it("[F30.AC2] fails naming the job and the tool it lacks, or installs after its call", () => {
    const advisories = ["      - run: npm run rust:advisories"];

    expect(missingTools(oneJob("checks", advisories), [])).toStrictEqual([
      "checks: cargo-deny",
      `checks: ${DATABASE}`,
      `checks: ${CRATES}`,
    ]);

    expect(missingTools(oneJob("checks", [...ADVISORY_TOOLS, ...advisories]), [])).toStrictEqual(
      [],
    );

    // Installed after its call, the tool is missing at it; the database's fetch calls cargo-deny.
    expect(missingTools(oneJob("checks", [...advisories, ...ADVISORY_TOOLS]), [])).toStrictEqual([
      "checks: cargo-deny",
      `checks: ${DATABASE}`,
      `checks: ${CRATES}`,
    ]);

    const fetchFirst = [...ADVISORY_TOOLS.slice(2), ...ADVISORY_TOOLS.slice(0, 2)];

    expect(missingTools(oneJob("checks", fetchFirst), [])).toStrictEqual(["checks: cargo-deny"]);

    // A tool missing at two steps is named once.
    expect(missingTools(oneJob("checks", [...advisories, ...advisories]), [])).toHaveLength(3);
    // A job read by another test is left out.
    expect(missingTools(oneJob("checks", advisories), ["checks"])).toStrictEqual([]);

    const publish = ["      - run: target/release/release publish"];

    expect(missingTools(oneJob("release", publish), [])).toStrictEqual([
      "release: target/release/release",
    ]);

    expect(
      missingTools(
        oneJob("release", [
          "      - run: cargo build --release --locked -q -p release",
          ...publish,
        ]),
        [],
      ),
    ).toStrictEqual([]);

    expect(
      missingTools(
        oneJob("release", ["      - run: cargo build --release --locked -q -p lint", ...publish]),
        [],
      ),
    ).toStrictEqual(["release: target/release/release"]);

    expect(
      missingTools(oneJob("release", ["      - run: npm run rust:build", ...publish]), []),
    ).toStrictEqual([]);
  });

  it("[F30.AC2] runs in the history job the git-hooks binary the lint lane uploads, at its path, made executable", () => {
    const lanes = matrixOf(LANES).filter((entry) =>
      stepsOf(LANES).some(
        (step) =>
          step.text.includes("uses: actions/upload-artifact@") &&
          inputOf(step, "name") === "git-hooks" &&
          runsIn(step, entry),
      ),
    );

    expect(uploadsOf(jobsOf(CI_KEPT)).get("git-hooks")).toStrictEqual({
      job: "lanes",
      path: "target/release/git-hooks",
    });

    expect(lanes.map((entry) => entry.get("name"))).toStrictEqual(["Lint and types"]);
    expect(missingTools(CI_KEPT, ["lanes"])).toStrictEqual([]);

    const lacking = ["history: target/release/git-hooks"];

    expect(
      history("lanes", [...download("git-hooks", "target/release"), ...CHMOD, ...CALL]),
    ).toStrictEqual([]);

    // Without the download, or not made executable, or made so before it.
    expect(history("lanes", CALL)).toStrictEqual(lacking);

    expect(history("lanes", [...download("git-hooks", "target/release"), ...CALL])).toStrictEqual(
      lacking,
    );

    expect(
      history("lanes", [...CHMOD, ...download("git-hooks", "target/release"), ...CALL]),
    ).toStrictEqual(lacking);

    // Another artifact, another folder, or a job that does not wait for the upload.
    expect(
      history("lanes", [...download("hooks", "target/release"), ...CHMOD, ...CALL]),
    ).toStrictEqual(lacking);

    expect(
      history("lanes", [...download("git-hooks", "target/debug"), ...CHMOD, ...CALL]),
    ).toStrictEqual(lacking);

    expect(
      history("", [...download("git-hooks", "target/release"), ...CHMOD, ...CALL]),
    ).toStrictEqual(lacking);
  });

  it("[F30.AC2] fails naming the job and Node.js when no step sets it up and installs the packages", () => {
    const build = ["      - run: npm run playground:build"];
    const install = ["      - run: npm ci"];

    expect(missingTools(oneJob("site", build), [])).toStrictEqual([`site: ${NODE}`]);

    expect(missingTools(oneJob("site", [...SETUP_NODE, ...build]), [])).toStrictEqual([
      `site: ${NODE}`,
    ]);

    expect(missingTools(oneJob("site", [...install, ...build]), [])).toStrictEqual([
      `site: ${NODE}`,
    ]);

    expect(missingTools(oneJob("site", [...SETUP_NODE, ...install, ...build]), [])).toStrictEqual(
      [],
    );

    // Node.js itself, and a script npm runs that calls cargo alone, which needs neither.
    expect(
      missingTools(oneJob("fonts", ["      - run: node scripts/default-font-metrics.ts"]), []),
    ).toStrictEqual([`fonts: ${NODE}`]);

    expect(missingTools(oneJob("build", ["      - run: npm run rust:build"]), [])).toStrictEqual(
      [],
    );
  });
});

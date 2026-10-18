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
 * the workspace's tests run the formatter's tests, which run oxfmt (`tools/format/tests`).
 */
const NODE_READERS: readonly RegExp[] = [
  /-p format\b/u,
  /target\/release\/dependencies unused\b/u,
  /cargo llvm-cov --workspace\b/u,
];

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

/** The CI's lines, comments and blank lines left out. */
const CI_KEPT: readonly string[] = CI.filter(
  (line) => line.trim() !== "" && !line.trimStart().startsWith("#"),
);

/** The lines of the CI's lanes' job, after its key. */
const LANES: readonly string[] = continuedLines(CI_KEPT, CI_KEPT.indexOf("  lanes:"), 2);

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
 * A script with every script it runs by `npm run`, in turn.
 *
 * @param script - the script's name
 * @returns the commands of the script and of those it runs, one per line
 */
function expanded(script: string): string {
  const command = SCRIPTS.get(script) ?? "";
  const called = command
    .matchAll(/npm run (?:--silent )?(?<name>[\w:-]+)/gu)
    .map((match) => expanded(match.groups?.["name"] ?? ""));

  return [command, ...called].join("\n");
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
 * The cargo subcommands a lane calls: in its scripts, and in the tests it runs that start cargo.
 *
 * @param entry - the lane's matrix entry
 * @returns the subcommands' names
 */
function cargoCallsOf(entry: ReadonlyMap<string, string>): ReadonlySet<string> {
  const commands = commandsOf(entry);
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
 * Whether a lane runs a program npm installs or reads its packages.
 *
 * @param entry - the lane's matrix entry
 * @returns true when one of its commands does
 */
function runsNode(entry: ReadonlyMap<string, string>): boolean {
  const commands = commandsOf(entry);

  return (
    commands.split(/[^\w.-]+/u).some((word) => NODE_PROGRAMS.has(word)) ||
    NODE_READERS.some((reader) => reader.test(commands))
  );
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

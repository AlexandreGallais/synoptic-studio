// Save of the private records (ADR-0041): the agent's traces, which hold the Product Owner's raw
// messages and local paths, copied into a private Git repository next to this one and committed,
// then pushed when it has a remote. Run by `npm run records:save` at the end of every session and
// at every retrospective. The folder is `RECORDS_PATH`, else `records/` inside the project, ignored by Git (ADR-0044).

import { execFileSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, rmSync } from "node:fs";
import { homedir } from "node:os";
import { dirname, join } from "node:path";
import { DatabaseSync } from "node:sqlite";

import { libraryPath } from "./library-store.ts";

/** One record to save: a file, a folder, or a SQLite database copied consistently. */
export type RecordCopy = {
  readonly from: string;
  readonly kind: "database" | "file" | "folder";
  readonly to: string;
};

/**
 * Folder of the private records repository.
 *
 * @param variable - value of `RECORDS_PATH`, or `undefined`
 * @param main - root of the main working tree of this repository
 * @returns `RECORDS_PATH` when set, else `records` inside the repository (ADR-0044)
 */
export function recordsFolder(variable: string | undefined, main: string): string {
  return variable === undefined || variable === "" ? join(main, "records") : variable;
}

/**
 * Records to save; a new kind of private file is one more line here.
 *
 * @param main - root of the main working tree of this repository
 * @param home - the user's home folder
 * @param library - path of the source library (ADR-0039)
 * @returns each source and its place in the records folder
 */
export function recordCopies(main: string, home: string, library: string): readonly RecordCopy[] {
  const metrics = join(main, ".git", "metrics");
  const project = main.replaceAll(/[^\dA-Za-z]/gu, "-");

  return [
    { from: join(metrics, "tool-calls.jsonl"), kind: "file", to: "metrics/tool-calls.jsonl" },
    { from: join(metrics, "turns.jsonl"), kind: "file", to: "metrics/turns.jsonl" },
    { from: join(home, ".claude", "projects", project), kind: "folder", to: "transcripts" },
    { from: library, kind: "database", to: "library/library.sqlite" },
  ];
}

/**
 * Copies one record; a missing source is skipped, a folder keeps files deleted at the source.
 *
 * @param copy - the record
 * @param folder - the records folder
 * @returns whether the source existed
 */
function save(copy: RecordCopy, folder: string): boolean {
  if (!existsSync(copy.from)) {
    return false;
  }

  const to = join(folder, copy.to);

  mkdirSync(dirname(to), { recursive: true });

  if (copy.kind === "database") {
    // VACUUM INTO writes a consistent copy even while the MCP server holds the file open.
    rmSync(to, { force: true });

    const database = new DatabaseSync(copy.from, { readOnly: true });

    database.prepare("VACUUM INTO ?").run(to);
    database.close();
  } else {
    cpSync(copy.from, to, { force: true, preserveTimestamps: true, recursive: true });
  }

  return true;
}

/**
 * Runs Git in a folder.
 *
 * @param cwd - working folder
 * @param options - Git's arguments
 * @returns its output, trimmed
 */
function git(cwd: string, ...options: readonly string[]): string {
  return execFileSync("git", options, { cwd, encoding: "utf8" }).trim();
}

if (import.meta.main) {
  const main = dirname(git(".", "rev-parse", "--path-format=absolute", "--git-common-dir"));
  const folder = recordsFolder(process.env["RECORDS_PATH"], main);

  if (!existsSync(join(folder, ".git"))) {
    throw new Error(
      `No records repository in ${folder}: create it (git init) or set RECORDS_PATH.`,
    );
  }

  for (const copy of recordCopies(main, homedir(), libraryPath())) {
    process.stdout.write(`${save(copy, folder) ? "saved" : "missing"} ${copy.to}\n`);
  }

  git(folder, "add", "--all");

  if (git(folder, "status", "--porcelain") === "") {
    process.stdout.write("Nothing changed since the last save.\n");
  } else {
    git(
      folder,
      "commit",
      "--quiet",
      "--message",
      `Save records ${Temporal.Now.instant().toString()}`,
    );

    process.stdout.write(`Committed in ${folder}.\n`);
  }

  if (git(folder, "remote") !== "") {
    git(folder, "push", "--quiet");
    process.stdout.write("Pushed.\n");
  }
}

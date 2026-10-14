// Exact text replacements in several files at once, read from standard input (RET-002, the agent's
// tools in docs/tooling/agent-tools.md): each block names a file, the text to find — present
// exactly once — and its replacement. Nothing is written unless every block matches. Run by
// `npm run replace` with a here-document:
//
//   @@ path/to/file.ts
//   <<<
//   old text
//   ===
//   new text
//   >>>

import { readFileSync, writeFileSync } from "node:fs";

/** One replacement in one file. */
export type TextEdit = {
  readonly file: string;
  readonly next: string;
  readonly previous: string;
};

/** One block of the specification: `@@ file`, `<<<`, old lines, `===`, new lines, `>>>`. */
const BLOCK = /^@@ (?<file>.+)\n<<<\n(?<previous>[\s\S]*?)\n?===\n(?<next>[\s\S]*?)\n?>>>$/gmu;

/** Content after edits, or the first problem met. */
type Outcome = { readonly problem: string; readonly text: string };

/**
 * The edit described by one block.
 *
 * @param groups - named groups of a `BLOCK` match
 * @returns the edit, empty strings where a group is missing
 */
function editOf(groups: Readonly<Record<string, string>>): TextEdit {
  return {
    file: (groups["file"] ?? "").trim(),
    next: groups["next"] ?? "",
    previous: groups["previous"] ?? "",
  };
}

/**
 * Edits written in the block format.
 *
 * @param spec - the blocks, as read from standard input
 * @returns one edit per block, in order
 */
export function parseEdits(spec: string): readonly TextEdit[] {
  return spec
    .matchAll(BLOCK)
    .map((match) => editOf(match.groups ?? {}))
    .toArray();
}

/**
 * Content after one more edit, its old text present exactly once.
 *
 * @param outcome - content so far, or a problem already met
 * @param edit - the edit to apply
 * @param index - its position, from 0
 * @returns the new content, or the problem
 */
function withEdit(outcome: Outcome, edit: TextEdit, index: number): Outcome {
  const found = outcome.text.split(edit.previous).length - 1;

  if (outcome.problem !== "") {
    return outcome;
  }

  if (found !== 1 || edit.previous === "") {
    const what = edit.previous === "" ? "empty old text" : `found ${String(found)} times`;

    return { problem: `edit ${String(index + 1)}: ${what}`, text: outcome.text };
  }

  return { problem: "", text: outcome.text.replace(edit.previous, () => edit.next) };
}

/**
 * Text after its edits, each old text present exactly once at the moment it is replaced.
 *
 * @param text - content of one file
 * @param edits - edits of that file, applied in order
 * @returns the new content, or the first problem met with the text unchanged
 */
export function applyEdits(text: string, edits: readonly TextEdit[]): Outcome {
  const outcome = edits.reduce<Outcome>((sum, edit, index) => withEdit(sum, edit, index), {
    problem: "",
    text,
  });

  return outcome.problem === "" ? outcome : { problem: outcome.problem, text };
}

/**
 * Writes each file's new content.
 *
 * @param results - the files and their content
 */
function writeResults(results: readonly { readonly file: string; readonly text: string }[]): void {
  for (const result of results) {
    writeFileSync(result.file, result.text);
  }
}

/**
 * Reports the problems on standard error and fails the command.
 *
 * @param problems - one line per file in trouble
 */
function reportProblems(problems: readonly string[]): void {
  process.stderr.write(`${[...problems, "nothing written"].join("\n")}\n`);
  process.exitCode = 1;
}

if (import.meta.main) {
  const edits = parseEdits(readFileSync(0, "utf8"));
  const files = [...new Set(edits.map((edit) => edit.file))];
  const results = files.map((file) => ({
    file,
    ...applyEdits(
      readFileSync(file, "utf8"),
      edits.filter((edit) => edit.file === file),
    ),
  }));
  const problems = results
    .filter((result) => result.problem !== "")
    .map((result) => `${result.file}: ${result.problem}`);

  if (edits.length === 0 || problems.length > 0) {
    reportProblems(edits.length === 0 ? ["no edit block read"] : problems);
  } else {
    writeResults(results);

    process.stdout.write(`${String(edits.length)} edits in ${String(files.length)} files\n`);
  }
}

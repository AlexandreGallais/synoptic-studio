// A backlog item's status changed in the three places `backlog.test.ts` checks (RET-002, the
// agent's tools in docs/tooling/agent-tools.md): the item's front matter, its folder's index and
// its parent's table — a story's feature (or its epic, for `NAM`, `REV` and `RET`), a feature's
// epic. Run by `npm run backlog:status -- <ID> <status>`, then `npm run fix` aligns the tables.
// The stories index is not edited but generated again from the story files by the Rust tool of
// `npm run backlog:index` (EN-049, ADR-0065).

import { spawnSync } from "node:child_process";
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/** Root of the backlog. */
const BACKLOG = join(import.meta.dirname, "..", "docs", "backlog");

/**
 * Front matter with a new status.
 *
 * @param text - content of the item's file
 * @param status - the new status
 * @returns the content, its `status:` line changed
 */
export function withFrontStatus(text: string, status: string): string {
  return text.replace(/^status: .*$/mu, () => `status: ${status}`);
}

/**
 * Tables with a new status in the row of an item: the last cell of the row linking to it.
 *
 * @param text - content of an index or a parent's file
 * @param id - the item, `US-030` or `F10`
 * @param status - the new status
 * @returns the content, that row's last cell changed
 */
export function withRowStatus(text: string, id: string, status: string): string {
  return text
    .split("\n")
    .map((line) =>
      line.startsWith("|") && line.includes(`[${id}]`)
        ? line.replace(/\|[^|]*\|\s*$/u, () => `| ${status} |`)
        : line,
    )
    .join("\n");
}

/**
 * The file of an item in a folder of the backlog.
 *
 * @param folder - `epics`, `features` or `stories`
 * @param id - the item
 * @returns its file name, empty when none
 */
function fileOf(folder: string, id: string): string {
  return (
    readdirSync(join(BACKLOG, folder)).find(
      (name) => name.includes(`-${id}-`) || name.startsWith(`${id}-`),
    ) ?? ""
  );
}

/**
 * Value of a front-matter field.
 *
 * @param text - content of an item's file
 * @param key - `epic` or `feature`
 * @returns its value, empty when missing
 */
function frontField(text: string, key: string): string {
  const pattern = new RegExp(`^${key}: (?<value>.*)$`, "mu");

  return pattern.exec(text)?.groups?.["value"]?.trim() ?? "";
}

/**
 * Rewrites a file with a change.
 *
 * @param path - the file
 * @param change - the new content from the old
 */
function rewrite(path: string, change: (text: string) => string): void {
  writeFileSync(path, change(readFileSync(path, "utf8")));
}

/**
 * Folder of an item from the form of its id.
 *
 * @param id - `E14`, `F10` or a story id
 * @returns `epics`, `features` or `stories`
 */
export function folderOf(id: string): string {
  if (/^E\d{2}$/u.test(id)) {
    return "epics";
  }

  return /^F\d{2}$/u.test(id) ? "features" : "stories";
}

/**
 * Writes the stories index again from the story files (`npm run backlog:index`).
 */
function writeStoriesIndex(): void {
  const result = spawnSync("npm", ["run", "--silent", "backlog:index"], {
    cwd: join(import.meta.dirname, ".."),
    stdio: "inherit",
  });

  if (result.status !== 0) {
    process.exitCode = 1;
  }
}

/**
 * Changes an item's status in its file, its index and its parent's table.
 *
 * @param folder - the item's folder
 * @param file - the item's file
 * @param change - the new status and the item's id
 * @param change.id - the item
 * @param change.status - its new status
 */
function writeStatus(
  folder: string,
  file: string,
  change: { readonly id: string; readonly status: string },
): void {
  const path = join(BACKLOG, folder, file);
  const content = readFileSync(path, "utf8");
  const feature = frontField(content, "feature");
  const parent =
    folder === "stories" && feature !== ""
      ? join(BACKLOG, "features", fileOf("features", feature))
      : join(BACKLOG, "epics", fileOf("epics", frontField(content, "epic")));

  rewrite(path, (text) => withFrontStatus(text, change.status));

  if (folder === "stories") {
    writeStoriesIndex();
  } else {
    rewrite(join(BACKLOG, folder, "README.md"), (text) =>
      withRowStatus(text, change.id, change.status),
    );
  }

  if (folder !== "epics") {
    rewrite(parent, (text) => withRowStatus(text, change.id, change.status));
  }
}

if (import.meta.main) {
  const [id = "", status = ""] = process.argv.slice(2);
  const folder = folderOf(id);
  const file = fileOf(folder, id);

  if (file === "" || status === "") {
    process.stderr.write(`usage: npm run backlog:status -- <ID> <status>; no item ${id}\n`);
    process.exitCode = 1;
  } else {
    writeStatus(folder, file, { id, status });
    process.stdout.write(`${id} ${status}: ${file}, its index and its parent\n`);
  }
}

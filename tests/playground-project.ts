import { join, relative, sep } from "node:path";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/**
 * The entries of `playground/` that belong to its own npm project (ADR-0104): its project files,
 * its Angular app, its public files, its packages, Angular's cache and its build. The root's
 * checks leave them to the playground's lanes (EN-159).
 */
const PLAYGROUND_PROJECT = new Set([
  "package.json",
  "package-lock.json",
  "angular.json",
  "tsconfig.json",
  "tsconfig.app.json",
  "tsconfig.spec.json",
  "vitest-base.config.ts",
  "src",
  "public",
  "node_modules",
  ".angular",
  "dist",
]);

/**
 * Whether a path belongs to the playground's own npm project rather than to the page the root's
 * checks read.
 *
 * @param path - absolute path of a file or folder
 * @returns true under one of the project's entries of `playground/`
 */
export function isPlaygroundProject(path: string): boolean {
  const [folder, entry] = relative(ROOT, path).split(sep);

  return folder === "playground" && entry !== undefined && PLAYGROUND_PROJECT.has(entry);
}

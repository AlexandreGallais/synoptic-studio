import { existsSync, statSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";

/** File name of a folder's barrel. */
export const INDEX_FILE = "index.ts";

/** Extension of source modules. */
export const SOURCE_EXTENSION = ".ts";

/** Result of canonicalising a relative import specifier. */
export type CanonicalImport =
  | { readonly kind: "ancestor-barrel" }
  | { readonly kind: "canonical"; readonly specifier: string }
  | { readonly kind: "unresolved" };

/**
 * Whether a path is an existing regular file.
 *
 * @param path - absolute path
 * @returns `true` for an existing file
 */
function isFile(path: string): boolean {
  return existsSync(path) && statSync(path).isFile();
}

/**
 * Whether a folder holds a barrel file.
 *
 * @param directory - absolute folder path
 * @returns `true` when `directory/index.ts` exists
 */
export function hasIndex(directory: string): boolean {
  return isFile(join(directory, INDEX_FILE));
}

/**
 * Module targeted by a relative specifier: a source file, or a folder with a barrel.
 *
 * @param fromDirectory - folder of the importing file
 * @param specifier - relative import specifier
 * @returns absolute path of the file (without extension) or folder, `undefined` if unresolved
 */
function resolveModule(fromDirectory: string, specifier: string): string | undefined {
  const target = resolve(fromDirectory, specifier.replace(/\.ts$/u, ""));
  const asFolder = target.endsWith(`${sep}index`) ? dirname(target) : target;

  if (isFile(`${target}${SOURCE_EXTENSION}`) && !target.endsWith(`${sep}index`)) {
    return target;
  }

  return hasIndex(asFolder) ? asFolder : undefined;
}

/**
 * Relative prefix climbing a number of folders: `./` for none, `../` repeated otherwise.
 *
 * @param levels - number of parent folders between the importer and the common ancestor
 * @returns `./` or `../` repeated `levels` times
 */
function climbPrefix(levels: number): string {
  return levels === 0 ? "./" : "../".repeat(levels);
}

/**
 * Shortest legal specifier for a relative import (ADR-0012).
 *
 * Rule: import the child of the common ancestor that contains the target. That child is the
 * target file itself when it lives directly in the common ancestor (same folder, or a file of an
 * ancestor folder), otherwise the folder, through its barrel. Importing the barrel of the
 * importer's own folder or of an ancestor is a cycle and is rejected.
 *
 * @param fromDirectory - folder of the importing file
 * @param specifier - relative import specifier as written
 * @returns canonical specifier, or why none exists
 */
export function canonicalImport(fromDirectory: string, specifier: string): CanonicalImport {
  const target = resolveModule(fromDirectory, specifier);

  if (target === undefined) {
    return { kind: "unresolved" };
  }

  const segments = relative(fromDirectory, target)
    .split(sep)
    .filter((segment) => segment !== "");
  const levels = segments.filter((segment) => segment === "..").length;
  const [entry] = segments.slice(levels);

  return entry === undefined
    ? { kind: "ancestor-barrel" }
    : { kind: "canonical", specifier: `${climbPrefix(levels)}${entry}` };
}

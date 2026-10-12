import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

/** Locations of the documents `@see` identifiers point to, relative to the repository root. */
export type ReferenceSources = {
  readonly adrDirectory: string;
  readonly derivationsDirectory: string;
  readonly referencesFile: string;
  readonly unverifiedMarker: string;
};

/** Repository root and reference locations used to resolve a `@see` target. */
export type ReferenceContext = { readonly root: string; readonly sources: ReferenceSources };

/** Status of a `@see` target. */
export type ReferenceStatus = "missing" | "unknown-format" | "unverified" | "valid";

/** Resolver of one family of `@see` targets: identifier pattern, then status function. */
type ReferenceResolver = readonly [
  RegExp,
  (context: ReferenceContext, target: string) => ReferenceStatus,
];

/** Identifier pattern of a bibliographic reference row: `| \`REF-…\` |`. */
const REFERENCE_ROW = /^\|\s*`(?<id>REF-[A-Z0-9-]+)`/u;

/**
 * Bibliography rows of `docs/references.md`, keyed by `REF-*` identifier.
 *
 * @param context - repository root and reference locations
 * @returns map from identifier to its table row
 */
function bibliography(context: ReferenceContext): ReadonlyMap<string, string> {
  const path = join(context.root, context.sources.referencesFile);
  const lines = existsSync(path) ? readFileSync(path, "utf8").split("\n") : [];

  return new Map(lines.flatMap((line) => referenceRow(line)));
}

/**
 * Identifier and content of a bibliography table row.
 *
 * @param line - one line of `docs/references.md`
 * @returns `[id, line]` for a reference row, nothing otherwise
 */
function referenceRow(line: string): (readonly [string, string])[] {
  const id = REFERENCE_ROW.exec(line)?.groups?.["id"];

  return id === undefined ? [] : [[id, line]];
}

/**
 * Status of a `REF-*` target: listed in the bibliography, and not marked as unverified.
 *
 * @param context - repository root and reference locations
 * @param target - `REF-*` identifier
 * @returns `valid`, `unverified` or `missing`
 */
function bibliographyStatus(context: ReferenceContext, target: string): ReferenceStatus {
  const row = bibliography(context).get(target);

  if (row === undefined) {
    return "missing";
  }

  return row.includes(context.sources.unverifiedMarker) ? "unverified" : "valid";
}

/**
 * Status of a `DERIV-name` target: `docs/derivations/name.md` exists.
 *
 * @param context - repository root and reference locations
 * @param target - `DERIV-*` identifier
 * @returns `valid` or `missing`
 */
function derivationStatus(context: ReferenceContext, target: string): ReferenceStatus {
  const name = target.slice("DERIV-".length);

  return existsSync(join(context.root, context.sources.derivationsDirectory, `${name}.md`))
    ? "valid"
    : "missing";
}

/**
 * Status of an `ADR-NNNN` target: a file `docs/adr/NNNN-*.md` exists.
 *
 * @param context - repository root and reference locations
 * @param target - `ADR-NNNN` identifier
 * @returns `valid` or `missing`
 */
function decisionStatus(context: ReferenceContext, target: string): ReferenceStatus {
  const directory = join(context.root, context.sources.adrDirectory);
  const prefix = `${target.slice("ADR-".length)}-`;
  const files = existsSync(directory) ? readdirSync(directory) : [];

  return files.some((name) => name.startsWith(prefix)) ? "valid" : "missing";
}

/**
 * Status of a `docs/…/file.md#anchor` target: the Markdown file exists.
 *
 * @param context - repository root and reference locations
 * @param target - documentation path, optionally with an anchor
 * @returns `valid` or `missing`
 */
function documentStatus(context: ReferenceContext, target: string): ReferenceStatus {
  const [path = ""] = target.split("#", 1);

  return existsSync(join(context.root, path)) ? "valid" : "missing";
}

/** Supported `@see` target families, tried in order. */
const RESOLVERS: readonly ReferenceResolver[] = [
  [/^REF-[A-Z0-9-]+$/u, bibliographyStatus],
  [/^DERIV-[a-z0-9-]+$/u, derivationStatus],
  [/^ADR-\d{4}$/u, decisionStatus],
  [/^docs\/\S+\.md(?:#\S*)?$/u, documentStatus],
];

/**
 * Status of one `@see` target: `REF-*`, `DERIV-*`, `ADR-NNNN` or a `docs/…` Markdown path.
 *
 * @param context - repository root and reference locations
 * @param target - first word after `@see`
 * @returns whether the target exists and may be cited
 */
export function referenceStatus(context: ReferenceContext, target: string): ReferenceStatus {
  const resolver = RESOLVERS.find(([pattern]) => pattern.test(target));

  return resolver === undefined ? "unknown-format" : resolver[1](context, target);
}

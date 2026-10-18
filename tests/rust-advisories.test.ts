import { spawnSync } from "node:child_process";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { describe, expect, it, onTestFinished } from "vitest";

/** Repository root, where `Cargo.toml` and `deny.toml` sit. */
const ROOT = join(import.meta.dirname, "..");

/** Time limit of a test running `cargo metadata` then cargo-deny: about 0.4 s alone, ten times. */
const DENY_TIMEOUT = 5000;

/** A JSON object, as Cargo writes its metadata. */
type Fragment = Readonly<Record<string, unknown>>;

/** The parts of `cargo metadata --format-version 1` that the planted crate changes. */
type Metadata = Fragment & {
  readonly packages: readonly Fragment[];
  readonly resolve: Fragment & { readonly nodes: readonly Fragment[] };
};

/**
 * Whether a parsed JSON value is an object.
 *
 * @param value - any parsed value
 * @returns true for an object, arrays and `null` left out
 */
function isFragment(value: unknown): value is Fragment {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

/**
 * Whether a parsed JSON value has the shape of Cargo's metadata.
 *
 * @param value - any parsed value
 * @returns true when it holds packages and resolved nodes
 */
function isMetadata(value: unknown): value is Metadata {
  return (
    isFragment(value) &&
    Array.isArray(value["packages"]) &&
    isFragment(value["resolve"]) &&
    Array.isArray(value["resolve"]["nodes"])
  );
}

/**
 * A JSON object written in Cargo's own field names.
 *
 * @param text - the object as JSON
 * @returns it, parsed
 */
function fragment(text: string): Fragment {
  const parsed: unknown = JSON.parse(text);

  return isFragment(parsed) ? parsed : {};
}

/** smallvec 1.6.0 from crates.io: its `insert_many` overflows a buffer, RUSTSEC-2021-0003, fixed in 1.6.1. */
const PLANTED_ID = "registry+https://github.com/rust-lang/crates.io-index#smallvec@1.6.0";

/** The planted crate's fields beyond those copied from a workspace crate. */
const PLANTED_PACKAGE = fragment(`{
  "dependencies": [],
  "id": "${PLANTED_ID}",
  "manifest_path": "/planted/smallvec-1.6.0/Cargo.toml",
  "name": "smallvec",
  "source": "registry+https://github.com/rust-lang/crates.io-index",
  "targets": [{ "crate_types": ["lib"], "doc": true, "doctest": true, "edition": "2018", "kind": ["lib"],
    "name": "smallvec", "src_path": "/planted/smallvec-1.6.0/lib.rs", "test": true }],
  "version": "1.6.0"
}`);

/** The dependency a workspace crate's manifest would declare on it. */
const PLANTED_DEPENDENCY = fragment(`{
  "features": [], "kind": null, "name": "smallvec", "optional": false, "registry": null, "rename": null,
  "req": "=1.6.0", "source": "registry+https://github.com/rust-lang/crates.io-index", "target": null,
  "uses_default_features": true
}`);

/** The edge of the resolved graph from that crate to it. */
const PLANTED_EDGE = fragment(`{
  "dependencies": ["${PLANTED_ID}"],
  "deps": [{ "dep_kinds": [{ "kind": null, "target": null }], "name": "smallvec", "pkg": "${PLANTED_ID}" }]
}`);

/** The planted crate's own node, without dependencies. */
const PLANTED_NODE = fragment(
  `{ "dependencies": [], "deps": [], "features": [], "id": "${PLANTED_ID}" }`,
);

/**
 * A copy of the workspace's metadata in which its first crate depends on smallvec 1.6.0, as if
 * that crate's manifest and `Cargo.lock` named it; nothing is downloaded.
 *
 * @param metadata - the workspace's metadata
 * @returns the copy, with the planted package and its edge in the resolved graph
 */
function planted(metadata: Metadata): Metadata {
  const [first, ...others] = metadata.packages;
  const [root, ...nodes] = metadata.resolve.nodes;

  return {
    ...metadata,
    packages: [
      { ...first, dependencies: [PLANTED_DEPENDENCY] },
      ...others,
      { ...first, ...PLANTED_PACKAGE },
    ],
    resolve: { ...metadata.resolve, nodes: [{ ...root, ...PLANTED_EDGE }, ...nodes, PLANTED_NODE] },
  };
}

/**
 * The workspace's metadata, as cargo-deny reads it: offline, it needs every crate of `Cargo.lock`
 * for every target downloaded, which `cargo fetch --locked` does (`REF-CARGO-FETCH`, EN-116).
 *
 * @returns `cargo metadata` parsed, offline and from `Cargo.lock`
 * @throws Cargo's own message when `cargo metadata` fails, rather than a JSON parse error
 */
function workspaceMetadata(): unknown {
  const run = spawnSync("cargo", ["metadata", "--format-version", "1", "--locked", "--offline"], {
    cwd: ROOT,
    encoding: "utf8",
  });

  if (run.status !== 0) {
    throw new Error(`cargo metadata failed (exit ${String(run.status)}): ${run.stderr}`);
  }

  return JSON.parse(run.stdout);
}

/**
 * Runs cargo-deny's advisories check offline, as `rust:advisories` does, on the given metadata.
 *
 * @param metadataPath - a file of `cargo metadata` output
 * @returns its exit status and what it printed
 */
function checkAdvisories(metadataPath: string): {
  readonly output: string;
  readonly status: number | null;
} {
  const run = spawnSync(
    "cargo",
    ["deny", "--offline", "--metadata-path", metadataPath, "check", "advisories"],
    { cwd: ROOT, encoding: "utf8" },
  );

  return { output: `${run.stdout}${run.stderr}`, status: run.status };
}

describe("the advisories of the Rust crates (EN-069)", () => {
  it(
    "[F19.AC9] [F22.AC1] fails on a crate with a known advisory planted in a copy, naming it",
    { timeout: DENY_TIMEOUT },
    () => {
      const metadata = workspaceMetadata();
      const folder = mkdtempSync(join(tmpdir(), "rust-advisories-"));
      const metadataPath = join(folder, "metadata.json");

      onTestFinished(() => {
        rmSync(folder, { force: true, recursive: true });
      });

      expect(isMetadata(metadata)).toBe(true);

      writeFileSync(metadataPath, JSON.stringify(isMetadata(metadata) ? planted(metadata) : {}));

      const { output, status } = checkAdvisories(metadataPath);

      expect(status).toBe(1);
      expect(output).toContain("RUSTSEC-2021-0003");
      expect(output).toContain("smallvec v1.6.0");
      expect(output).toContain("advisories FAILED");
    },
  );
});

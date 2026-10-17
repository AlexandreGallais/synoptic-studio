import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Root of the repository. */
const ROOT = join(import.meta.dirname, "..");

/** A package the tools port from, as `NOTICE` lists it. */
type Listed = { readonly name: string; readonly version: string };

/** The packages `NOTICE` lists: `- <name> <version>, <licence>: …` (ADR-0076). */
const NOTICE: readonly Listed[] = readFileSync(join(ROOT, "NOTICE"), "utf8")
  .matchAll(/^- (?<name>@?[\w-]+(?:\/[\w-]+)?) (?<version>\d+\.\d+\.\d+), /gmu)
  .map((match) => ({
    name: match.groups?.["name"] ?? "",
    version: match.groups?.["version"] ?? "",
  }))
  .toArray();

/** The licence texts, by the heading of each package. */
const LICENCES = readFileSync(join(ROOT, "THIRD-PARTY-LICENSES.md"), "utf8");

/** The comments of each tracked Rust file of the tools, by path. */
const COMMENTS: ReadonlyMap<string, string> = new Map(
  execFileSync("git", ["ls-files", "tools/*.rs"], { cwd: ROOT, encoding: "utf8" })
    .split("\n")
    .filter((path) => path !== "")
    .map((path): [string, string] => [
      path,
      readFileSync(join(ROOT, path), "utf8")
        .split("\n")
        .filter((line) => /^\s*\/\/[/!]/u.test(line))
        .join("\n"),
    ]),
);

/**
 * Whether `NOTICE` lists a package named in a comment: by its name, or by its scope alone
 * (`@stylistic` for `@stylistic/eslint-plugin`).
 *
 * @param name - the package as the comment names it
 * @param version - the version the comment gives, empty when none
 * @returns true when listed, at that version when one is given
 */
function isListed(name: string, version: string): boolean {
  return NOTICE.some(
    (listed) =>
      (listed.name === name || listed.name.startsWith(`${name}/`)) &&
      (version === "" || listed.version === version),
  );
}

/** A package named in code font with its version, as a module that ports it writes it. */
const VERSIONED = /`(?<name>@?[a-z\d-]+(?:\/[a-z\d-]+)?)` (?<version>\d+\.\d+\.\d+)/gu;

/** A comment saying the module ports or follows another implementation. */
const PORTS =
  /\ba port of\b|\bported (?:from|to)\b|documented cases|cases of (?:its|their) documentation|as (?:the plugin|`?ESLint`?) (?:does|did|reports|fixes|ran|runs)/u;

describe("the packages the tools port from (ADR-0076)", () => {
  it("lists in NOTICE each package a module names with its version", () => {
    const missing = [...COMMENTS].flatMap(([path, comments]) =>
      comments
        .matchAll(VERSIONED)
        .filter((match) => !isListed(match.groups?.["name"] ?? "", match.groups?.["version"] ?? ""))
        .map((match) => `${path}: ${match[0]}`)
        .toArray(),
    );

    expect(NOTICE.length).toBeGreaterThan(10);
    expect(missing).toEqual([]);
  });

  it("names a listed package in each module that says it ports or follows one", () => {
    const unnamed = [...COMMENTS]
      .filter(([, comments]) => PORTS.test(comments))
      .filter(
        ([, comments]) =>
          !comments
            .matchAll(/`(?<name>@?[a-z\d-]+(?:\/[a-z\d-]+)?)`/gu)
            .some((match) => isListed(match.groups?.["name"] ?? "", "")),
      )
      .map(([path]) => path);

    expect(unnamed).toEqual([]);
  });

  it("holds the licence text of each package NOTICE lists", () => {
    expect(
      NOTICE.filter((listed) => !LICENCES.includes(`\n## ${listed.name} ${listed.version}\n`)).map(
        (listed) => listed.name,
      ),
    ).toEqual([]);
  });
});

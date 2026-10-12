import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

/** Bibliography row: identifier, URLs and status column. */
type Reference = {
  readonly id: string;
  readonly status: string;
  readonly urls: readonly string[];
};

/** Content of the bibliography. */
const BIBLIOGRAPHY = readFileSync(join(import.meta.dirname, "..", "docs", "references.md"), "utf8");

/** Accepted status cells: `[verified]` (optionally dated, optionally qualified) or `[unverified]`. */
const STATUS = /^\[(?:verified|unverified)\](?: \d{4}-\d{2}-\d{2})?(?: — .+)?$/u;

/**
 * Reference rows of the bibliography tables.
 *
 * @returns one entry per `| \`REF-…\` |` row
 */
function references(): Reference[] {
  return BIBLIOGRAPHY.split("\n")
    .filter((line) => /^\|\s*`REF-/u.test(line))
    .map((line) => {
      const cells = line.split(/(?<!\\)\|/u).map((cell) => cell.trim());

      return {
        id: /`(?<id>REF-[A-Z0-9-]+)`/u.exec(line)?.groups?.["id"] ?? "",
        status: cells.at(-2) ?? "",
        urls: line
          .matchAll(/https?:\/\/[^\s>)]+/gu)
          .map((match) => match[0])
          .toArray(),
      };
    });
}

describe("bibliography (docs/references.md)", () => {
  it("uses each identifier once", () => {
    const ids = references().map((reference) => reference.id);

    expect(ids.filter((id, index) => ids.indexOf(id) !== index)).toEqual([]);
  });

  it("never lists one URL under two identifiers", () => {
    const owners = references().flatMap((reference) =>
      reference.urls.map((url) => ({ id: reference.id, url })),
    );
    const duplicates = owners.filter((owner) =>
      owners.some((other) => other.url === owner.url && other.id !== owner.id),
    );

    expect(duplicates).toEqual([]);
  });

  it("gives every row a well-formed status", () => {
    expect(
      references()
        .filter((reference) => !STATUS.test(reference.status))
        .map((reference) => reference.id),
    ).toEqual([]);
  });
});

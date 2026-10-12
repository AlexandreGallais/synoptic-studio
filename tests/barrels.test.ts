import { readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";

import { describe, expect, it } from "vitest";

/** Library root: every `index.ts` below it is a barrel. */
const SOURCE = join(import.meta.dirname, "..", "src");

/**
 * Every barrel file of the library.
 *
 * @returns absolute paths of the `index.ts` files under `src/`
 */
function barrels(): string[] {
  return readdirSync(SOURCE, { recursive: true, withFileTypes: true })
    .filter((entry) => entry.isFile() && entry.name === "index.ts")
    .map((entry) => join(entry.parentPath, entry.name));
}

/**
 * Specifiers re-exported as types only (`export type * from "./x";`) by a barrel.
 *
 * @param barrel - absolute path of the barrel
 * @returns absolute paths of the modules re-exported as types
 */
function typeOnlyTargets(barrel: string): string[] {
  return readFileSync(barrel, "utf8")
    .matchAll(/^export type \* from "(?<specifier>[^"]+)";$/gmu)
    .map((match) => join(dirname(barrel), match.groups?.["specifier"] ?? ""))
    .toArray();
}

/**
 * Names of the runtime values a module exports.
 *
 * @param target - absolute path of the module
 * @returns its exported names, types excluded (they do not exist at run time)
 */
async function runtimeExports(target: string): Promise<string[]> {
  // eslint-disable-next-line no-restricted-syntax -- loading each re-exported module is the point of this test.
  const module: unknown = await import(target);

  return typeof module === "object" && module !== null ? Object.keys(module) : [];
}

describe("barrels", () => {
  it("re-export as types only the modules that export no runtime value", async () => {
    const targets = barrels().flatMap((barrel) => typeOnlyTargets(barrel));
    const modules = await Promise.all(
      targets.map(async (target) => ({ exported: await runtimeExports(target), target })),
    );

    expect(
      modules.filter((module) => module.exported.length > 0).map((module) => module.target),
    ).toEqual([]);
  });
});

import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { RuleTester } from "@typescript-eslint/rule-tester";
import { afterAll } from "vitest";

import { FOLDER_LAYOUT_RULE } from "./folder-layout";

/** Fixture tree: a folder of domains, a mixed one, full ones, listed ones, one at the signal. */
const ROOT = mkdtempSync(join(tmpdir(), "folder-layout-"));

/**
 * Writes empty TypeScript files into a fixture folder, creating it.
 *
 * @param folder - path below the fixture root
 * @param names - file names
 */
function writeFiles(folder: string, names: readonly string[]): void {
  mkdirSync(join(ROOT, folder), { recursive: true });

  for (const name of names) {
    writeFileSync(join(ROOT, folder, name), "export {};\n");
  }
}

/** Three modules, a test and a barrel: past a signal of 2. */
const PAST = ["m1.ts", "m2.ts", "m3.ts", "m1.test.ts", "index.ts"];

writeFiles("domains/a", []);
writeFiles("snapshots/__snapshots__", []);
writeFiles("snapshots/.cache", []);
writeFiles("mixed/a", []);
writeFiles("full", PAST);
writeFiles("listed", PAST);
writeFiles("a/listed", PAST);
writeFiles("b/listed", PAST);
// Two modules, at the signal and not past it, once the test and the barrel are left out.
writeFiles("even", ["m1.ts", "m2.ts", "m1.test.ts", "index.ts"]);

afterAll(() => {
  rmSync(ROOT, { force: true, recursive: true });
});

/** Rule tester bound to Vitest (vitest.setup.ts). */
const TESTER = new RuleTester();
/** Rule options shared by the cases: a signal of 2, two folders listed by their path. */
const OPTIONS = [{ listed: ["listed", "a/listed"], root: ROOT, signal: 2 }] as const;
/** Code of every case: the rule reads only the file's place. */
const CODE = "export {};";

TESTER.run("folder-layout", FOLDER_LAYOUT_RULE, {
  invalid: [
    {
      code: CODE,
      errors: [{ data: { name: "b.ts", path: "mixed" }, messageId: "mixed" }],
      filename: join(ROOT, "mixed/b.ts"),
      options: OPTIONS,
    },
    {
      code: CODE,
      errors: [{ data: { name: "b.test.ts", path: "mixed" }, messageId: "mixed" }],
      filename: join(ROOT, "mixed/b.test.ts"),
      options: OPTIONS,
    },
    {
      code: CODE,
      errors: [{ data: { modules: 3, path: "full", signal: 2 }, messageId: "past" }],
      filename: join(ROOT, "full/m1.ts"),
      options: OPTIONS,
    },
    {
      code: CODE,
      errors: [{ data: { modules: 3, path: "b/listed", signal: 2 }, messageId: "past" }],
      filename: join(ROOT, "b/listed/m1.ts"),
      options: OPTIONS,
    },
  ],
  valid: [
    { code: CODE, filename: join(ROOT, "domains/index.ts"), options: OPTIONS },
    { code: CODE, filename: join(ROOT, "domains/a/x.ts"), options: OPTIONS },
    { code: CODE, filename: join(ROOT, "full/m1.test.ts"), options: OPTIONS },
    { code: CODE, filename: join(ROOT, "full/index.ts"), options: OPTIONS },
    { code: CODE, filename: join(ROOT, "listed/m1.ts"), options: OPTIONS },
    { code: CODE, filename: join(ROOT, "a/listed/m1.ts"), options: OPTIONS },
    { code: CODE, filename: join(ROOT, "even/m1.ts"), options: OPTIONS },
    { code: CODE, filename: join(ROOT, "snapshots/m1.test.ts"), options: OPTIONS },
  ],
});

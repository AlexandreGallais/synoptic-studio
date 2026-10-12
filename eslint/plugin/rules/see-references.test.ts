import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { RuleTester } from "@typescript-eslint/rule-tester";

import { SEE_REFERENCES_RULE } from "./see-references";

/** Fixture documentation: one verified and one unverified reference, one derivation, one ADR. */
const ROOT = mkdtempSync(join(tmpdir(), "see-references-"));

mkdirSync(join(ROOT, "docs/adr"), { recursive: true });
mkdirSync(join(ROOT, "docs/derivations"));
mkdirSync(join(ROOT, "src"));

writeFileSync(
  join(ROOT, "docs/references.md"),
  "| ID | Ref | Use | Status |\n|---|---|---|---|\n| `REF-OK` | x | y | [verified] |\n| `REF-TODO` | x | y | [unverified] |\n",
);

writeFileSync(join(ROOT, "docs/derivations/fit.md"), "# fit\n");
writeFileSync(join(ROOT, "docs/adr/0001-scope.md"), "# scope\n");

/** Rule tester bound to Vitest (vitest.setup.ts). */
const TESTER = new RuleTester();
/** Linted file inside the fixture repository. */
const FILENAME = join(ROOT, "src/a.ts");
/** Reference locations of the fixture repository. */
const SOURCES = {
  adrDirectory: "docs/adr",
  derivationsDirectory: "docs/derivations",
  referencesFile: "docs/references.md",
  unverifiedMarker: "[unverified]",
};
/** Rule options shared by the cases. */
const OPTIONS = [{ requiredKinds: ["math"], root: ROOT, sources: SOURCES }] as const;

/**
 * Source of a math function documented with the given `@see` target.
 *
 * @param target - `@see` value
 * @returns TypeScript source
 */
function citing(target: string): string {
  return `/**\n * Doc.\n *\n * @kind math\n * @see ${target}\n */\nfunction f(): void {}`;
}

TESTER.run("see-references", SEE_REFERENCES_RULE, {
  invalid: [
    {
      code: "/** @kind math */\nfunction f(): void {}",
      errors: [{ messageId: "missingSee" }],
      filename: FILENAME,
      options: OPTIONS,
    },
    {
      code: citing("REF-NOPE"),
      errors: [{ messageId: "missing" }],
      filename: FILENAME,
      options: OPTIONS,
    },
    {
      code: citing("REF-TODO"),
      errors: [{ messageId: "unverified" }],
      filename: FILENAME,
      options: OPTIONS,
    },
    {
      code: citing("DERIV-nope"),
      errors: [{ messageId: "missing" }],
      filename: FILENAME,
      options: OPTIONS,
    },
    {
      code: citing("ADR-0002"),
      errors: [{ messageId: "missing" }],
      filename: FILENAME,
      options: OPTIONS,
    },
    {
      code: citing("docs/nope.md"),
      errors: [{ messageId: "missing" }],
      filename: FILENAME,
      options: OPTIONS,
    },
    {
      code: citing("https://example.org"),
      errors: [{ messageId: "unknown-format" }],
      filename: FILENAME,
      options: OPTIONS,
    },
  ],
  valid: [
    { code: citing("REF-OK"), filename: FILENAME, options: OPTIONS },
    { code: citing("DERIV-fit"), filename: FILENAME, options: OPTIONS },
    { code: citing("ADR-0001"), filename: FILENAME, options: OPTIONS },
    { code: citing("docs/references.md#section"), filename: FILENAME, options: OPTIONS },
    { code: "/** @kind procedure */\nfunction f(): void {}", filename: FILENAME, options: OPTIONS },
  ],
});

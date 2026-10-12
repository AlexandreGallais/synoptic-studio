import { RuleTester } from "@typescript-eslint/rule-tester";

import { ONE_EXPORT_PER_FILE_RULE } from "./one-export-per-file";

/** Rule tester bound to Vitest (vitest.setup.ts). */
const TESTER = new RuleTester();

/** Module named after the function `filletSetback`. */
const FILLET_SETBACK = "/project/src/geometry/fillet-setback.ts";

/** Module named after the type `Point`. */
const POINT = "/project/src/math/point.ts";

TESTER.run("one-export-per-file", ONE_EXPORT_PER_FILE_RULE, {
  invalid: [
    {
      code: "export function filletSetback(): void {}",
      errors: [{ messageId: "filenameMismatch" }],
      filename: "/project/src/geometry/filletSetback.ts",
    },
    {
      code: "export function other(): void {}",
      errors: [{ messageId: "filenameMismatch" }],
      filename: FILLET_SETBACK,
    },
    {
      code: "export function filletSetback(): void {}\nexport function other(): void {}",
      errors: [{ messageId: "multipleValues" }],
      filename: FILLET_SETBACK,
    },
    {
      code: "export type Unrelated = number;\nexport function filletSetback(): void {}",
      errors: [{ messageId: "unrelatedType" }],
      filename: FILLET_SETBACK,
    },
    {
      code: "export type Point = number;\nexport type Vector = number;",
      errors: [{ messageId: "multipleTypes" }],
      filename: POINT,
    },
    { code: "const A = 1;", errors: [{ messageId: "noExport" }], filename: FILLET_SETBACK },
    { code: 'export * from "./x";', errors: [{ messageId: "reexport" }], filename: FILLET_SETBACK },
    {
      code: "function filletSetback(): void {}\nexport { filletSetback };",
      errors: [{ messageId: "exportList" }],
      filename: FILLET_SETBACK,
    },
  ],
  valid: [
    { code: "export function filletSetback(): void {}", filename: FILLET_SETBACK },
    { code: "export type Point = { readonly x: number };", filename: POINT },
    {
      // The type is part of the function's contract: allowed next to it.
      code: "export type Setback = number;\nexport type Corner = number;\nexport function filletSetback(corner: Corner): Setback { return corner; }",
      filename: FILLET_SETBACK,
    },
    {
      code: "type Local = number;\nconst HALF: Local = 2;\nexport function filletSetback(): Local { return HALF; }",
      filename: FILLET_SETBACK,
    },
    { code: "export const filletSetback = 1;", filename: FILLET_SETBACK },
    { code: "export const SVG_DECIMALS = 5;", filename: "/project/src/io/svg-decimals.ts" },
    { code: "export function toSVGPath(): void {}", filename: "/project/src/io/to-svg-path.ts" },
  ],
});

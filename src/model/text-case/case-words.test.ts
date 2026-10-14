import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { caseWords } from "./case-words";

describe("caseWords (Q36)", () => {
  it.each([
    { expected: ["pump", "P", "101", "is", "running"], line: "pump P-101 is running" },
    { expected: ["a", "b", "c"], line: "a_b-c" },
    // Runs of separators and separators at the ends give no empty word.
    { expected: ["a", "b"], line: " -a__ b- " },
    { expected: [], line: "" },
    { expected: [], line: "-_ " },
  ])("cuts '$line' into $expected", ({ expected, line }) => {
    expect(caseWords(line)).toEqual(expected);
  });

  test.prop({ line: fc.string({ maxLength: 30 }) })(
    "gives non-empty words without separators, which join back to the line's other characters",
    ({ line }) => {
      const words = caseWords(line);

      expect(words.every((word) => word !== "" && !/[ _-]/u.test(word))).toBe(true);
      expect(words.join("")).toBe(line.replaceAll(/[ _-]/gu, ""));
    },
  );
});

import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { textInCase } from "./text-in-case";

import type { TextCase } from "../text";

/** Every case of a text. */
const CASES: readonly TextCase[] = [
  "none",
  "upper",
  "lower",
  "sentence",
  "title",
  "camel",
  "pascal",
  "kebab",
  "snake",
  "constant",
];

/** Words of letters and digits, joined by spaces, hyphens and underscores. */
const LABEL = fc.stringMatching(/^[A-Za-z0-9 _-]{0,30}$/u);

describe("textInCase (Q36)", () => {
  it.each([
    { expected: "pump P-101 is running", textCase: "none" },
    { expected: "PUMP P-101 IS RUNNING", textCase: "upper" },
    { expected: "pump p-101 is running", textCase: "lower" },
    // The first character capitalized, the rest kept: P-101 stays in upper case.
    { expected: "Pump P-101 is running", textCase: "sentence" },
    // The first character after each space capitalized: "P-101" is one word, already upper.
    { expected: "Pump P-101 Is Running", textCase: "title" },
    // Words cut at spaces and hyphens: pump, p, 101, is, running.
    { expected: "pumpP101IsRunning", textCase: "camel" },
    { expected: "PumpP101IsRunning", textCase: "pascal" },
    { expected: "pump-p-101-is-running", textCase: "kebab" },
    { expected: "pump_p_101_is_running", textCase: "snake" },
    { expected: "PUMP_P_101_IS_RUNNING", textCase: "constant" },
  ] as const)(
    "[F05.AC4] writes 'pump P-101 is running' in $textCase case as '$expected'",
    ({ expected, textCase }) => {
      expect(textInCase("pump P-101 is running", textCase)).toBe(expected);
    },
  );

  it("cuts words at underscores and runs of separators, and lowers the rest of each word", () => {
    // Words: valve, ON, off → valve, on, off.
    expect(textInCase("valve__ON - off", "camel")).toBe("valveOnOff");
    expect(textInCase("valve__ON - off", "kebab")).toBe("valve-on-off");
  });

  it("writes each line in the case, its line breaks kept", () => {
    expect(textInCase("pump on\nvalve off", "kebab")).toBe("pump-on\nvalve-off");
    expect(textInCase("pump on\nvalve off", "sentence")).toBe("Pump on\nValve off");
  });

  it("capitalizes a whole code point, beyond the basic plane", () => {
    // Deseret small long i (U+10428) has the capital U+10400.
    expect(textInCase("\u{10428}x", "sentence")).toBe("\u{10400}x");
    expect(textInCase("éclair", "title")).toBe("Éclair");
  });

  it.each(CASES)("writes an empty content as empty in %s case", (textCase) => {
    expect(textInCase("", textCase)).toBe("");
  });

  it("writes a line of separators only as empty in the programmers' cases", () => {
    expect(textInCase(" -_ ", "pascal")).toBe("");
    expect(textInCase(" -_ ", "constant")).toBe("");
  });

  test.prop({ content: LABEL })(
    "writes constant case as kebab case in upper case, hyphens become underscores",
    ({ content }) => {
      expect(textInCase(content, "constant")).toBe(
        textInCase(content, "kebab").toUpperCase().replaceAll("-", "_"),
      );
    },
  );

  test.prop({ lines: fc.array(LABEL, { maxLength: 4, minLength: 1 }) })(
    "keeps the number of lines in every case",
    ({ lines }) => {
      const content = lines.join("\n");

      expect(CASES.map((textCase) => textInCase(content, textCase).split("\n").length)).toEqual(
        CASES.map(() => lines.length),
      );
    },
  );
});

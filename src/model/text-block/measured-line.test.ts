import { describe, expect, it } from "vitest";

import { ARIAL_METRICS } from "../../../tests";

import { measuredLine } from "./measured-line";

describe("measuredLine (DERIV-text-width steps 1 to 3)", () => {
  it("measures 'Wo W' in font units, with its units, separators and ends", () => {
    // W 1933 + o 1139 − 37 (W o) + space 569 + W 1933 = 5537 units; 4 units, one separator.
    expect(measuredLine(ARIAL_METRICS, "Wo W")).toEqual({
      advance: 5537,
      first: 87,
      last: 87,
      separatorCount: 1,
      text: "Wo W",
      unitCount: 4,
    });
  });

  it("leaves out formatting characters from its ends, and has none for an empty line", () => {
    // A zero-width joiner (Cf) is measured as absent: the first and last are o.
    expect(measuredLine(ARIAL_METRICS, "\u{200D}o\u{200D}")).toMatchObject({
      advance: 1139,
      first: 111,
      last: 111,
      unitCount: 1,
    });

    expect(measuredLine(ARIAL_METRICS, "")).toEqual({
      advance: 0,
      first: -1,
      last: -1,
      separatorCount: 0,
      text: "",
      unitCount: 0,
    });
  });
});

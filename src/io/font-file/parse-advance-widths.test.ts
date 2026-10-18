import { describe, expect, it } from "vitest";

import { uint16, viewOf } from "../../../tests";

import { parseAdvanceWidths } from "./parse-advance-widths";

/** An `hmtx` table of three records: advances 500, 600, 700, each with a left side bearing 0. */
const HMTX = [500, 600, 700].flatMap((advance) => [...uint16(advance), ...uint16(0)]);

describe("parseAdvanceWidths (REF-OT-HMTX)", () => {
  it("reads the advance of each record", () => {
    expect(parseAdvanceWidths(viewOf(HMTX), 3)).toEqual([500, 600, 700]);
  });

  it("reads only the records the table holds when numberOfHMetrics asks for more", () => {
    // numberOfHMetrics 5, but the table's 12 bytes hold 3 records.
    expect(parseAdvanceWidths(viewOf(HMTX), 5)).toEqual([500, 600, 700]);
  });
});

import { describe, expect, it } from "vitest";

import { readTurn, turnsTable } from "./turns";

describe("turns", () => {
  const start = Temporal.Instant.from("2026-10-13T08:30:00Z").epochMilliseconds;
  // A turn of 10 minutes, as the session report saves it (tools/metrics, turn_record).
  const record = {
    activeMs: 360_000,
    calls: 4,
    commits: 2,
    failures: 1,
    outputTokens: 75,
    session: "168220a7-35a5",
    start,
    tokens: 3279,
    wallMs: 600_000,
  };

  it("reads a saved line back, a missing field as 0 or empty", () => {
    const saved: unknown = structuredClone(record);

    expect(readTurn(saved)).toEqual(record);
    expect(readTurn(JSON.parse("{}"))).toMatchObject({ calls: 0, session: "" });
  });

  it("lists each turn once, its last figures, oldest first", () => {
    const later = { ...record, start: start + 3_600_000 };
    const table = turnsTable([later, record, { ...record, calls: 9 }]);
    const rows = table.split("\n").slice(2);

    expect(rows).toEqual([
      "| 2026-10-13T08:30 | 168220a7 | 10 min 00 s | 6 min 00 s | 3,279 | 75 | 9 | 1 | 2 |",
      "| 2026-10-13T09:30 | 168220a7 | 10 min 00 s | 6 min 00 s | 3,279 | 75 | 4 | 1 | 2 |",
    ]);
  });
});

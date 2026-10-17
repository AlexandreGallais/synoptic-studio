import { describe, expect, it } from "vitest";

import { classesByCharacter } from "./classes-by-character";

describe("classesByCharacter (REF-OT-GPOS)", () => {
  it("keeps the classes other than 0, by code point", () => {
    expect(
      classesByCharacter(
        [
          [65, 1],
          [66, 2],
          [67, 3],
        ],
        [2, 0, 1],
      ),
    ).toEqual({ 65: 2, 67: 1 });
  });

  it("leaves out a character past the classes given", () => {
    expect(
      classesByCharacter(
        [
          [65, 1],
          [66, 2],
        ],
        [1],
      ),
    ).toEqual({ 65: 1 });
  });
});

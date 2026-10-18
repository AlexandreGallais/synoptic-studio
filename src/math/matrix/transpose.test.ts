import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { transpose } from "./transpose";

describe("transpose (REF-MATHWORLD-TRANSPOSE)", () => {
  it("exchanges the rows and columns of a 2 × 3 matrix", () => {
    // Row 0 (1 2 3) becomes column 0, row 1 (4 5 6) column 1.
    expect(
      transpose([
        [1, 2, 3],
        [4, 5, 6],
      ]),
    ).toEqual([
      [1, 4],
      [2, 5],
      [3, 6],
    ]);
  });

  it("reads an element missing from a shorter row as 0", () => {
    // Two columns, from the first row; the second row lacks its second element.
    expect(transpose([[1, 2], [3]])).toEqual([
      [1, 3],
      [2, 0],
    ]);
  });

  it("takes as many columns as the longest row, whichever row it is", () => {
    // The second row is the longest: two columns, the first row lacking its second element.
    expect(transpose([[1], [2, 3]])).toEqual([
      [1, 2],
      [0, 3],
    ]);
  });

  it("gives an empty matrix for an empty one", () => {
    expect(transpose([])).toEqual([]);
  });

  test.prop({
    matrix: fc
      .tuple(fc.integer({ max: 6, min: 1 }), fc.integer({ max: 6, min: 1 }))
      .chain(([rows, columns]) =>
        fc.array(fc.array(fc.integer(), { maxLength: columns, minLength: columns }), {
          maxLength: rows,
          minLength: rows,
        }),
      ),
  })("gives back the matrix when applied twice, and moves a(i, j) to (j, i)", ({ matrix }) => {
    const transposed = transpose(matrix);

    expect(transpose(transposed)).toEqual(matrix);

    expect(transposed[(matrix[0]?.length ?? 1) - 1]?.[matrix.length - 1]).toBe(
      matrix.at(-1)?.at(-1),
    );
  });
});

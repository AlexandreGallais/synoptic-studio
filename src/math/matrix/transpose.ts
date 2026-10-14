import type { MatrixRow } from "./matrix-row";

/**
 * The transpose of a matrix: its rows and columns exchanged, the element of row i and column j
 * moved to row j and column i. A matrix whose rows differ in length has as many columns as its
 * longest row, an element missing from a shorter row read as 0.
 *
 * @kind math
 * @param matrix - its rows; an empty matrix has no column
 * @returns its columns, as rows
 * @see REF-MATHWORLD-TRANSPOSE
 */
export function transpose(matrix: readonly MatrixRow[]): readonly MatrixRow[] {
  return Array.from({ length: Math.max(0, ...matrix.map((row) => row.length)) }, (unused, column) =>
    matrix.map((row) => row[column] ?? 0),
  );
}

import type { MatrixRow } from "./matrix-row";

/**
 * The transpose of a matrix: its rows and columns exchanged, the element of row i and column j
 * moved to row j and column i.
 *
 * @kind math
 * @param matrix - rows of equal length; an empty matrix has no column, and an element missing
 * from a shorter row is read as 0
 * @returns its columns, as rows
 * @see REF-MATHWORLD-TRANSPOSE
 */
export function transpose(matrix: readonly MatrixRow[]): readonly MatrixRow[] {
  return (matrix[0] ?? []).map((unused, column) => matrix.map((row) => row[column] ?? 0));
}

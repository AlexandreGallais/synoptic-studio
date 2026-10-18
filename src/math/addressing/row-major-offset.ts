/**
 * The offset of an element of a two-dimensional array laid out in row-major order, each row's
 * elements consecutive and every row before the next: the rows before it, then its column. Of
 * 3 columns, row 1 column 2 is element 5.
 *
 * Formula: n₂ + N₂ · n₁, written N₂ · n₁ + n₂, both indexes from 0.
 *
 * @kind math
 * @param row - the row's index, n₁
 * @param column - the column's index, n₂, from 0 to `columnCount − 1`
 * @param columnCount - the number of columns, N₂
 * @returns the element's offset, in elements
 * @see REF-WIKI-ROW-MAJOR
 */
export function rowMajorOffset(row: number, column: number, columnCount: number): number {
  return columnCount * row + column;
}

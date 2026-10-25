/**
 * Rows of the list of points, as shown.
 *
 * @kind procedure
 * @returns the text of each row, in contour order
 * @see EN-021
 */
export function readPointRows(): readonly string[] {
  return [...document.querySelectorAll("#points [data-vertex]")].map((row) => row.textContent);
}

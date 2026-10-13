/**
 * Rows of the list of points, as shown.
 *
 * @kind procedure
 * @returns the text of each row, in contour order
 * @see docs/backlog/stories/E14-F11-EN-021-shared-test-helpers.md
 */
export function readPointRows(): readonly string[] {
  return [...document.querySelectorAll("#points [data-vertex]")].map((row) => row.textContent);
}

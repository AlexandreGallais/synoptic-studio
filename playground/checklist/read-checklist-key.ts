/**
 * The key a checklist's ticks are kept under in the browser's storage, one per checklist.
 *
 * @kind procedure
 * @param name - the checklist's name, `E15`
 * @returns its storage key, `synoptic-studio.checklist.E15`
 * @see docs/backlog/stories/E15-F17-US-051-validation-checklist-page.md
 */
export function readChecklistKey(name: string): string {
  return `synoptic-studio.checklist.${name}`;
}

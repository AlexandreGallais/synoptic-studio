/**
 * The key a checklist's ticks are kept under in the browser's storage, one per checklist.
 *
 * @kind procedure
 * @param name - the checklist's name, `E15`
 * @returns its storage key, `synoptic-studio.checklist.E15`
 */
export function readChecklistKey(name: string): string {
  return `synoptic-studio.checklist.${name}`;
}

import { selectInput } from "./select-input";

/**
 * Chooses files in a file picker, as a user would: its `files`, then its `change` event
 * (`REF-MDN-INPUT-FILE`).
 *
 * @kind procedure
 * @param id - id of the picker, without `#`
 * @param files - the files chosen
 * @see docs/backlog/stories/E15-F15-US-036-fonts-embedding-and-html-layer.md
 */
export function chooseFiles(id: string, files: readonly File[]): void {
  const picker = selectInput(id);

  Object.defineProperty(picker, "files", { configurable: true, value: files });
  picker.dispatchEvent(new Event("change", { bubbles: true }));
}

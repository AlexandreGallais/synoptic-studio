import { selectInput } from "./select-input";

/**
 * Chooses files in a file picker, as a user would: its `files`, then its `change` event
 * (`REF-MDN-INPUT-FILE`).
 *
 * @kind procedure
 * @param id - id of the picker, without `#`
 * @param files - the files chosen
 */
export function chooseFiles(id: string, files: readonly File[]): void {
  const picker = selectInput(id);

  Object.defineProperty(picker, "files", { configurable: true, value: files });
  picker.dispatchEvent(new Event("change", { bubbles: true }));
}

import { readFontSession } from "../fonts";
import { selectElement } from "../page";

/**
 * Enables or disables a control of the page.
 *
 * @kind procedure
 * @param document - page document
 * @param id - the control's `id`, without `#`
 * @param isDisabled - whether it cannot be used
 * @see docs/backlog/stories/E15-F17-US-059-approved-fonts-only.md
 */
function writeDisabled(document: Document, id: string, isDisabled: boolean): void {
  selectElement(document, id).toggleAttribute("disabled", isDisabled);
}

/**
 * Lets the designer use the font controls only when they lead to a font read and approved
 * (US-059): while the installed fonts the browser listed are read, the font list and
 * **Installed fonts** are disabled, so that no font is chosen during the reading; while no font
 * is available, the font list, empty, and the Content field are disabled, so that no text is
 * written before a font is.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/backlog/stories/E15-F17-US-059-approved-fonts-only.md
 */
export function writeFontAvailability(document: Document): void {
  const session = readFontSession(document);
  const isReading = session.installed.length > 0;
  const isFontMissing = session.kept.length === 0;

  writeDisabled(document, "text-font", isReading || isFontMissing);
  writeDisabled(document, "installed-fonts", isReading);
  writeDisabled(document, "text-content", isFontMissing);
}

import { readFontSession } from "../fonts";
import { selectElement } from "../page";

/**
 * Enables or disables a control of the page.
 *
 * @kind procedure
 * @param document - page document
 * @param id - the control's `id`, without `#`
 * @param isDisabled - whether it cannot be used
 * @see US-059
 */
function writeDisabled(document: Document, id: string, isDisabled: boolean): void {
  selectElement(document, id).toggleAttribute("disabled", isDisabled);
}

/**
 * Lets the designer use the font controls only when they lead to a font read and approved
 * (US-059): while the installed fonts the browser listed are read, ahead and then the default
 * font's files again (US-061), the font list and **Installed fonts** are disabled, so that no
 * font is chosen during the reading; while no font
 * is available, the font list, empty, and the Content field are disabled, so that no text is
 * written before a font is.
 *
 * @kind procedure
 * @param document - page document
 * @see US-059
 */
export function writeFontAvailability(document: Document): void {
  const session = readFontSession(document);
  const { isReading } = session;
  const isFontMissing = session.kept.length === 0;

  writeDisabled(document, "text-font", isReading || isFontMissing);
  writeDisabled(document, "installed-fonts", isReading);
  writeDisabled(document, "text-content", isFontMissing);
}

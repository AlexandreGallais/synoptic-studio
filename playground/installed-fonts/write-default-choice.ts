import { isSameFamilyName } from "../../src";
import { readFontSession, writeFontOptions, writeFontSession } from "../fonts";
import { writeText } from "../page";

/**
 * Chooses a default font once a family is read while none is available (US-056, provisional):
 * when the family the text is measured with has no metrics kept — Inter, not drawn by the browser,
 * left out of the font list —, the first family of the list with metrics is chosen and measured
 * with, Inter first when it is kept, and the page says so. A family already measured with, chosen
 * or imported, is left as it is.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/shapes.md#text
 */
export function writeDefaultChoice(document: Document): void {
  const session = readFontSession(document);
  const [first] = session.kept;

  if (
    first === undefined ||
    session.kept.some((kept) => isSameFamilyName(kept.name, session.measured))
  ) {
    return;
  }

  writeFontSession(document, { ...session, measured: first.name });
  writeFontOptions(document, first.name);

  writeText(
    document,
    "font-message",
    `${first.name} is chosen, the first font read: Inter is not installed on this computer.`,
  );
}

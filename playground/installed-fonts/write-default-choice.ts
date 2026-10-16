import { isSameFamilyName } from "../../src";
import { readFontSession, writeFontOptions, writeFontSession } from "../fonts";
import { writeText } from "../page";

/**
 * Chooses a default font once a family is read and approved while none is available (US-056):
 * when the family the text is measured with has no metrics kept — Inter, not drawn by the browser,
 * left out of the font list —, the first family of the list, read and approved, is chosen and
 * measured with, and the page says so. A font available, Inter drawn, a font imported or the
 * family chosen so, is left as it is; the font list cannot be used during the reading (US-059).
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

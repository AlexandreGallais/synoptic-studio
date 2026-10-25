import { firstOfEachClass } from "../../math";
import { caseFoldedFamilyName } from "../../model";

import { formatFontFamilyName } from "./format-font-family-name";

/** The generic family every list ends with, so that a reader without either font still draws the text. */
const LAST_FAMILY = "sans-serif";

/**
 * The `font-family` of a text: its font, then its fallback, then `sans-serif`, each name once
 * (names matched without regard to case, as CSS matches them, and without the white space at
 * their ends: the first of each class, `firstOfEachClass`), separated by commas (CSS Fonts 4,
 * §2.1); `Inter` with the fallback `sans-serif` gives `"Inter", sans-serif`.
 *
 * @kind format
 * @param font - the family the text names first
 * @param fallback - the family named after it
 * @returns the list, written for a `font-family` attribute
 */
export function formatFontFamily(font: string, fallback: string): string {
  const names = firstOfEachClass([font, fallback, LAST_FAMILY], (name) =>
    caseFoldedFamilyName(name.trim()),
  );

  return names.map((name) => formatFontFamilyName(name)).join(", ");
}

import { isSameFamilyName } from "../../model";

import { formatFontFamilyName } from "./format-font-family-name";

/** The generic family every list ends with, so that a reader without either font still draws the text (F05). */
const LAST_FAMILY = "sans-serif";

/**
 * The `font-family` of a text: its font, then its fallback, then `sans-serif`, each name once
 * (names matched without regard to case, as CSS matches them), separated by commas (CSS Fonts 4,
 * §2.1); `Inter` with the fallback `sans-serif` gives `"Inter", sans-serif`.
 *
 * @kind format
 * @param font - the family the text names first
 * @param fallback - the family named after it
 * @returns the list, written for a `font-family` attribute
 * @see REF-CSS-FONTS-4
 */
export function formatFontFamily(font: string, fallback: string): string {
  const names = [font, fallback, LAST_FAMILY].filter(
    (name, index, all) => all.findIndex((other) => isSameFamilyName(name, other)) === index,
  );

  return names.map((name) => formatFontFamilyName(name)).join(", ");
}

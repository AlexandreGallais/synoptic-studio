import { firstOfEachClass } from "../../math";
import { caseFoldedFamilyName, isSameFamilyName } from "../font";
import { faceStyleKey } from "../font-import";
import { textFontFace } from "../text-font";

import type { UsedFace } from "./used-face";
import type { FontFamily } from "../font";
import type { NamedFontFace } from "../font-import";
import type { Text } from "../text";

/**
 * The key of a face: its weight and style (`faceStyleKey`), then its family's name folded to
 * match without regard to case (`caseFoldedFamilyName`): two faces are one when their keys are
 * equal (`REF-CSS-FONTS-4`, §4.1).
 *
 * @kind domain
 * @param named - a face and its family
 * @returns its key
 * @see DERIV-font-embedding
 */
function faceKey(named: NamedFontFace): string {
  return `${faceStyleKey(named.face)}/${caseFoldedFamilyName(named.family)}`;
}

/**
 * Whether a text names a family, as its font or its fallback, without regard to case.
 *
 * @kind domain
 * @param text - its font and its fallback
 * @param family - a family's name
 * @returns true when either names it
 * @see REF-CSS-FONTS-4
 */
function isNamedBy(text: Text, family: string): boolean {
  return isSameFamilyName(family, text.font) || isSameFamilyName(family, text.fallback);
}

/**
 * The face a text is measured with (`textFontFace`), and whether the text names its family.
 *
 * @kind domain
 * @param text - a text
 * @param fonts - the font families read
 * @returns the face, alone; none for a text measured with no face
 * @see DERIV-font-embedding
 */
function textUsedFace(text: Text, fonts: readonly FontFamily[]): readonly UsedFace[] {
  const named = textFontFace(text, fonts);

  return named === undefined ? [] : [{ isNamed: isNamedBy(text, named.family), named }];
}

/**
 * The faces texts are measured with, each once, in the order of their first use, the first of
 * each class of equal keys (`faceKey`, `firstOfEachClass`; `DERIV-font-embedding`, step 7),
 * each named when one of its texts names its family.
 *
 * @kind domain
 * @param texts - the document's texts
 * @param fonts - the font families read
 * @returns the distinct faces
 * @see DERIV-font-embedding
 */
export function usedFaces(
  texts: readonly Text[],
  fonts: readonly FontFamily[],
): readonly UsedFace[] {
  const used = texts.flatMap((text) => textUsedFace(text, fonts));
  const named = new Set(used.filter((face) => face.isNamed).map((face) => faceKey(face.named)));

  return firstOfEachClass(used, (face) => faceKey(face.named)).map((face) => ({
    ...face,
    isNamed: named.has(faceKey(face.named)),
  }));
}

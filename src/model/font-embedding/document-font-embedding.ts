import { isSameFamilyName } from "../font";
import { textFontFace } from "../text-font";

import { leftOutReasonOf } from "./left-out-reason-of";

import type { DocumentOptions } from "../document";
import type { FontFamily } from "../font";
import type { NamedFontFace } from "../font-import";
import type { Text } from "../text";
import type { EmbeddedFace } from "./embedded-face";
import type { FontEmbedding } from "./font-embedding";
import type { LeftOutFace } from "./left-out-face";

/**
 * Whether two faces are one: the same family, its name matched without regard to case
 * (`isSameFamilyName`), weight and style (`REF-CSS-FONTS-4`, §4.1).
 *
 * @kind domain
 * @param face - a face and its family
 * @param other - another
 * @returns true when they are the same face
 * @see DERIV-font-embedding
 */
function isSameFace(face: NamedFontFace, other: NamedFontFace): boolean {
  return (
    isSameFamilyName(face.family, other.family) &&
    face.face.weight === other.face.weight &&
    face.face.isItalic === other.face.isItalic
  );
}

/**
 * The faces texts are measured with (`textFontFace`), each once, in the order of their first use
 * (`DERIV-font-embedding`, step 7); a text measured with no face adds none.
 *
 * @kind domain
 * @param texts - the document's texts
 * @param fonts - the font families read
 * @returns the distinct faces and their families
 * @see DERIV-font-embedding
 */
function usedFaces(texts: readonly Text[], fonts: readonly FontFamily[]): readonly NamedFontFace[] {
  const used = texts.flatMap((text) => textFontFace(text, fonts) ?? []);

  return used.filter(
    (named, index) => used.findIndex((other) => isSameFace(named, other)) === index,
  );
}

/**
 * The face embedded, when its file is held and may be embedded.
 *
 * @kind domain
 * @param named - a face used and its family
 * @returns the face to write, alone; none otherwise
 * @see DERIV-font-embedding
 */
function embeddedFace(named: NamedFontFace): readonly EmbeddedFace[] {
  const { file, isItalic, weight } = named.face;

  return file?.isEmbeddable === true ? [{ family: named.family, file, isItalic, weight }] : [];
}

/**
 * The face left out and why, when its file is not held or may not be embedded.
 *
 * @kind domain
 * @param named - a face used and its family
 * @returns the face and its reason, alone; none for a face embedded
 * @see DERIV-font-embedding
 */
function leftOutFace(named: NamedFontFace): readonly LeftOutFace[] {
  const { file, isItalic, weight } = named.face;
  const reason = leftOutReasonOf(file);

  return reason === undefined ? [] : [{ family: named.family, isItalic, reason, weight }];
}

/**
 * The faces a document embeds, its embedded fonts on (ADR-0053): the face each text is measured
 * with, each once whatever the number of its texts, in the order of its first use
 * (`DERIV-font-embedding`, step 7), when its file is held and its `fsType` permits it; the others
 * left out with their reason, no file or not permitted, for the editor to say so. Off, the
 * document embeds nothing and leaves nothing out: 10 texts in Arial 400 and 700 embed 2 faces.
 *
 * @kind domain
 * @param texts - the texts it holds, in their order
 * @param fonts - the font families read, the default font among them
 * @param options - whether its fonts are embedded
 * @returns the faces embedded and those left out
 * @see DERIV-font-embedding
 */
export function documentFontEmbedding(
  texts: readonly Text[],
  fonts: readonly FontFamily[],
  options: DocumentOptions,
): FontEmbedding {
  const faces = options.hasEmbeddedFonts ? usedFaces(texts, fonts) : [];

  return {
    embedded: faces.flatMap((named) => embeddedFace(named)),
    leftOut: faces.flatMap((named) => leftOutFace(named)),
  };
}

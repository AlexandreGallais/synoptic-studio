import { isSameFamilyName } from "../font";
import { isGenericFamilyName } from "../text";
import { textFontFace } from "../text-font";

import { leftOutReasonOf } from "./left-out-reason-of";

import type { DocumentOptions } from "../document";
import type { FontFamily } from "../font";
import type { NamedFontFace } from "../font-import";
import type { Text } from "../text";
import type { EmbeddedFace } from "./embedded-face";
import type { FontEmbedding } from "./font-embedding";
import type { LeftOutFace } from "./left-out-face";
import type { LeftOutReason } from "./left-out-reason";

/** A face texts are measured with, and whether one of them names its family. */
type UsedFace = {
  /** The face and its family's name, as read. */
  readonly named: NamedFontFace;
  /** Whether a text names its family, as its font or its fallback; false for a face reached only as the default font. */
  readonly isNamed: boolean;
};

/** A face used, and why it is left out. */
type JudgedFace = UsedFace & {
  /** Why it is not embedded; undefined for a face embedded. */
  readonly reason: LeftOutReason | undefined;
};

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
 * The faces texts are measured with, each once, in the order of their first use
 * (`DERIV-font-embedding`, step 7), each named when one of its texts names its family.
 *
 * @kind domain
 * @param texts - the document's texts
 * @param fonts - the font families read
 * @returns the distinct faces
 * @see DERIV-font-embedding
 */
function usedFaces(texts: readonly Text[], fonts: readonly FontFamily[]): readonly UsedFace[] {
  const used = texts.flatMap((text) => textUsedFace(text, fonts));

  return used
    .filter(
      (face, index) => used.findIndex((other) => isSameFace(face.named, other.named)) === index,
    )
    .map((face) => ({
      ...face,
      isNamed: used.some((other) => other.isNamed && isSameFace(face.named, other.named)),
    }));
}

/**
 * Why a face used is left out, for itself: a family named like a generic family, which an
 * `@font-face` rule may not define; a face reached only as the default font, which no text's
 * `font-family` names; else no file held or not permitted (`leftOutReasonOf`).
 *
 * @kind domain
 * @param face - a face used
 * @returns its reason; undefined for a face that may be embedded
 * @see REF-CSS-FONTS-4
 */
function faceReason(face: UsedFace): LeftOutReason | undefined {
  if (isGenericFamilyName(face.named.family)) {
    return "generic-name";
  }

  return face.isNamed ? leftOutReasonOf(face.named.face.file) : "not-named";
}

/**
 * Whether a face of a family is left out.
 *
 * @kind domain
 * @param judged - the faces used and their reasons
 * @param family - a family's name
 * @returns true when one of its faces has a reason
 * @see REF-CSS-FONTS-4
 */
function isIncomplete(judged: readonly JudgedFace[], family: string): boolean {
  return judged.some(
    (other) => other.reason !== undefined && isSameFamilyName(other.named.family, family),
  );
}

/**
 * The faces used with their reasons, a family embedded whole or not at all: a face that may be
 * embedded is left out when another face of its family used is, since an `@font-face` family
 * hides the installed family of that name (CSS Fonts 4, §5.2).
 *
 * @kind domain
 * @param faces - the faces used
 * @returns them with their reasons
 * @see REF-CSS-FONTS-4
 */
function judgedFaces(faces: readonly UsedFace[]): readonly JudgedFace[] {
  const judged = faces.map((face) => ({ ...face, reason: faceReason(face) }));

  return judged.map((face) =>
    face.reason === undefined && isIncomplete(judged, face.named.family)
      ? { ...face, reason: "family-incomplete" }
      : face,
  );
}

/**
 * The face embedded, when it has no reason to be left out.
 *
 * @kind domain
 * @param judged - a face used and its reason
 * @returns the face to write, alone; none otherwise
 * @see DERIV-font-embedding
 */
function embeddedFace(judged: JudgedFace): readonly EmbeddedFace[] {
  const { file, isItalic, weight } = judged.named.face;

  return file !== undefined && judged.reason === undefined
    ? [{ family: judged.named.family, file, isItalic, weight }]
    : [];
}

/**
 * The face left out and why.
 *
 * @kind domain
 * @param judged - a face used and its reason
 * @returns the face and its reason, alone; none for a face embedded
 * @see DERIV-font-embedding
 */
function leftOutFace(judged: JudgedFace): readonly LeftOutFace[] {
  const { isItalic, weight } = judged.named.face;
  const { reason } = judged;

  return reason === undefined ? [] : [{ family: judged.named.family, isItalic, reason, weight }];
}

/**
 * The faces a document embeds, its embedded fonts on (ADR-0053): the face each text is measured
 * with, each once whatever the number of its texts, in the order of its first use
 * (`DERIV-font-embedding`, step 7), when a text names its family, its family is not named like a
 * generic family, its file is held and its `fsType` permits it, and every other face of its
 * family used may be embedded too; the others left out with their reason, for the editor to say
 * so. Off, the document embeds nothing and leaves nothing out: 10 texts in Arial 400 and 700
 * embed 2 faces.
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
  const faces = judgedFaces(options.hasEmbeddedFonts ? usedFaces(texts, fonts) : []);

  return {
    embedded: faces.flatMap((face) => embeddedFace(face)),
    leftOut: faces.flatMap((face) => leftOutFace(face)),
  };
}

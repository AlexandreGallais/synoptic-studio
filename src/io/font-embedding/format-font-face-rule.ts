import { formatFontFamilyName } from "../text-style";

import { formatDataUrl } from "./format-data-url";
import { parseFontFormat } from "./parse-font-format";

import type { EmbeddedFace } from "../../model";

/**
 * The `@font-face` rule of an embedded face (ADR-0053; CSS Fonts 4, §4): its family's name as a
 * text's `font-family` writes it (`formatFontFamilyName`), its style, `italic` or `normal`, its
 * weight, and its whole file as a base-64 `data:` URL with its format hint; no `local()`, which
 * Batik refuses (`REF-BATIK-1159`): `@font-face { font-family: "Arial"; font-style: italic;
 * font-weight: 700; src: url(data:font/ttf;base64,…) format("truetype"); }`.
 *
 * @kind format
 * @param face - the face and its file
 * @returns the rule, on one line
 * @see REF-CSS-FONTS-4
 */
export function formatFontFaceRule(face: EmbeddedFace): string {
  const { hint, mediaType } = parseFontFormat(face.file.bytes);
  const declarations = [
    `font-family: ${formatFontFamilyName(face.family)}`,
    `font-style: ${face.isItalic ? "italic" : "normal"}`,
    `font-weight: ${String(face.weight)}`,
    `src: url(${formatDataUrl(mediaType, face.file.bytes)}) format("${hint}")`,
  ];

  return `@font-face { ${declarations.join("; ")}; }`;
}

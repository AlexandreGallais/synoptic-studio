// @vitest-environment happy-dom

import { describe, expect, it } from "vitest";

import { formatFontFaceRule } from "../io";

import { createEmbeddedFontsElements } from "./create-embedded-fonts-elements";
import { SVG_NAMESPACE } from "./svg-namespace";

import type { EmbeddedFace } from "../model";

/** Arial regular, from a TrueType file. */
const REGULAR: EmbeddedFace = {
  family: "Arial",
  file: { bytes: Uint8Array.from([0, 1, 0, 0]), isEmbeddable: true, version: "" },
  isItalic: false,
  weight: 400,
};

/** Arial bold, from another file. */
const BOLD: EmbeddedFace = {
  ...REGULAR,
  file: { ...REGULAR.file, bytes: Uint8Array.from([0, 1, 0, 0, 7]) },
  weight: 700,
};

describe("createEmbeddedFontsElements (ADR-0053)", () => {
  it("[F15.AC2] writes one <defs> holding one <style> with a rule per face, in order", () => {
    const elements = createEmbeddedFontsElements(document, [BOLD, REGULAR]);
    const [defs = document.createElementNS(SVG_NAMESPACE, "g")] = elements;

    expect(elements).toHaveLength(1);
    expect([defs.namespaceURI, defs.tagName]).toEqual([SVG_NAMESPACE, "defs"]);
    expect([...defs.children].map((child) => child.tagName)).toEqual(["style"]);

    expect(defs.querySelector("style")?.textContent).toBe(
      `${formatFontFaceRule(BOLD)}\n${formatFontFaceRule(REGULAR)}`,
    );

    // Two rules: bold first, its file of 5 bytes in 8 characters, then regular.
    expect(defs.textContent.match(/@font-face/gu)).toHaveLength(2);

    expect(defs.textContent).toContain("font-weight: 700; src: url(data:font/ttf;base64,AAEAAAc=)");
  });

  it("[F15.AC2] writes no <style> when no face is embedded, the option off among them", () => {
    expect(createEmbeddedFontsElements(document, [])).toEqual([]);
  });
});

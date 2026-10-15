// @vitest-environment happy-dom
import { describe, expect, it } from "vitest";

import { DECORATED_METRICS, HELLO_TEXT } from "../../tests";
import { DEFAULT_DOCUMENT_OPTIONS, textDrawing, textHtmlLayer } from "../model";

import { createDocumentTextElement } from "./create-document-text-element";
import { createTextElement } from "./create-text-element";

import type { Text } from "../model";

/** A text turned, decorated, with a background, in a box. */
const TEXT: Text = {
  ...HELLO_TEXT,
  angle: 30,
  boxHeight: 40,
  boxWidth: 50,
  content: "Hello\nWorld",
  hasBackground: true,
  isInBox: true,
  isUnderlined: true,
};

describe("createDocumentTextElement (ADR-0053)", () => {
  const drawn = textDrawing(DECORATED_METRICS, TEXT);
  const drawing = { drawn, layer: textHtmlLayer(DECORATED_METRICS, TEXT, drawn), text: TEXT };

  it("[F15.AC3] writes a text exactly as before with the HTML layer off, the default", () => {
    const written = createDocumentTextElement(document, DEFAULT_DOCUMENT_OPTIONS, drawing);

    expect(DEFAULT_DOCUMENT_OPTIONS.hasHtmlLayer).toBe(false);
    expect(written.outerHTML).toBe(createTextElement(document, TEXT, drawn).outerHTML);
    expect(written.querySelector("switch, foreignObject")).toBeNull();
  });

  it("[F15.AC3] writes the switch with the HTML layer on", () => {
    const options = { ...DEFAULT_DOCUMENT_OPTIONS, hasHtmlLayer: true };
    const written = createDocumentTextElement(document, options, drawing);

    expect(written.querySelectorAll(":scope > switch > foreignObject")).toHaveLength(1);
  });

  it.each([
    { boxHeight: 0, boxWidth: 80 },
    { boxHeight: 40, boxWidth: 0 },
  ])(
    "[F15.AC3] writes no layer for a box $boxWidth × $boxHeight, which SVG would not render",
    (box) => {
      const flat = { ...TEXT, ...box };
      const flatDrawn = textDrawing(DECORATED_METRICS, flat);
      const layer = textHtmlLayer(DECORATED_METRICS, flat, flatDrawn);
      const options = { ...DEFAULT_DOCUMENT_OPTIONS, hasHtmlLayer: true };
      const written = createDocumentTextElement(document, options, {
        drawn: flatDrawn,
        layer,
        text: flat,
      });

      expect(written.outerHTML).toBe(createTextElement(document, flat, flatDrawn).outerHTML);
    },
  );
});

// @vitest-environment happy-dom

import { describe, expect, it } from "vitest";

import { DECORATED_METRICS, HELLO_TEXT } from "../../tests";
import { textDrawing } from "../model";

import { createTextElement } from "./create-text-element";
import { SVG_NAMESPACE } from "./svg-namespace";

/** "Hello" over "World", centered, underlined and struck through, in the decorated test font 16. */
const TEXT = {
  ...HELLO_TEXT,
  alignment: "center",
  content: "Hello\nWorld",
  isStruckThrough: true,
  isUnderlined: true,
} as const;

describe("createTextElement (ADR-0052)", () => {
  const group = createTextElement(document, TEXT, textDrawing(DECORATED_METRICS, TEXT));
  const text = group.querySelector("text");

  it("[F05.AC4] groups one <text> and the decorations' paths in a <g>", () => {
    expect(group.namespaceURI).toBe(SVG_NAMESPACE);
    expect(group.tagName).toBe("g");

    expect([...group.children].map((child) => child.tagName)).toEqual([
      "text",
      "path",
      "path",
      "path",
      "path",
    ]);
  });

  it("[F05.AC4] writes one <tspan> per line, each at its x and its baseline", () => {
    const spans = [...(text?.querySelectorAll("tspan") ?? [])];

    // 'Hello' centered: (41.78125 − 36.4609375) / 2 = 2.66015625, written 2.66016; baselines
    // 14.484375 and 32.8828125, written 14.48438 and 32.88281.
    expect(
      spans.map((span) => [span.textContent, span.getAttribute("x"), span.getAttribute("y")]),
    ).toEqual([
      ["Hello", "2.66016", "14.48438"],
      ["World", "0", "32.88281"],
    ]);
  });

  it("[F05.AC4] keeps every space, without text-anchor nor dominant-baseline", () => {
    expect(text?.getAttributeNS("http://www.w3.org/XML/1998/namespace", "space")).toBe("preserve");
    expect(text?.getAttribute("font-family")).toBe('"Inter", sans-serif');
    expect(group.querySelector("[text-anchor], [dominant-baseline], [text-decoration]")).toBeNull();
  });

  it("[F05.AC4] draws the underline and the strike-through as rectangles in the text's color", () => {
    const [underline] = group.querySelectorAll("path");

    // 'Hello' underline: from (2.66015625, 16.484375), 36.4609375 long, 1 thick.
    expect(underline?.getAttribute("d")).toBe(
      "M2.66016 16.48438 L39.12109 16.48438 L39.12109 17.48438 L2.66016 17.48438 Z",
    );

    expect([underline?.getAttribute("fill"), underline?.getAttribute("fill-opacity")]).toEqual([
      "#000000",
      "1",
    ]);
  });

  it("[F05.AC4] draws the strike-through 0.5 thick, 4 above the baseline", () => {
    const strike = group.querySelectorAll("path")[1];

    // Hello strike-out: top 14.484375 − 4 = 10.484375, 64 × 16 / 2048 = 0.5 thick.
    expect(strike?.getAttribute("d")).toBe(
      "M2.66016 10.48438 L39.12109 10.48438 L39.12109 10.98438 L2.66016 10.98438 Z",
    );
  });

  it.each([
    { opacity: 0, written: "0" },
    { opacity: 40, written: "0.4" },
  ])(
    "[F05.AC4] writes an opacity of $opacity % as $written on the text and its paths",
    ({ opacity, written }) => {
      const faded = { ...TEXT, opacity };
      const drawn = createTextElement(document, faded, textDrawing(DECORATED_METRICS, faded));

      expect(
        [...drawn.querySelectorAll("text, path")].map((element) =>
          element.getAttribute("fill-opacity"),
        ),
      ).toEqual(Array.from({ length: 5 }, () => written));
    },
  );

  it("draws an empty text as an empty <text>", () => {
    const empty = { ...HELLO_TEXT, content: "" };
    const drawn = createTextElement(document, empty, textDrawing(DECORATED_METRICS, empty));

    expect(drawn.querySelectorAll("tspan, path")).toHaveLength(0);
    expect(drawn.textContent).toBe("");
  });
});

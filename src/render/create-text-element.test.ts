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
    // white-space for SVG 2 readers, XML:space for SVG 1.1 ones (ADR-0054).
    expect(text?.getAttribute("white-space")).toBe("pre");
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

  it("[F16.AC4] draws an underline chosen #ff0000 at 50 % in it, the strike-through in the text's #000000 at 1", () => {
    const chosen = {
      ...TEXT,
      hasUnderlineColor: true,
      underlineColor: "#ff0000",
      underlineOpacity: 50,
    };
    const drawn = createTextElement(document, chosen, textDrawing(DECORATED_METRICS, chosen));

    // Per line, its underline then its strike-through: 'Hello' then 'World'.
    expect(
      [...drawn.querySelectorAll("path")].map((path) => [
        path.getAttribute("fill"),
        path.getAttribute("fill-opacity"),
      ]),
    ).toEqual([
      ["#ff0000", "0.5"],
      ["#000000", "1"],
      ["#ff0000", "0.5"],
      ["#000000", "1"],
    ]);
  });

  it("[F16.AC4] draws both strokes in the text's #0000ff when neither color was chosen", () => {
    const blue = { ...TEXT, color: "#0000ff", strikeThroughColor: "#ff0000" };
    const drawn = createTextElement(document, blue, textDrawing(DECORATED_METRICS, blue));

    expect([...drawn.querySelectorAll("path")].map((path) => path.getAttribute("fill"))).toEqual(
      Array.from({ length: 4 }, () => "#0000ff"),
    );
  });

  it("[F05.AC5] turns the group by 90 about the middle of the measured block", () => {
    const turned = { ...TEXT, angle: 90, x: 10, y: 20 };
    const drawn = createTextElement(document, turned, textDrawing(DECORATED_METRICS, turned));

    // Block 41.78125 × 36.796875 at (10, 20): middle (30.890625, 38.3984375), at 5 decimals. The
    // group turns, so the underline and the strike-through turn with the letters.
    expect(drawn.getAttribute("transform")).toBe("rotate(90 30.89063 38.39844)");
    expect(drawn.querySelector("[transform]")).toBeNull();
  });

  it("[F05.AC5] writes no transform for a text that does not turn", () => {
    expect(group.getAttribute("transform")).toBeNull();
  });

  describe("[F14.AC4] with a background", () => {
    const highlighted = {
      ...HELLO_TEXT,
      angle: 30,
      backgroundColor: "#22c55e",
      backgroundOpacity: 40,
      content: "Hello\nWorld",
      hasBackground: true,
    };
    const drawn = createTextElement(
      document,
      highlighted,
      textDrawing(DECORATED_METRICS, highlighted),
    );

    it("[F14.AC4] draws one <path> before the <text>, so that the letters stay on top", () => {
      expect([...drawn.children].map((child) => child.tagName)).toEqual(["path", "text"]);
    });

    it("[F14.AC4] holds one rectangle per line, clockwise from its top-left (Q11)", () => {
      // (0, 0) 36.4609375 × 17.875 and (0, 18.3984375) 41.78125 × 17.875: bottom
      // 18.3984375 + 17.875 = 36.2734375, at 5 decimals.
      expect(drawn.querySelector("path")?.getAttribute("d")).toBe(
        "M0 0 L36.46094 0 L36.46094 17.875 L0 17.875 Z M0 18.39844 L41.78125 18.39844 L41.78125 36.27344 L0 36.27344 Z",
      );
    });

    it("[F14.AC4] fills it with the background's color, its opacity 40 % written 0.4", () => {
      const path = drawn.querySelector("path");

      expect([path?.getAttribute("fill"), path?.getAttribute("fill-opacity")]).toEqual([
        "#22c55e",
        "0.4",
      ]);
    });

    it("[F14.AC4] turns it with the group, carrying no transform of its own", () => {
      expect(drawn.getAttribute("transform")).not.toBeNull();
      expect(drawn.querySelector("path")?.getAttribute("transform")).toBeNull();
    });

    it("[F14.AC4] draws one <path> for a single line", () => {
      const one = { ...highlighted, content: "Hello" };
      const oneGroup = createTextElement(document, one, textDrawing(DECORATED_METRICS, one));

      expect(oneGroup.querySelectorAll("path")).toHaveLength(1);
    });

    it("[F14.AC4] draws no <path> when no line has a length", () => {
      const blank = { ...highlighted, content: "\n" };
      const blankGroup = createTextElement(document, blank, textDrawing(DECORATED_METRICS, blank));

      expect(blankGroup.querySelectorAll("path")).toHaveLength(0);
    });
  });

  it("draws an empty text as an empty <text>", () => {
    const empty = { ...HELLO_TEXT, content: "" };
    const drawn = createTextElement(document, empty, textDrawing(DECORATED_METRICS, empty));

    expect(drawn.querySelectorAll("tspan, path")).toHaveLength(0);
    expect(drawn.textContent).toBe("");
  });
});

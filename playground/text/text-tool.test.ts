// @vitest-environment happy-dom

import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  clickWithKeys,
  openServedPage,
  readDisplay,
  readPressedState,
  readText,
  readValue,
  selectPageElement,
  typeValue,
} from "../user-actions";

/** Namespace of the `xml:` attributes. */
const XML_NAMESPACE = "http://www.w3.org/XML/1998/namespace";

/**
 * Types a content in the Text tool's text area, as the designer would.
 *
 * @param content - the lines, a line feed between two
 */
function typeContent(content: string): void {
  const area = selectPageElement("#text-content");

  if (area instanceof HTMLTextAreaElement) {
    area.value = content;
  }

  area.dispatchEvent(new Event("input", { bubbles: true }));
}

/**
 * Reads the text drawn on the canvas.
 *
 * @returns its `<text>` element
 */
function readTextElement(): Element {
  return selectPageElement("#canvas text");
}

/**
 * Reads each line drawn: its characters, `x` and `y`.
 *
 * @returns one row per `<tspan>`
 */
function readLines(): readonly (readonly (string | null)[])[] {
  return [...document.querySelectorAll("#canvas tspan")].map((span) => [
    span.textContent,
    span.getAttribute("x"),
    span.getAttribute("y"),
  ]);
}

describe("Text tool", () => {
  beforeEach(() => {
    openServedPage("");
    clickElement("tool-text");
    // "H" over "HH" in Inter Regular 16: H advances 1522 of 2048 units, 11.890625; HH 23.78125.
    typeValue("text-size", "16");
    // happy-dom does not select the option marked `selected` once the page is moved into its
    // document: the weight the page opens with, 400, is chosen again.
    chooseOption("text-weight", "400");
    typeContent("H\nHH");
  });

  it("[F05.AC2] opens its fields, the shapes' size and corners hidden", () => {
    expect(readPressedState("tool-text")).toBe("true");
    expect(readPressedState("tool-rectangle")).toBe("false");
    expect(readText("#shape-title")).toBe("Text");
    expect(readDisplay("text-options") === "none").toBe(false);
    expect(readDisplay("size-section") === "none").toBe(true);
    expect(readDisplay("corners-section") === "none").toBe(true);
  });

  it("[F05.AC4] draws one <text> with one positioned <tspan> per line, no anchor nor baseline", () => {
    // Baselines: ascender 1984 × 16 / 2048 = 15.5, then 15.5 + 2478 × 16 / 2048 = 34.859375.
    expect(readLines()).toEqual([
      ["H", "0", "15.5"],
      ["HH", "0", "34.85938"],
    ]);

    expect(document.querySelectorAll("#canvas text")).toHaveLength(1);
    expect(document.querySelector("#canvas [text-anchor], #canvas [dominant-baseline]")).toBeNull();
    expect(readTextElement().getAttributeNS(XML_NAMESPACE, "space")).toBe("preserve");
    expect(readTextElement().getAttribute("font-family")).toBe('"Inter", sans-serif');
  });

  it("[F05.AC4] aligns each line in the block: right, then center", () => {
    chooseOption("text-alignment", "right");
    // Room 23.78125 − 11.890625 = 11.890625 before "H".
    expect(readLines().map(([, x]) => x)).toEqual(["11.89063", "0"]);
    chooseOption("text-alignment", "center");
    expect(readLines().map(([, x]) => x)).toEqual(["5.94531", "0"]);
  });

  it("[F05.AC4] writes a right-to-left line at its right edge", () => {
    chooseOption("text-direction", "rtl");
    expect(readTextElement().getAttribute("direction")).toBe("rtl");
    expect(readLines().map(([, x]) => x)).toEqual(["11.89063", "23.78125"]);
  });

  it("[F05.AC5] frames the measured block, 23.78125 × 38.71875, dashed as a shape's box", () => {
    // Two lines of 2478 × 16 / 2048 = 19.359375. Dashes 4 long every 7, 1 thick: the top side
    // from x 0 to 23.78125 holds ceil(23.78125 / 7) = 4, the last one cut at 23.78125.
    const dashes = selectPageElement("#canvas g.text-frame path.straight-box").getAttribute("d");

    expect(dashes).toMatch(/^M0 0H4V1H0Z M7 0H11V1H7Z M14 0H18V1H14Z M21 0H23\.78125V1H21Z /u);
    expect(dashes).toContain("M22.78125 35H23.78125V38.71875H22.78125Z");
    expect(readText("#contour")).toContain('"width": 23.78125');
  });

  it("[F05.AC2] hides every control of the shapes, its points and handles too, and shows them back", () => {
    for (const id of ["size-section", "corners-section", "straight-box", "tools"]) {
      expect([id, readDisplay(id)]).toEqual([id, "none"]);
    }

    clickElement("tool-rectangle");

    for (const id of ["size-section", "corners-section", "turn", "tools"]) {
      expect(readDisplay(id) === "none").toBe(false);
    }

    // A vertex picked in Points mode: its panels go with the Text tool.
    clickWithKeys("#mode-points", {});
    clickWithKeys('#points [data-vertex="1"]', {});
    clickElement("tool-text");

    for (const id of ["points", "point-fields", "handle-fields"]) {
      expect([id, readDisplay(id)]).toEqual([id, "none"]);
    }
  });

  it("[F05.AC2] offers the weights 100 to 900; bold sets 700 and back to 400", () => {
    const weights = [...document.querySelectorAll<HTMLOptionElement>("#text-weight option")];

    expect(weights.map((option) => option.value)).toEqual(
      ["100", "200", "300", "400", "500", "600", "700", "800", "900"].map(String),
    );

    clickElement("text-bold");
    expect([readValue("text-weight"), readPressedState("text-bold")]).toEqual(["700", "true"]);
    expect(readTextElement().getAttribute("font-weight")).toBe("700");
    clickElement("text-bold");
    expect([readValue("text-weight"), readPressedState("text-bold")]).toEqual(["400", "false"]);
    chooseOption("text-weight", "900");
    expect(readTextElement().getAttribute("font-weight")).toBe("900");
  });

  it.each([
    { after: "400", pressed: "true", weight: "800" },
    { after: "400", pressed: "true", weight: "900" },
    { after: "700", pressed: "false", weight: "600" },
  ])(
    "[F05.AC2] presses Bold at weight $weight: $pressed, and a click sets $after",
    ({ after, pressed, weight }) => {
      chooseOption("text-weight", weight);
      expect(readPressedState("text-bold")).toBe(pressed);
      clickElement("text-bold");
      expect(readValue("text-weight")).toBe(after);
    },
  );

  it("[F05.AC2] turns italic on and off", () => {
    clickElement("text-italic");
    expect(readTextElement().getAttribute("font-style")).toBe("italic");
    clickElement("text-italic");
    expect(readTextElement().getAttribute("font-style")).toBe("normal");
  });

  it("[F05.AC2] writes the color, the opacity and the spacing typed", () => {
    typeValue("text-color", "#FF0000");
    typeValue("text-opacity", "40");
    typeValue("text-letter-spacing", "2");
    typeValue("text-word-spacing", "-4");

    expect(
      ["fill", "fill-opacity", "letter-spacing", "word-spacing"].map((name) =>
        readTextElement().getAttribute(name),
      ),
    ).toEqual(["#FF0000", "0.4", "2", "-4"]);
  });

  it("[F05.AC4] underlines and strikes each line through with paths in the text's color", () => {
    clickElement("text-underline");
    clickElement("text-strike");

    const paths = document.querySelectorAll("#canvas g:has(> text) path");

    // Two lines, each underlined and struck through; Inter Regular's underline top −348 units:
    // 15.5 + 348 × 16 / 2048 = 18.21875, 140 × 16 / 2048 = 1.09375 thick.
    expect(paths).toHaveLength(4);

    expect(paths[0]?.getAttribute("d")).toBe(
      "M0 18.21875 L11.89063 18.21875 L11.89063 19.3125 L0 19.3125 Z",
    );

    expect(paths[0]?.getAttribute("fill")).toBe("#1e293b");
    expect(document.querySelector("#canvas [text-decoration]")).toBeNull();
  });

  it.each([
    {
      // 15.5 + 348 × 16 / 2048 = 18.21875, 140 × 16 / 2048 = 1.09375 thick.
      d: "M0 18.21875 L11.89063 18.21875 L11.89063 19.3125 L0 19.3125 Z",
      id: "text-underline",
    },
    {
      // 15.5 − 671 × 16 / 2048 = 10.2578125, 1.09375 thick.
      d: "M0 10.25781 L11.89063 10.25781 L11.89063 11.35156 L0 11.35156 Z",
      id: "text-strike",
    },
  ])("[F05.AC4] draws $id alone along each line", ({ d, id }) => {
    clickElement(id);

    const paths = document.querySelectorAll("#canvas g:has(> text) path");

    expect(paths).toHaveLength(2);
    expect(paths[0]?.getAttribute("d")).toBe(d);
  });

  it("[F05.AC2] names a font without metrics first, measured with Inter", () => {
    typeValue("text-font", "Arial");
    expect(readTextElement().getAttribute("font-family")).toBe('"Arial", sans-serif');

    expect(readLines()).toEqual([
      ["H", "0", "15.5"],
      ["HH", "0", "34.85938"],
    ]);
  });

  it("[F05.AC4] writes the lines in the case chosen", () => {
    typeContent("pump p-101 is running");
    chooseOption("text-case", "constant");
    expect(readLines().map(([content]) => content)).toEqual(["PUMP_P_101_IS_RUNNING"]);
  });

  it("moves the text with X and Y", () => {
    typeValue("shape-x", "10");
    typeValue("shape-y", "20");
    // 20 + 15.5.
    expect(readLines()[0]).toEqual(["H", "10", "35.5"]);
  });

  it("refuses a size of 0 and draws nothing, saying why, the last valid text kept", () => {
    typeValue("text-size", "0");
    expect(readText("#status")).toContain("Size must be an integer from 1 to 100 000");
    expect(document.querySelectorAll("#canvas text")).toHaveLength(0);
    expect(selectPageElement("#text-size").getAttribute("aria-invalid")).toBe("true");
    expect(selectPageElement("#shape-x").getAttribute("aria-invalid")).toBe("true");
    expect(readText("#model")).toContain('"size": 16');
  });

  it("draws the text again once the size is valid, its fields no longer outlined", () => {
    typeValue("text-size", "0");
    typeValue("text-size", "16");
    expect(readText("#status")).toBe("");
    expect(selectPageElement("#text-size").getAttribute("aria-invalid")).toBe("false");
    expect(readLines()).toHaveLength(2);
  });

  it("draws an empty content as an empty text, without a frame", () => {
    typeContent("");
    expect(readLines()).toEqual([]);
    expect(document.querySelectorAll("#canvas path")).toHaveLength(0);
  });

  it("[F05.AC6] goes back to the shapes, the text's fields hidden", () => {
    clickElement("tool-rectangle");
    expect(readDisplay("text-options") === "none").toBe(true);
    expect(readDisplay("size-section") === "none").toBe(false);
    expect(readPressedState("tool-rectangle")).toBe("true");
    expect(document.querySelectorAll("#canvas text")).toHaveLength(0);
    expect(readText("#path-data")).toMatch(/^M0 0 L/u);
  });
});

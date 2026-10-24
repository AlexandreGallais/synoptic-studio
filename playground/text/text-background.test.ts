// @vitest-environment happy-dom

import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  chooseShapeKind,
  clickElement,
  openServedPage,
  readText,
  selectPageElement,
  typeAndEnter,
  typeValue,
} from "../user-actions";

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
 * Reads the elements of the text's group, in drawing order.
 *
 * @returns their tag names
 */
function readGroupChildren(): readonly string[] {
  const group = selectPageElement("#canvas text").parentElement;

  return [...(group?.children ?? [])].map((child) => child.tagName);
}

/**
 * Reads the background's `<path>`: the one drawn before the `<text>` in its group.
 *
 * @returns its path data, fill and fill opacity, undefined without a path
 */
function readBackground(): readonly (string | null | undefined)[] {
  const path = selectPageElement("#canvas text").parentElement?.querySelector("path");

  return ["d", "fill", "fill-opacity"].map((name) => path?.getAttribute(name));
}

describe("Text tool: a background behind the text (US-034)", () => {
  beforeEach(() => {
    openServedPage("");
    chooseShapeKind("text");
    typeValue("text-size", "16");
    // happy-dom loses the option marked `selected` once the page is moved into its document: the
    // choices the page opens with are chosen again.
    chooseOption("text-weight", "400");
    chooseOption("text-fit", "content");
    chooseOption("text-vertical-alignment", "top");
    typeContent("Pump P-101\nRunning");
  });

  it("[F14.AC6] draws no background until Background is ticked", () => {
    expect(readGroupChildren()).toEqual(["text"]);
    expect(readText("#model")).toContain('"hasBackground": false');
  });

  it("[F14.AC4] draws one band a line, as long as the line, yellow and opaque by default", () => {
    clickElement("text-background");

    // Inter 16: ascent 1984 × 16 / 2048 = 15.5 above each baseline (15.5, 34.859375), height
    // (1984 + 494) × 16 / 2048 = 19.359375; widths as `lineWidth` measures them, kerning included.
    expect(readGroupChildren()).toEqual(["path", "text"]);

    expect(readBackground()).toEqual([
      "M0 0 L87.95313 0 L87.95313 19.35938 L0 19.35938 Z M0 19.35938 L61.80469 19.35938 L61.80469 38.71875 L0 38.71875 Z",
      "#ffff00",
      "1",
    ]);
  });

  it("[F14.AC4] fills the bands with the background's color and opacity", () => {
    clickElement("text-background");
    typeValue("text-background-color", "#22c55e");
    typeValue("text-background-opacity", "40");

    expect(readBackground().slice(1)).toEqual(["#22c55e", "0.4"]);
    expect(readText("#model")).toContain('"backgroundOpacity": 40');
  });

  it("[F16.AC1] reads a background color typed in upper case in lower case", () => {
    clickElement("text-background");
    typeValue("text-background-color", "#FF8000");

    expect(readBackground()[1]).toBe("#ff8000");
    expect(readText("#model")).toContain('"backgroundColor": "#ff8000"');
  });

  it("[F14.AC4] follows the wrapped lines in a box, the last one up to its ellipsis", () => {
    clickElement("text-background");
    chooseOption("text-fit", "box");
    typeAndEnter("text-box-width", "50");
    typeAndEnter("text-box-height", "100");
    typeValue("text-maximum-lines", "2");

    // "Pump" (43.49219) over "P-10…" (46.875), cut at 2 lines in a box 50 wide.
    expect([...document.querySelectorAll("#canvas tspan")].map((span) => span.textContent)).toEqual(
      ["Pump", "P-10…"],
    );

    expect(readBackground()[0]).toBe(
      "M0 0 L43.49219 0 L43.49219 19.35938 L0 19.35938 Z M0 19.35938 L46.875 19.35938 L46.875 38.71875 L0 38.71875 Z",
    );
  });

  it("[F14.AC4] turns the bands with the text: the group turns, not the path", () => {
    clickElement("text-background");
    typeValue("angle", "30");

    const group = selectPageElement("#canvas text").parentElement;

    expect(group?.getAttribute("transform")).toMatch(/^rotate\(30 /u);
    expect(group?.querySelector("path")?.getAttribute("transform")).toBeNull();
  });

  it("[F14.AC4] gives an empty line no band", () => {
    clickElement("text-background");
    typeContent("Pump P-101\n\nRunning");

    expect(readBackground()[0]?.match(/M/gu)).toHaveLength(2);
  });

  it.each([
    { id: "text-background-color", value: "#ff800" },
    { id: "text-background-opacity", value: "101" },
    { id: "text-background-opacity", value: "" },
  ])("refuses $id = '$value' and draws nothing", ({ id, value }) => {
    clickElement("text-background");
    typeValue(id, value);

    expect(readText("#status")).toContain("background opacity from 0 to 100");
    expect(document.querySelectorAll("#canvas text")).toHaveLength(0);
    expect(selectPageElement(`#${id}`).getAttribute("aria-invalid")).toBe("true");
  });

  it("[F14.AC6] takes the bands away when Background is unticked", () => {
    clickElement("text-background");
    clickElement("text-background");

    expect(readGroupChildren()).toEqual(["text"]);
  });
});

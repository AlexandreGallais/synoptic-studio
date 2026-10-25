// @vitest-environment happy-dom

import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  chooseShapeKind,
  clickElement,
  commitField,
  openPage,
  readDisplay,
  readText,
  readValue,
  selectPageElement,
  typeAndEnter,
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
 * Reads the `transform` of the text's group and of its block's frame.
 *
 * @returns the two values, the text's first
 */
function readTransforms(): readonly (string | null | undefined)[] {
  return [
    document.querySelector("#canvas g:has(> text)")?.getAttribute("transform"),
    document.querySelector("#canvas g.text-frame")?.getAttribute("transform"),
  ];
}

describe("Text tool: turning", () => {
  beforeEach(() => {
    openPage("");
    chooseShapeKind("text");
    // "H" over "HH" in Inter Regular 16: block 23.78125 × 2 × 19.359375 = 38.71875, its middle
    // (11.890625, 19.359375), written 11.89063 and 19.35938.
    typeAndEnter("text-size", "16");
    chooseOption("text-weight", "400");
    typeContent("H\nHH");
  });

  it("[F05.AC5] shows the angle row in Text mode, the shapes' modes still hidden", () => {
    expect(readDisplay("turn") === "none").toBe(false);
    expect(readDisplay("tools")).toBe("none");
    expect(readDisplay("straight-box")).toBe("none");
  });

  it("[F05.AC5] turns the text and its block's frame by 90 about the block's middle", () => {
    typeAndEnter("angle", "90");

    expect(readTransforms()).toEqual([
      "rotate(90 11.89063 19.35938)",
      "rotate(90 11.89063 19.35938)",
    ]);

    expect(readText("#model")).toContain('"angle": 90');
  });

  it("[F05.AC5] keeps the middle with the position: X 10, Y 20 at 45", () => {
    typeAndEnter("shape-x", "10");
    typeAndEnter("shape-y", "20");
    typeAndEnter("angle", "45");

    // (10 + 11.890625, 20 + 19.359375).
    expect(readTransforms()[0]).toBe("rotate(45 21.89063 39.35938)");
  });

  it.each([
    { kept: "10", typed: "370" },
    { kept: "270", typed: "-90" },
    { kept: "0", typed: "360" },
  ])("[F05.AC5] keeps an angle typed $typed as $kept", ({ kept, typed }) => {
    typeAndEnter("angle", typed);
    commitField("angle");

    expect(readValue("angle")).toBe(kept);
    expect(readText("#model")).toContain(`"angle": ${kept}`);
  });

  it("[F05.AC5] turns by a quarter turn with the +90° and −90° buttons", () => {
    clickElement("turn-right");
    expect(readTransforms()[0]).toBe("rotate(90 11.89063 19.35938)");

    clickElement("turn-left");
    clickElement("turn-left");
    expect(readTransforms()[0]).toBe("rotate(270 11.89063 19.35938)");
  });

  it("[F05.AC5] writes no transform on the text at 0, its frame turned by 0", () => {
    const [textTransform, frameTransform] = readTransforms();

    expect(textTransform).toBeNull();
    expect(frameTransform).toBe("rotate(0 11.89063 19.35938)");
  });

  it("refuses an angle emptied by a script and draws nothing, saying why", () => {
    typeAndEnter("angle", "");

    expect(readText("#status")).toContain("the angle integers");
    expect(document.querySelectorAll("#canvas text")).toHaveLength(0);
    expect(selectPageElement("#angle").getAttribute("aria-invalid")).toBe("true");
  });
});

// @vitest-environment happy-dom

import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  chooseShapeKind,
  clickElement,
  commitField,
  openPage,
  pressPointer,
  readDisplay,
  readText,
  readValue,
  selectPageElement,
  setElementBox,
  typeAndEnter,
  typeInField,
  typeValue,
} from "../../../../../testing";

/**
 * Types a value into a field it gives the focus to, then leaves it, as the designer pressing Tab.
 *
 * @param id - the field
 * @param value - what is typed or pasted
 */
function typeAndLeave(id: string, value: string): void {
  const field = selectPageElement(`#${id}`);

  field.focus();
  field.dispatchEvent(new FocusEvent("focus"));
  typeValue(id, value);
  commitField(id);
  field.blur();
}

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
 * Reads the fill and the fill opacity of the text's strokes, line by line, its underline then
 * its strike-through.
 *
 * @returns each stroke's fill and fill opacity, joined by a space
 */
function readStrokeFills(): readonly string[] {
  const group = selectPageElement("#canvas text").parentElement;

  return [...(group?.querySelectorAll("path") ?? [])].map(
    (path) => `${path.getAttribute("fill") ?? ""} ${path.getAttribute("fill-opacity") ?? ""}`,
  );
}

/**
 * Reads whether a "follows the text's color" box is ticked.
 *
 * @param id - the box
 * @returns whether it is ticked
 */
function isFollowing(id: string): boolean {
  const box = selectPageElement(`#${id}`);

  return box instanceof HTMLInputElement && box.checked;
}

describe("The underline's and the strike-through's colors", () => {
  beforeEach(() => {
    openPage("");
    chooseShapeKind("text");
    chooseOption("text-weight", "400");
    chooseOption("text-fit", "content");
    typeContent("P-101");
    clickElement("text-underline");
    clickElement("text-strike");
  });

  it("[F16.AC4] draws both strokes in the text's color, as before, their fields showing it", () => {
    expect(readStrokeFills()).toEqual(["#1e293b 1", "#1e293b 1"]);

    expect([isFollowing("text-underline-follows"), isFollowing("text-strike-follows")]).toEqual([
      true,
      true,
    ]);

    expect([readValue("text-underline-color"), readValue("text-strike-opacity")]).toEqual([
      "#1e293b",
      "100",
    ]);

    expect(selectPageElement("#text-strike-color-swatch").style.backgroundColor).toBe("#1e293b");
  });

  it("[F16.AC4] draws the strike-through in #ff0000 typed, the underline in the text's color", () => {
    typeAndLeave("text-strike-color", "#ff0000");

    expect(isFollowing("text-strike-follows")).toBe(false);
    expect(isFollowing("text-underline-follows")).toBe(true);
    expect(readStrokeFills()).toEqual(["#1e293b 1", "#ff0000 1"]);
  });

  it("[F16.AC4] draws the underline at 50 % once its opacity is typed, chosen from then on", () => {
    typeAndLeave("text-underline-opacity", "50");

    expect(isFollowing("text-underline-follows")).toBe(false);
    expect(readStrokeFills()).toEqual(["#1e293b 0.5", "#1e293b 1"]);
  });

  it("[F16.AC4] keeps the chosen colors when the text turns green, the following one turning with it", () => {
    typeAndLeave("text-strike-color", "#ff0000");
    typeAndLeave("text-color", "#22c55e");

    expect(selectPageElement("#canvas text").getAttribute("fill")).toBe("#22c55e");
    expect(readStrokeFills()).toEqual(["#22c55e 1", "#ff0000 1"]);
    expect(readValue("text-underline-color")).toBe("#22c55e");
  });

  it("[F16.AC4] follows the text's opacity 40, shown in the strokes' opacity fields", () => {
    typeAndEnter("text-opacity", "40");

    expect(readStrokeFills()).toEqual(["#1e293b 0.4", "#1e293b 0.4"]);

    expect([readValue("text-underline-opacity"), readValue("text-strike-opacity")]).toEqual([
      "40",
      "40",
    ]);
  });

  it("[F16.AC4] follows the text's color again once its box is ticked again", () => {
    typeAndLeave("text-underline-color", "#0000ff");
    clickElement("text-underline-follows");
    typeAndLeave("text-color", "#22c55e");

    expect(readStrokeFills()).toEqual(["#22c55e 1", "#22c55e 1"]);
    expect(readValue("text-underline-color")).toBe("#22c55e");
  });

  it("[F16.AC1] keeps F00 typed as the underline's color as #ff0000", () => {
    typeAndLeave("text-underline-color", "F00");

    expect(readValue("text-underline-color")).toBe("#ff0000");
    expect(readStrokeFills()).toEqual(["#ff0000 1", "#1e293b 1"]);
  });

  it("[F16.AC1] refuses red: the field shows its previous color, the line under it says why", () => {
    typeAndLeave("text-underline-color", "#ff0000");
    typeAndLeave("text-underline-color", "red");

    expect(readValue("text-underline-color")).toBe("#ff0000");
    expect(readText("#text-underline-color-message")).toContain("“red” is no hexadecimal color");
    expect(readStrokeFills()).toEqual(["#ff0000 1", "#1e293b 1"]);
  });

  it("[F16.AC1] takes text-underline-opacity = '101' at 100, the color then chosen", () => {
    typeInField("text-underline-opacity", "101");

    expect(isFollowing("text-underline-follows")).toBe(false);
    expect([readValue("text-underline-opacity"), readText("#status")]).toEqual(["100", ""]);
  });

  it.each([
    { id: "text-underline-opacity", value: "abc" },
    { id: "text-strike-opacity", value: "" },
  ])(
    "[F16.AC1] refuses $id = '$value', 100 shown again, the stroke still following",
    ({ id, value }) => {
      typeInField(id, value);

      expect(isFollowing(id.replace("opacity", "follows"))).toBe(true);
      expect([readValue(id), readText("#status")]).toEqual(["100", ""]);
      expect(document.querySelectorAll("#canvas text")).toHaveLength(1);
    },
  );

  it("[F16.AC1] leaves the box ticked for red refused, the underline following the text's #22c55e", () => {
    typeAndLeave("text-underline-color", "red");
    typeAndLeave("text-color", "#22c55e");

    expect([isFollowing("text-underline-follows"), ...readStrokeFills()]).toEqual([
      true,
      "#22c55e 1",
      "#22c55e 1",
    ]);
  });

  it("[F16.AC1] leaves the box ticked while a color is typed, until it is committed", () => {
    selectPageElement("#text-underline-color").focus();
    typeValue("text-underline-color", "#0000f");

    expect([isFollowing("text-underline-follows"), readValue("text-underline-color")]).toEqual([
      true,
      "#0000f",
    ]);

    selectPageElement("#text-strike-opacity").focus();
    typeValue("text-strike-opacity", "50");
    expect(isFollowing("text-strike-follows")).toBe(true);
  });

  it("[F16.AC2] opens the same picker from the underline's swatch, under its fields", () => {
    clickElement("text-underline-color-swatch");

    expect(readDisplay("color-picker")).not.toBe("none");

    expect(selectPageElement("#text-underline-color-message").nextElementSibling?.id).toBe(
      "color-picker",
    );

    expect(isFollowing("text-underline-follows")).toBe(true);
  });

  it("[F16.AC2] makes the underline blue at 50 % picked: hue 240 typed, the opacity bar's middle", () => {
    chooseOption("color-model", "hsb");
    clickElement("text-underline-color-swatch");
    setElementBox("#color-opacity", { height: 10, left: 0, top: 0, width: 200 });
    typeAndLeave("color-hue-value", "240");
    typeAndLeave("color-saturation-value", "100");
    typeAndLeave("color-level-value", "100");
    // 100 · 100 / 200 = 50 %.
    pressPointer("#color-opacity", "pointerdown", { buttons: 1, clientX: 100, clientY: 5 });

    expect(isFollowing("text-underline-follows")).toBe(false);
    expect(readStrokeFills()).toEqual(["#0000ff 0.5", "#1e293b 1"]);
  });

  it("[F16.AC2] marks the underline chosen by the opacity bar alone", () => {
    clickElement("text-underline-color-swatch");
    setElementBox("#color-opacity", { height: 10, left: 0, top: 0, width: 200 });
    pressPointer("#color-opacity", "pointerdown", { buttons: 1, clientX: 100, clientY: 5 });

    expect(isFollowing("text-underline-follows")).toBe(false);
    expect(readStrokeFills()).toEqual(["#1e293b 0.5", "#1e293b 1"]);
  });
});

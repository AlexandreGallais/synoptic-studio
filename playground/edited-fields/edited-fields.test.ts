// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { isDrawnText } from "../../src";
import { readKeptDrawing } from "../kept-shapes";
import { NUMBER_FIELDS } from "../number-fields";
import {
  chooseOption,
  clickAt,
  clickElement,
  commitField,
  openOnStep,
  openServedPage,
  readDisplay,
  readModel,
  readShapeModel,
  readText,
  readValue,
  selectPageElement,
  setElementBox,
  typeAndEnter,
  typeTextContent,
  typeValue,
} from "../user-actions";

import type { Text } from "../../src";

/**
 * Presses a key in a field of the panel, then releases it, as the browser sends them: bubbling,
 * cancelable.
 *
 * @param id - the field, without `#`
 * @param init - the key, as `KeyboardEvent.key` names it, and the modifiers held
 * @returns the key pressed, telling whether its default was prevented
 */
function pressIn(id: string, init: KeyboardEventInit): KeyboardEvent {
  const field = selectPageElement(`#${id}`);
  const down = new KeyboardEvent("keydown", { ...init, bubbles: true, cancelable: true });

  field.dispatchEvent(down);
  field.dispatchEvent(new KeyboardEvent("keyup", { ...init, bubbles: true, cancelable: true }));

  return down;
}

/**
 * Reads the text the panel edits.
 *
 * @returns the text, as the page shows its model
 * @throws {TypeError} when the panel edits a shape
 */
function readTextModel(): Text {
  const model = readModel();

  if (!isDrawnText(model)) {
    throw new TypeError("The panel edits a shape, not a text.");
  }

  return model;
}

describe("one shape's Width and Height, Up and Down (US-085)", () => {
  beforeEach(() => {
    // The page as it opens: the rectangle 120 × 80 at (0, 0), selected alone.
    openServedPage("");
  });

  it.each([
    { expected: 121, init: { key: "ArrowUp" }, name: "Up" },
    { expected: 130, init: { key: "ArrowUp", shiftKey: true }, name: "Shift + Up" },
    { expected: 110, init: { key: "ArrowDown", shiftKey: true }, name: "Shift + Down" },
    { expected: 119, init: { key: "ArrowDown" }, name: "Down" },
  ])("[F44.AC2] makes the rectangle $expected wide for $name in Width", ({ expected, init }) => {
    // 120 + 1, 120 + 10, 120 − 10 and 120 − 1: the browser's step of 1, or 10 with Shift.
    const down = pressIn("width", init);

    expect(readShapeModel().width).toBe(expected);
    expect(readValue("width")).toBe(String(expected));
    expect(down.defaultPrevented).toBe(true);
  });

  it("[F44.AC2] makes it 80 + 10 high for Shift + Up in Height", () => {
    pressIn("height", { key: "ArrowUp", shiftKey: true });

    expect(readShapeModel().height).toBe(90);
    expect(readValue("height")).toBe("90");
  });

  it.each(["ctrlKey", "metaKey", "altKey"])(
    "[F44.AC2] leaves Up with %s to the field, the rectangle 120 wide",
    (modifier) => {
      const down = pressIn("width", { key: "ArrowUp", [modifier]: true });

      expect(down.defaultPrevented).toBe(false);
      expect(readShapeModel().width).toBe(120);
    },
  );

  it.each(["ArrowLeft", "ArrowRight", "5"])("[F44.AC2] leaves %s to the field", (key) => {
    const down = pressIn("width", { key });

    expect(down.defaultPrevented).toBe(false);
    expect(readShapeModel().width).toBe(120);
  });
});

describe("one shape's X and Y, Up and Down (US-085)", () => {
  beforeEach(() => {
    openServedPage("");
  });

  it("[F44.AC2] moves the rectangle 10 to the right for Shift + Up in X, X showing 10", () => {
    pressIn("shape-x", { key: "ArrowUp", shiftKey: true });

    expect(readShapeModel().x).toBe(10);
    expect(readValue("shape-x")).toBe("10");
  });

  it("[F44.AC2] moves it 10 up for Shift + Down in Y, Y showing −10, the y axis pointing down", () => {
    pressIn("shape-y", { key: "ArrowDown", shiftKey: true });

    expect(readShapeModel().y).toBe(-10);
    expect(readValue("shape-y")).toBe("-10");
  });

  it("[F44.AC2] moves it 20 to the right for Shift + Up twice within one frame, not 10", () => {
    // The browser holds the redraw until its next frame: the second key starts from the shape
    // the first set, not from the shape as the frame before drew it.
    const frames: FrameRequestCallback[] = [];

    Object.defineProperty(document.defaultView ?? {}, "requestAnimationFrame", {
      configurable: true,
      value: (callback: FrameRequestCallback): number => {
        frames.push(callback);

        return frames.length;
      },
    });

    pressIn("shape-x", { key: "ArrowUp", shiftKey: true });
    pressIn("shape-x", { key: "ArrowUp", shiftKey: true });

    for (const frame of frames) {
      frame(0);
    }

    expect(readShapeModel().x).toBe(20);
    expect(readValue("shape-x")).toBe("20");
  });

  it("[F44.AC2] moves it 20 to the right for Shift + Up twice in X", () => {
    pressIn("shape-x", { key: "ArrowUp", shiftKey: true });
    pressIn("shape-x", { key: "ArrowUp", shiftKey: true });

    expect(readShapeModel().x).toBe(20);
    expect(readValue("shape-x")).toBe("20");
  });
});

describe("a value typed in one shape's fields, relative or fixed (US-085)", () => {
  beforeEach(() => {
    openServedPage("");
  });

  it("[F44.AC2] makes the rectangle 130 wide for +10 in Width and Enter, the field showing 130", () => {
    typeAndEnter("width", "+10");

    expect(readShapeModel().width).toBe(130);
    expect(readValue("width")).toBe("130");
  });

  it("[F44.AC2] moves it to x −20 for -20 in X", () => {
    typeAndEnter("shape-x", "-20");

    expect(readShapeModel().x).toBe(-20);
    expect(readValue("shape-x")).toBe("-20");
  });

  it("[F44.AC2] keeps it 120 wide while 150 is typed, 150 once the field is left", () => {
    typeValue("width", "1");
    typeValue("width", "15");
    typeValue("width", "150");

    // Typed, not applied: the rectangle as it was drawn, 120 wide.
    expect(readShapeModel().width).toBe(120);

    commitField("width");

    expect(readShapeModel().width).toBe(150);
  });

  it("[F44.AC2] sets 50 in X without a sign, the rectangle at x 50", () => {
    typeAndEnter("shape-x", "50");

    expect(readShapeModel().x).toBe(50);
  });

  it.each(["-200", "1.5", "abc", ""])(
    "[F44.AC2] refuses %j in Width, the rectangle 120 wide and the field showing 120",
    (typed) => {
      // 120 − 200 is below 0; a fraction, a word or nothing is no integer.
      typeAndEnter("width", typed);

      expect(readShapeModel().width).toBe(120);
      expect(readValue("width")).toBe("120");
    },
  );

  it("[F44.AC2] caps 200 000 in Width at 100 000 (Q20)", () => {
    typeAndEnter("width", "200000");

    expect(readShapeModel().width).toBe(100_000);
    expect(readValue("width")).toBe("100000");
  });

  it("[F44.AC2] keeps the turned top-left corner of a sharp turned by 30°, +50 in Width", () => {
    // A sharp 100 × 50 at (10, 20) turned by 30° about its middle (60, 45): its turned top-left
    // corner is (60 − 50 cos 30° + 25 sin 30°, 45 − 50 sin 30° − 25 cos 30°) = (29.19873, −1.65064).
    // 150 wide, its middle is that corner + (75 cos 30° − 25 sin 30°, 75 sin 30° + 25 cos 30°)
    // = (81.65064, 57.5), its position (81.65064 − 75, 57.5 − 25) = (6.65064, 32.5); its straight
    // box starts at 81.65064 − (75 cos 30° + 25 sin 30°) = 4.19873 across.
    typeAndEnter("radius", "0");
    typeAndEnter("width", "100");
    typeAndEnter("height", "50");
    typeAndEnter("shape-x", "10");
    typeAndEnter("shape-y", "20");
    typeAndEnter("angle", "30");
    typeAndEnter("width", "+50");

    const shape = readShapeModel();

    expect(shape.width).toBe(150);
    expect(shape.x).toBeCloseTo(6.65064, 5);
    expect(shape.y).toBeCloseTo(32.5, 9);
    expect(readValue("shape-x")).toBe("4.19873");
  });

  it("[F44.AC2] adds 10 to a converted triangle's height of 50√3, kept exact", () => {
    // Step 10, a triangle 100 wide converted to a free shape: 50√3 = 86.60254 high (Q27);
    // +10 makes it 50√3 + 10 = 96.60254 high, exact behind the 5 decimals shown.
    openOnStep(10);
    clickElement("mode-points");
    clickElement("convert");
    typeAndEnter("height", "+10");

    expect(readShapeModel().height).toBeCloseTo(50 * Math.sqrt(3) + 10, 12);
    expect(readValue("height")).toBe("96.60254");
    expect(readText("#status")).toBe("");
  });

  it("[F44.AC2] writes a value typed in a refused shape's Width as typed, as a number field", () => {
    // The angle emptied refuses the shape: no value of its own to add to, 200 read as typed.
    typeAndEnter("angle", "");
    typeAndEnter("width", "200");
    typeAndEnter("angle", "0");

    expect(readShapeModel().width).toBe(200);
  });
});

describe("a text's X, Y and box, alone (US-085)", () => {
  beforeEach(() => {
    openServedPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
    // A text placed by a click, typed, then put in a box 200 × 100 at (40, 40).
    clickElement("tool-text");
    clickAt("#canvas", { clientX: 400, clientY: 300 });
    typeTextContent("Text");
    chooseOption("text-fit", "box");
    typeAndEnter("text-box-width", "200");
    typeAndEnter("text-box-height", "100");
    typeAndEnter("shape-x", "40");
    typeAndEnter("shape-y", "40");
  });

  it("[F44.AC2] makes its box 210 wide for Shift + Up in the box's Width", () => {
    pressIn("text-box-width", { key: "ArrowUp", shiftKey: true });

    expect(readTextModel()).toMatchObject({ boxHeight: 100, boxWidth: 210, x: 40, y: 40 });
    expect(readValue("text-box-width")).toBe("210");
  });

  it("[F44.AC2] keeps the text drawn while +10 is typed in the box's Width, 210 at Enter", () => {
    typeValue("text-box-width", "+10");

    // Typed, not applied: no redraw reads the + as a refused width.
    expect(readText("#status")).toBe("");
    expect(readTextModel().boxWidth).toBe(200);

    commitField("text-box-width");

    expect(readTextModel().boxWidth).toBe(210);
  });

  it("[F44.AC2] makes its box 90 high for -10 in the box's Height", () => {
    typeAndEnter("text-box-height", "-10");

    expect(readTextModel()).toMatchObject({ boxHeight: 90, boxWidth: 200 });
    expect(readValue("text-box-height")).toBe("90");
  });

  it("[F44.AC2] moves it 10 to the right for Shift + Up in X, at 0° its position", () => {
    pressIn("shape-x", { key: "ArrowUp", shiftKey: true });

    expect(readTextModel()).toMatchObject({ x: 50, y: 40 });
    expect(readValue("shape-x")).toBe("50");
  });

  it("[F44.AC2] moves it to y 25 for -15 in Y", () => {
    typeAndEnter("shape-y", "-15");

    expect(readTextModel()).toMatchObject({ x: 40, y: 25 });
  });

  it("[F44.AC2] refuses -300 in the box's Width, the box 200 wide and the field showing 200", () => {
    typeAndEnter("text-box-width", "-300");

    expect(readTextModel().boxWidth).toBe(200);
    expect(readValue("text-box-width")).toBe("200");
  });

  it("[F44.AC2] shows no Width for a text fitting its content, as before", () => {
    chooseOption("text-fit", "content");

    expect(readDisplay("text-box-fields")).toBe("none");
  });
});

describe("several shapes' fields, the same rule (US-085)", () => {
  beforeEach(() => {
    openServedPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
  });

  it("[F44.AC2] makes the rectangle and a second one 10 wider for Shift + Up in W, as one", () => {
    // B placed by the Rectangle tool, typed 40 wide, then both selected by Ctrl + A.
    clickElement("tool-rectangle");
    clickAt("#canvas", { clientX: 600, clientY: 450 });
    typeAndEnter("width", "40");

    selectPageElement("body").dispatchEvent(
      new KeyboardEvent("keydown", { bubbles: true, cancelable: true, ctrlKey: true, key: "a" }),
    );

    pressIn("several-width", { key: "ArrowUp", shiftKey: true });

    // 120 + 10 and 40 + 10, W Mixed.
    const widths = readKeptDrawing(document).map(({ shape }) =>
      isDrawnText(shape) ? undefined : shape.width,
    );

    expect(widths).toEqual([130, 50]);
    expect(selectPageElement("#several-width").getAttribute("placeholder")).toBe("Mixed");
  });

  it("[F44.AC2] marks one shape's fields and several's as number fields, the one rule", () => {
    const ids = ["width", "height", "shape-x", "shape-y", "text-box-width", "text-box-height"];
    const several = ["several-x", "several-y", "several-width", "several-height"];

    for (const id of [...ids, ...several]) {
      const field = selectPageElement(`#${id}`);

      expect(field.getAttribute("type")).toBe("text");
      expect(field.matches(NUMBER_FIELDS)).toBe(true);
    }
  });
});

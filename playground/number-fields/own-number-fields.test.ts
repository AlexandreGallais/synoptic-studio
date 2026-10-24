// @vitest-environment happy-dom
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { EDITED_FIELDS } from "../edited-fields";
import { PAGE_HISTORIES } from "../history";
import { SEVERAL_FIELDS } from "../selection";
import {
  chooseShapeKind,
  clickElement,
  openServedPage,
  readShapeModel,
  readValue,
  selectInput,
  selectPageElement,
  setElementBox,
  typeAndEnter,
  typeInField,
  typeValue,
} from "../user-actions";

import { NUMBER_FIELDS } from "./number-fields";
import { OWN_NUMBER_FIELDS } from "./own-number-fields";
import { readOwnValue } from "./read-own-value";

/** The pause after an arrow's release within which another arrow joins its burst, in ms. */
const PAUSE = 500;

/**
 * Sends a key to a field as the browser sends it: bubbling, cancelable.
 *
 * @param id - the field, without `#`
 * @param type - `keydown` or `keyup`
 * @param init - the key, whether it repeats and the modifiers held
 * @returns the event, telling whether its default was prevented
 */
function sendKey(id: string, type: string, init: KeyboardEventInit): KeyboardEvent {
  const event = new KeyboardEvent(type, { ...init, bubbles: true, cancelable: true });

  selectPageElement(`#${id}`).dispatchEvent(event);

  return event;
}

/**
 * Presses a key in a field, then releases it.
 *
 * @param id - the field, without `#`
 * @param init - the key and the modifiers held
 * @returns the key pressed, telling whether its default was prevented
 */
function pressIn(id: string, init: KeyboardEventInit): KeyboardEvent {
  const down = sendKey(id, "keydown", init);

  sendKey(id, "keyup", init);

  return down;
}

/**
 * Reads the radius of the shape's first vertex, the rectangle's.
 *
 * @returns the radius; NaN for a shape without vertices of its own
 */
function readRadius(): number {
  const model = readShapeModel();

  if (!("corners" in model) || typeof model.corners === "number") {
    return NaN;
  }

  return model.corners[0]?.radius ?? NaN;
}

/**
 * Makes a field with bounds.
 *
 * @param bounds - its least and its most, `data-min` and `data-max`
 * @returns an input outside the page
 */
function makeField(bounds: Readonly<Record<string, string>>): HTMLInputElement {
  const field = document.createElement("input");

  Object.assign(field.dataset, bounds);

  return field;
}

/** Presses Ctrl + Z, then releases it, outside the fields. */
function undo(): void {
  const init = { bubbles: true, cancelable: true, ctrlKey: true, key: "z" };
  const body = selectPageElement("body");

  body.dispatchEvent(new KeyboardEvent("keydown", init));
  body.dispatchEvent(new KeyboardEvent("keyup", init));
}

describe("every number field, the rule of one shape's (US-087)", () => {
  beforeEach(() => {
    // The page as it opens: the rectangle 120 × 80 at (0, 0), selected alone, its radius 10.
    openServedPage("");
  });

  it("[F44.AC4] steps the radius by 1, then by 10 with Shift, the key's default prevented", () => {
    // 10 + 1 = 11, 11 − 1 = 10, 10 + 10 = 20, 20 − 10 = 10.
    const steps = [
      { expected: 11, init: { key: "ArrowUp" } },
      { expected: 10, init: { key: "ArrowDown" } },
      { expected: 20, init: { key: "ArrowUp", shiftKey: true } },
      { expected: 10, init: { key: "ArrowDown", shiftKey: true } },
    ];

    for (const { expected, init } of steps) {
      const down = pressIn("radius", init);

      expect([readValue("radius"), readRadius(), down.defaultPrevented]).toEqual([
        String(expected),
        expected,
        true,
      ]);
    }
  });

  it.each([{ ctrlKey: true }, { metaKey: true }, { altKey: true }])(
    "[F44.AC4] leaves Up to the field with %o held",
    (modifier) => {
      const down = pressIn("radius", { key: "ArrowUp", ...modifier });

      expect([readValue("radius"), down.defaultPrevented]).toEqual(["10", false]);
    },
  );

  it("[F44.AC4] adds 5 to the radius for +5 and Enter, nothing while it is typed", () => {
    selectInput("radius").focus();
    typeValue("radius", "+5");

    expect(readRadius()).toBe(10);

    typeAndEnter("radius", "+5");

    // 10 + 5 = 15.
    expect([readValue("radius"), readRadius()]).toEqual(["15", 15]);
  });

  it.each([
    // 10 − 15 = −5, below the radius's 0.
    { id: "radius", shown: "10", typed: "-15" },
    { id: "radius", shown: "10", typed: "1.5" },
    { id: "angle", shown: "0", typed: "abc" },
    // 100 001, above the size's 100 000.
    { id: "text-size", shown: "32", typed: "100001" },
    { id: "history-memory", shown: "100", typed: "1001" },
  ])("[F44.AC4] refuses $typed in $id, showing $shown again", ({ id, shown, typed }) => {
    typeInField(id, typed);

    expect(readValue(id)).toBe(shown);
  });

  it("[F44.AC4] keeps a polygon's sides at least 3, its 6 shown again for 2", () => {
    chooseShapeKind("polygon");
    typeInField("corners", "2");

    expect(readValue("corners")).toBe("6");

    typeInField("corners", "3");

    expect(readValue("corners")).toBe("3");
  });

  it("[F44.AC4] steps a text's size and the history's memory by 1, and 10 with Shift", () => {
    // 32 + 1, 33 + 10; 100 MB + 1, 101 + 10.
    pressIn("text-size", { key: "ArrowUp" });
    pressIn("text-size", { key: "ArrowUp", shiftKey: true });
    pressIn("history-memory", { key: "ArrowUp" });
    pressIn("history-memory", { key: "ArrowUp", shiftKey: true });

    expect([readValue("text-size"), readValue("history-memory")]).toEqual(["43", "111"]);
  });

  it("[F44.AC4] applies nothing for a value refused, the zoom of 141 % kept exactly", () => {
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
    clickElement("zoom-in");

    const shown = document.querySelector("#canvas svg")?.getAttribute("viewBox");

    // × √2 shows 141, which applied again would zoom to 141 % exactly.
    typeInField("zoom-field", "abc");

    expect(readValue("zoom-field")).toBe("141");
    expect(document.querySelector("#canvas svg")?.getAttribute("viewBox")).toBe(shown);
  });

  it("[F44.AC4] keeps a field at its bound for Up past it", () => {
    typeInField("history-memory", "1000");
    pressIn("history-memory", { key: "ArrowUp" });

    // 1 000 + 1 above the memory's 1 000 MB: refused, 1 000 shown again.
    expect(readValue("history-memory")).toBe("1000");
  });

  it("[F44.AC4] steps nothing in a field showing no number, a radius emptied as Mixed shows it", () => {
    const field = selectInput("radius");

    // Given the focus at 10, then left and emptied by the page, as a radius Mixed shows none.
    field.focus();
    field.blur();
    field.value = "";
    field.focus();

    const down = pressIn("radius", { key: "ArrowUp" });

    expect([readValue("radius"), down.defaultPrevented]).toEqual(["", true]);
  });

  it("[F44.AC4] steps the number shown before a text typed and not applied", () => {
    selectInput("radius").focus();
    typeValue("radius", "+5");
    pressIn("radius", { key: "ArrowUp" });

    // 10 + 1, the +5 dropped.
    expect([readValue("radius"), readRadius()]).toEqual(["11", 11]);
  });

  it("[F44.AC4] steps the number shown before 5 typed with its key, not the 5", () => {
    selectInput("radius").focus();
    pressIn("radius", { key: "5" });
    typeValue("radius", "5");
    pressIn("radius", { key: "ArrowUp" });

    // 10 + 1: the 5 typed, a number, is no value applied.
    expect([readValue("radius"), readRadius()]).toEqual(["11", 11]);
  });

  it("[F44.AC4] adds -5 to the Angle at 30, each key of it starting from 30, not from the text", () => {
    typeInField("angle", "30");
    typeInField("angle", "-5");

    // 30 − 5 = 25: the keys of `-`, `5` and Enter each sent before the text applied.
    expect(readValue("angle")).toBe("25");
  });

  it("[F44.AC4] draws the angle applied, not the one typed, at a redraw before Enter", () => {
    selectInput("angle").focus();
    pressIn("angle", { key: "3" });
    typeValue("angle", "30");

    selectPageElement("#canvas").dispatchEvent(
      new WheelEvent("wheel", { bubbles: true, cancelable: true, deltaY: 100 }),
    );

    // The wheel's pan redraws the page from the angle applied, 0, never the 30 typed.
    expect([readShapeModel().angle, readValue("angle")]).toEqual([0, "30"]);
  });

  it("[F44.AC4] finds every number field of the page under the rule", () => {
    const shared = new Set([...EDITED_FIELDS, ...SEVERAL_FIELDS].map(([, id]) => id));
    const fields = [...document.querySelectorAll(NUMBER_FIELDS)];
    const others = fields.filter(
      (field) => !(field.matches(OWN_NUMBER_FIELDS) || shared.has(field.id)),
    );

    expect(document.querySelectorAll('input[type="number"]')).toHaveLength(0);
    expect(others.map((field) => field.id)).toEqual([]);
    // Every field of the panel and the toolbar: one shape's 6, several's 4 and their own 25.
    expect(fields).toHaveLength(35);
  });
});

describe("a burst of arrows in the radius, one step (US-087)", () => {
  beforeEach(() => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });
    openServedPage("");
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("[F44.AC4] makes the radius 40 for Up held 30 keys, one Ctrl + Z giving 10", () => {
    const sent = Array.from({ length: 30 }).keys();

    for (const index of sent) {
      sendKey("radius", "keydown", { key: "ArrowUp", repeat: index > 0 });
    }

    sendKey("radius", "keyup", { key: "ArrowUp" });

    // 10 + 30 × 1, the step taken 500 ms after the release.
    expect(readRadius()).toBe(40);
    expect(PAGE_HISTORIES.get(document)?.past).toHaveLength(0);

    vi.advanceTimersByTime(PAUSE);
    undo();

    expect([readValue("radius"), readRadius()]).toEqual(["10", 10]);
  });
});

describe("readOwnValue (US-087)", () => {
  it.each([
    // 3 typed in a field from 3 to 12; 12 its most; 10 + 2.
    {
      bounds: { max: "12", min: "3" },
      expected: 3,
      shown: 6,
      typed: { isRelative: false, value: 3 },
    },
    {
      bounds: { max: "12", min: "3" },
      expected: 12,
      shown: 6,
      typed: { isRelative: false, value: 12 },
    },
    {
      bounds: { max: "12", min: "3" },
      expected: 12,
      shown: 10,
      typed: { isRelative: true, value: 2 },
    },
    // No bound: −100 000 typed kept.
    { bounds: {}, expected: -100_000, shown: 0, typed: { isRelative: false, value: -100_000 } },
    // A fixed value needs no number shown.
    { bounds: {}, expected: 7, shown: undefined, typed: { isRelative: false, value: 7 } },
  ])("gives $expected for $typed.value from $shown", ({ bounds, expected, shown, typed }) => {
    expect(readOwnValue(makeField(bounds), typed, shown)).toBe(expected);
  });

  it.each([
    // 2 below 3; 10 + 3 above 12; +1 from no number shown.
    { bounds: { max: "12", min: "3" }, shown: 6, typed: { isRelative: false, value: 2 } },
    { bounds: { max: "12", min: "3" }, shown: 10, typed: { isRelative: true, value: 3 } },
    { bounds: {}, shown: undefined, typed: { isRelative: true, value: 1 } },
  ])("refuses $typed.value from $shown", ({ bounds, shown, typed }) => {
    expect(readOwnValue(makeField(bounds), typed, shown)).toBeUndefined();
  });
});

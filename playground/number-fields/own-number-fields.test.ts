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
  readText,
  readValue,
  selectInput,
  selectPageElement,
  setElementBox,
  typeAndEnter,
  typeInField,
  typeValue,
} from "../user-actions";

import { NUMBER_FIELDS } from "./number-fields";
import { OWN_FIELD_RANGES } from "./own-field-ranges";
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
 * Makes a field of the id of one of the page's.
 *
 * @param id - its id, its range read by it (`OWN_FIELD_RANGES`)
 * @returns an input outside the page
 */
function makeField(id: string): HTMLInputElement {
  const field = document.createElement("input");

  field.id = id;

  return field;
}

/** Presses Ctrl + Z, then releases it, outside the fields. */
function undo(): void {
  const init = { bubbles: true, cancelable: true, ctrlKey: true, key: "z" };
  const body = selectPageElement("body");

  body.dispatchEvent(new KeyboardEvent("keydown", init));
  body.dispatchEvent(new KeyboardEvent("keyup", init));
}

describe("every number field, the rule of one shape's", () => {
  beforeEach(() => {
    // The page as it opens: the rectangle 120 × 80 at (0, 0), selected alone, its radius 10; the
    // history's bound the browser keeps forgotten, a memory taken at 1 000 MB by a test before.
    localStorage.clear();
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
    // 10 − 15 = −5, taken at the radius's 0; 1.5 rounded to 2, a tie up; 10 + 1.5 = 11.5 → 12.
    { id: "radius", shown: "0", typed: "-15" },
    { id: "radius", shown: "2", typed: "1.5" },
    { id: "radius", shown: "12", typed: "+1.5" },
    // A comma is a decimal separator: 10 + 1,5 = 11.5 → 12.
    { id: "radius", shown: "12", typed: "+1,5" },
    // 100 001 taken at the size's 100 000; 32 − 40 = −8 at its least, 1.
    { id: "text-size", shown: "100000", typed: "100001" },
    { id: "text-size", shown: "1", typed: "-40" },
    // 1 001 MB at the memory's 1 000; 2.5 MB rounded to 3.
    { id: "history-memory", shown: "1000", typed: "1001" },
    { id: "history-memory", shown: "3", typed: "2.5" },
  ])("[F44.AC4] takes $typed in $id as $shown, within its range", ({ id, shown, typed }) => {
    typeInField(id, typed);

    expect(readValue(id)).toBe(shown);
  });

  it.each([
    { id: "angle", shown: "0", typed: "abc" },
    { id: "radius", shown: "10", typed: "" },
    { id: "radius", shown: "10", typed: "-25+12" },
    { id: "text-size", shown: "32", typed: "+" },
  ])("[F44.AC4] refuses $typed in $id, no number, showing $shown again", ({ id, shown, typed }) => {
    typeInField(id, typed);

    expect(readValue(id)).toBe(shown);
  });

  it("[F44.AC4] goes round the Angle: -20 at 10 gives 350, 370 gives 10", () => {
    typeInField("angle", "10");
    typeInField("angle", "-20");

    // 10 − 20 = −10, kept from 0 to 359: 350.
    expect([readValue("angle"), readShapeModel().angle]).toEqual(["350", 350]);

    typeInField("angle", "370");

    expect([readValue("angle"), readShapeModel().angle]).toEqual(["10", 10]);
  });

  it("[F44.AC4] keeps a polygon's sides from 3 to 12: 2 gives 3, 13 gives 12", () => {
    chooseShapeKind("polygon");
    typeInField("corners", "2");

    expect(readValue("corners")).toBe("3");

    typeInField("corners", "13");

    expect([readValue("corners"), readText("#status")]).toEqual(["12", ""]);
  });

  it("[F44.AC4] steps a text's size and the history's memory by 1, and 10 with Shift", () => {
    // 32 + 1, 33 + 10; 100 MB + 1, 101 + 10.
    pressIn("text-size", { key: "ArrowUp" });
    pressIn("text-size", { key: "ArrowUp", shiftKey: true });
    pressIn("history-memory", { key: "ArrowUp" });
    pressIn("history-memory", { key: "ArrowUp", shiftKey: true });

    expect([readValue("text-size"), readValue("history-memory")]).toEqual(["43", "111"]);
  });

  it.each(["text-letter-spacing", "text-word-spacing"])(
    "steps %s at −5 by 1 and 10, never reading the step again as typed",
    (id) => {
      // −5 + 1 = −4, then −4 + 10 = 6; `-4` shown read again as a relative value would give
      // −5 − 4 = −9, then −9 + 10 = 1 read as fixed.
      typeInField(id, "-5");
      pressIn(id, { key: "ArrowUp" });

      expect(readValue(id)).toBe("-4");

      pressIn(id, { key: "ArrowUp", shiftKey: true });

      expect(readValue(id)).toBe("6");
    },
  );

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

    // 1 000 + 1 above the memory's 1 000 MB: taken at 1 000.
    expect(readValue("history-memory")).toBe("1000");
  });

  it("[F44.AC4] adds Up's 1 to each vertex's own radius in a field showing no number, as Mixed", () => {
    const field = selectInput("radius");

    // Given the focus at 10, then left and emptied by the page, as a radius Mixed shows none.
    field.focus();
    field.blur();
    field.value = "";
    field.focus();

    const down = pressIn("radius", { key: "ArrowUp" });

    // Each vertex's 10 + 1, the field then showing their common 11.
    expect([readValue("radius"), readRadius(), down.defaultPrevented]).toEqual(["11", 11, true]);
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

describe("a burst of arrows in the radius, one step", () => {
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

describe("readOwnValue", () => {
  it.each([
    // Corners, 3 to 12: 3 kept; 2 at 3; 10 + 3 = 13 at 12; 6 + 1.5 = 7.5 rounded to 8.
    { expected: 3, id: "corners", shown: 6, typed: { isRelative: false, value: 3 } },
    { expected: 3, id: "corners", shown: 6, typed: { isRelative: false, value: 2 } },
    { expected: 12, id: "corners", shown: 10, typed: { isRelative: true, value: 3 } },
    { expected: 8, id: "corners", shown: 6, typed: { isRelative: true, value: 1.5 } },
    // The Angle goes round: 10 − 20 = −10 is 350, 350 + 20 = 370 is 10, 360 is 0.
    { expected: 350, id: "angle", shown: 10, typed: { isRelative: true, value: -20 } },
    { expected: 10, id: "angle", shown: 350, typed: { isRelative: true, value: 20 } },
    { expected: 0, id: "angle", shown: 0, typed: { isRelative: false, value: 360 } },
    // The picker's hue is an angle: 370 is 10.
    { expected: 10, id: "color-hue-value", shown: 0, typed: { isRelative: false, value: 370 } },
    // The zoom keeps a fraction and starts at 1 %: 141 + 0.5; −50 at 1.
    { expected: 141.5, id: "zoom-field", shown: 141, typed: { isRelative: true, value: 0.5 } },
    { expected: 1, id: "zoom-field", shown: 100, typed: { isRelative: false, value: -50 } },
    // A field the table does not name: whole numbers, no bound.
    { expected: -100_000, id: "", shown: 0, typed: { isRelative: false, value: -100_000 } },
    { expected: 3, id: "", shown: 0, typed: { isRelative: false, value: 2.5 } },
    // A fraction shown is rounded whatever is typed: 33.54102 + 1 = 34.54102 to 35,
    // 11.83476 + 1 = 12.83476 to 13, 0.5 − 1 = −0.5 to 0, a tie up; an integer fixed is kept.
    { expected: 35, id: "", shown: 33.54102, typed: { isRelative: true, value: 1 } },
    { expected: 13, id: "", shown: 11.83476, typed: { isRelative: true, value: 1 } },
    { expected: 0, id: "", shown: 0.5, typed: { isRelative: true, value: -1 } },
    { expected: 4, id: "", shown: 11.83476, typed: { isRelative: false, value: 4 } },
    // A fixed value needs no number shown.
    { expected: 7, id: "radius", shown: undefined, typed: { isRelative: false, value: 7 } },
  ])("gives $expected for $typed.value in $id from $shown", ({ expected, id, shown, typed }) => {
    expect(readOwnValue(makeField(id), typed, shown)).toBe(expected);
  });

  it("leaves a relative value from no number shown to what the field sets, a Mixed radius", () => {
    expect(readOwnValue(makeField("radius"), { isRelative: true, value: 1 }, undefined)).toBe(
      undefined,
    );
  });

  it("[F44.AC4] names every field of its own in its table, bounds from what each sets", () => {
    openServedPage("");

    const ids = [...document.querySelectorAll(OWN_NUMBER_FIELDS)].map((field) => field.id);

    expect(new Set(ids)).toEqual(new Set(OWN_FIELD_RANGES.keys()));
    expect(ids).toHaveLength(OWN_FIELD_RANGES.size);
    expect(OWN_FIELD_RANGES.get("corners")).toMatchObject({ maximum: 12, minimum: 3 });
  });
});

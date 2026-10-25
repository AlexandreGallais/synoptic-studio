// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import {
  chooseOption,
  clickElement,
  clickPoint,
  openPage,
  readText,
  selectInput,
  selectPageElement,
  typeInField,
} from "../../../../../testing";

/** The four handle fields, before x and y, then after x and y. */
const HANDLE_FIELDS = ["before-x", "before-y", "after-x", "after-y"] as const;

/** An arrow's key: Up or Down, Shift held or not. */
type Arrow = { readonly key: "ArrowDown" | "ArrowUp"; readonly shiftKey: boolean };

/** Up, by 1. */
const UP: Arrow = { key: "ArrowUp", shiftKey: false };

/** Shift + Up, by 10. */
const SHIFT_UP: Arrow = { key: "ArrowUp", shiftKey: true };

/** Down, by 1. */
const DOWN: Arrow = { key: "ArrowDown", shiftKey: false };

/** Shift + Down, by 10. */
const SHIFT_DOWN: Arrow = { key: "ArrowDown", shiftKey: true };

/**
 * Presses an arrow in a field as a designer does and a browser sends it: the field given the
 * focus, then the key pressed and released, bubbling, cancelable.
 *
 * @param id - the field, without `#`
 * @param arrow - the key and whether Shift is held
 */
function pressArrow(id: string, arrow: Arrow): void {
  const field = selectPageElement(`#${id}`);
  const init = { ...arrow, bubbles: true, cancelable: true };

  field.focus();
  field.dispatchEvent(new KeyboardEvent("keydown", init));
  field.dispatchEvent(new KeyboardEvent("keyup", init));
}

/**
 * The values of the four handle fields, as written in them.
 *
 * @returns before x and y, then after x and y
 */
function readHandleFields(): readonly string[] {
  return HANDLE_FIELDS.map((field) => selectInput(`handle-${field}`).value);
}

/**
 * Types point 1's handles from the side: after at 30°, length 20, from its side going right,
 * before mirrored. Point 1's corner is square, its before side going down: the angles of two
 * opposite handles add up to 270° there (the after (12, 5) at 22.61986° shown before at
 * 247.38014°), so that before is at 270 − 30 = 240°, length 20.
 *
 * @param mode - the handles' mode, chosen once they are typed
 */
function typeFromTheSide(mode: string): void {
  chooseOption("handle-method", "side");
  typeInField("handle-after-x", "30");
  typeInField("handle-after-y", "20");
  chooseOption("handle-mode", mode);
}

/** One arrow pressed in one handle field, and the four fields it leaves. */
type StepCase = {
  readonly arrow: Arrow;
  readonly expected: readonly string[];
  readonly field: (typeof HANDLE_FIELDS)[number];
};

describe("a handle's fields stepped by the arrows", () => {
  beforeEach(() => {
    // The rectangle 120 × 80, radius 0, so that no fillet cuts a side: point 1 at (0, 0), its
    // after side towards point 2 (120, 0), its before side towards point 4 (0, 80). Point 1
    // given handles in Mirror angle and length: after (12, 5) typed, before mirrored at
    // (−12, −5), both of length 13 (5, 12, 13), on the page the vectors themselves.
    openPage("");
    typeInField("radius", "0");
    clickElement("mode-points");
    clickPoint(1, false);
    clickElement("handles-switch");
    typeInField("handle-after-x", "12");
    typeInField("handle-after-y", "5");
  });

  it("[F45.AC1] steps Before y at −5 by 1 at Up, 10 at Shift + Up, back at Down", () => {
    // Before (−12, −5), after its mirror (12, 5). Up: −5 + 1 = −4, after y 4; Up: −3; Shift + Up:
    // −3 + 10 = 7, after y −7; Down: 6; Shift + Down: 6 − 10 = −4. Before the fix the step's
    // value, −4, was read again as typed, a relative −4 added to the −5 shown: −9, then −17,
    // the value doubling at each key.
    const steps: readonly (readonly [Arrow, readonly string[]])[] = [
      [UP, ["-12", "-4", "12", "4"]],
      [UP, ["-12", "-3", "12", "3"]],
      [SHIFT_UP, ["-12", "7", "12", "-7"]],
      [DOWN, ["-12", "6", "12", "-6"]],
      [SHIFT_DOWN, ["-12", "-4", "12", "4"]],
    ];

    expect(readHandleFields()).toEqual(["-12", "-5", "12", "5"]);

    for (const [arrow, expected] of steps) {
      pressArrow("handle-before-y", arrow);

      expect(readHandleFields()).toEqual(expected);
    }

    // The handle drawn as stepped: point 1's curve towards point 2 starts at the after handle's
    // end, (12, 4).
    expect(readText("#path-data")).toMatch(/^M0 0 C12 4 /u);
  });

  // On the page, the fields are the handles' ends, point 1 at (0, 0): the vectors themselves.
  // Mirror angle and length: the other handle the opposite, −(x, y).
  const pageMirrored: readonly StepCase[] = [
    { arrow: UP, expected: ["-11", "-5", "11", "5"], field: "before-x" },
    { arrow: SHIFT_UP, expected: ["-2", "-5", "2", "5"], field: "before-x" },
    { arrow: UP, expected: ["-12", "-4", "12", "4"], field: "before-y" },
    { arrow: SHIFT_UP, expected: ["-12", "5", "12", "-5"], field: "before-y" },
    { arrow: UP, expected: ["-13", "-5", "13", "5"], field: "after-x" },
    { arrow: SHIFT_UP, expected: ["-22", "-5", "22", "5"], field: "after-x" },
    { arrow: UP, expected: ["-12", "-6", "12", "6"], field: "after-y" },
    { arrow: SHIFT_UP, expected: ["-12", "-15", "12", "15"], field: "after-y" },
  ];

  it.each(pageMirrored)(
    "[F45.AC1] steps $field on the page, mirroring angle and length",
    ({ arrow, expected, field }) => {
      pressArrow(`handle-${field}`, arrow);

      expect(readHandleFields()).toEqual(expected);
    },
  );

  // Mirror angle: the other handle opposite, its length kept at 13, −(x, y) · 13 / |(x, y)|,
  // to 5 decimals. Before (−11, −5): |(−11, −5)| = √146 = 12.083046, 13 / 12.083046 =
  // 1.075887, after (11.83476, 5.37944). Before (−2, −5): √29 = 5.385165, 13 / 5.385165 =
  // 2.414023, after (4.82808, 12.0702). Before (−12, −4): √160 = 12.649111, 1.027740, after
  // (12.33288, 4.11096). Before (−12, 5): length 13, after (12, −5). After (13, 5): √194 =
  // 13.928388, 0.933345, before (−12.13349, −4.66673). After (22, 5): √509 = 22.561028,
  // 0.576215, before (−12.67673, −2.88107). After (12, 6): √180 = 13.416408, 0.968963, before
  // (−11.62755, −5.81378). After (12, 15): √369 = 19.209373, 0.676753, before (−8.12104,
  // −10.15129).
  const pageAngle: readonly StepCase[] = [
    { arrow: UP, expected: ["-11", "-5", "11.83476", "5.37944"], field: "before-x" },
    { arrow: SHIFT_UP, expected: ["-2", "-5", "4.82808", "12.0702"], field: "before-x" },
    { arrow: UP, expected: ["-12", "-4", "12.33288", "4.11096"], field: "before-y" },
    { arrow: SHIFT_UP, expected: ["-12", "5", "12", "-5"], field: "before-y" },
    { arrow: UP, expected: ["-12.13349", "-4.66673", "13", "5"], field: "after-x" },
    { arrow: SHIFT_UP, expected: ["-12.67673", "-2.88107", "22", "5"], field: "after-x" },
    { arrow: UP, expected: ["-11.62755", "-5.81378", "12", "6"], field: "after-y" },
    { arrow: SHIFT_UP, expected: ["-8.12104", "-10.15129", "12", "15"], field: "after-y" },
  ];

  it.each(pageAngle)(
    "[F45.AC1] steps $field on the page, mirroring the angle",
    ({ arrow, expected, field }) => {
      chooseOption("handle-mode", "angle");
      pressArrow(`handle-${field}`, arrow);

      expect(readHandleFields()).toEqual(expected);
    },
  );

  // No mirroring: the other handle stays at its end, (−12, −5) or (12, 5).
  const pageAlone: readonly StepCase[] = [
    { arrow: UP, expected: ["-11", "-5", "12", "5"], field: "before-x" },
    { arrow: SHIFT_UP, expected: ["-2", "-5", "12", "5"], field: "before-x" },
    { arrow: UP, expected: ["-12", "-4", "12", "5"], field: "before-y" },
    { arrow: SHIFT_UP, expected: ["-12", "5", "12", "5"], field: "before-y" },
    { arrow: UP, expected: ["-12", "-5", "13", "5"], field: "after-x" },
    { arrow: SHIFT_UP, expected: ["-12", "-5", "22", "5"], field: "after-x" },
    { arrow: UP, expected: ["-12", "-5", "12", "6"], field: "after-y" },
    { arrow: SHIFT_UP, expected: ["-12", "-5", "12", "15"], field: "after-y" },
  ];

  it.each(pageAlone)(
    "[F45.AC1] steps $field on the page, mirroring nothing",
    ({ arrow, expected, field }) => {
      chooseOption("handle-mode", "none");
      pressArrow(`handle-${field}`, arrow);

      expect(readHandleFields()).toEqual(expected);
    },
  );

  // From the side, x is the angle and y the length. Mirror angle and length: the other angle
  // 270 − the one stepped, the other length the same.
  const sideMirrored: readonly StepCase[] = [
    { arrow: UP, expected: ["241", "20", "29", "20"], field: "before-x" },
    { arrow: SHIFT_UP, expected: ["250", "20", "20", "20"], field: "before-x" },
    { arrow: UP, expected: ["240", "21", "30", "21"], field: "before-y" },
    { arrow: SHIFT_UP, expected: ["240", "30", "30", "30"], field: "before-y" },
    { arrow: UP, expected: ["239", "20", "31", "20"], field: "after-x" },
    { arrow: SHIFT_UP, expected: ["230", "20", "40", "20"], field: "after-x" },
    { arrow: UP, expected: ["240", "21", "30", "21"], field: "after-y" },
    { arrow: SHIFT_UP, expected: ["240", "30", "30", "30"], field: "after-y" },
  ];

  it.each(sideMirrored)(
    "[F45.AC1] steps $field from the side, mirroring angle and length",
    ({ arrow, expected, field }) => {
      typeFromTheSide("angle-and-length");
      pressArrow(`handle-${field}`, arrow);

      expect(readHandleFields()).toEqual(expected);
    },
  );

  // Mirror angle: the other angle 270 − the one stepped, the other length kept at 20.
  const sideAngle: readonly StepCase[] = [
    { arrow: UP, expected: ["241", "20", "29", "20"], field: "before-x" },
    { arrow: SHIFT_UP, expected: ["250", "20", "20", "20"], field: "before-x" },
    { arrow: UP, expected: ["240", "21", "30", "20"], field: "before-y" },
    { arrow: SHIFT_UP, expected: ["240", "30", "30", "20"], field: "before-y" },
    { arrow: UP, expected: ["239", "20", "31", "20"], field: "after-x" },
    { arrow: SHIFT_UP, expected: ["230", "20", "40", "20"], field: "after-x" },
    { arrow: UP, expected: ["240", "20", "30", "21"], field: "after-y" },
    { arrow: SHIFT_UP, expected: ["240", "20", "30", "30"], field: "after-y" },
  ];

  it.each(sideAngle)(
    "[F45.AC1] steps $field from the side, mirroring the angle",
    ({ arrow, expected, field }) => {
      typeFromTheSide("angle");
      pressArrow(`handle-${field}`, arrow);

      expect(readHandleFields()).toEqual(expected);
    },
  );

  // No mirroring: the other handle stays at 240° or 30°, length 20.
  const sideAlone: readonly StepCase[] = [
    { arrow: UP, expected: ["241", "20", "30", "20"], field: "before-x" },
    { arrow: SHIFT_UP, expected: ["250", "20", "30", "20"], field: "before-x" },
    { arrow: UP, expected: ["240", "21", "30", "20"], field: "before-y" },
    { arrow: SHIFT_UP, expected: ["240", "30", "30", "20"], field: "before-y" },
    { arrow: UP, expected: ["240", "20", "31", "20"], field: "after-x" },
    { arrow: SHIFT_UP, expected: ["240", "20", "40", "20"], field: "after-x" },
    { arrow: UP, expected: ["240", "20", "30", "21"], field: "after-y" },
    { arrow: SHIFT_UP, expected: ["240", "20", "30", "30"], field: "after-y" },
  ];

  it.each(sideAlone)(
    "[F45.AC1] steps $field from the side, mirroring nothing",
    ({ arrow, expected, field }) => {
      typeFromTheSide("none");
      pressArrow(`handle-${field}`, arrow);

      expect(readHandleFields()).toEqual(expected);
    },
  );

  it("rounds Up on a handle showing a fraction, mirroring the angle", () => {
    // Up in Before x: before (−11, −5), after mirrored at (11.83476, 5.37944), as above. Up in
    // After x: 11.83476 + 1 = 12.83476, rounded to 13, the handle drawn at (13, 5.37944), where
    // before the fix the fraction was refused and the handle left unmoved. Before kept at its
    // length √146 = 12.0830460 opposite (13, 65 / √146 = 5.3794383): |(13, 5.3794383)| =
    // √197.938356 = 14.0690567, 12.0830460 / 14.0690567 = 0.8588384, before (−13 · 0.8588384,
    // −5.3794383 · 0.8588384) = (−11.1649, −4.62007), its fifth decimal a 0 dropped.
    chooseOption("handle-mode", "angle");
    pressArrow("handle-before-x", UP);
    pressArrow("handle-after-x", UP);

    expect(readHandleFields()).toEqual(["-11.1649", "-4.62007", "13", "5.37944"]);
    expect(readText("#path-data")).toMatch(/^M0 0 C13 5.37944 /u);
  });

  it("rounds Up on an angle from the side showing a fraction", () => {
    // After (12, 5) seen from its side going right: angle atan(5 / 12) = 22.61986°, length 13,
    // before mirrored at 270 − 22.61986 = 247.38014°. Up: 22.61986 + 1 = 23.61986, rounded to
    // 24; before at 270 − 24 = 246°, length 13.
    chooseOption("handle-method", "side");

    expect(readHandleFields()).toEqual(["247.38014", "13", "22.61986", "13"]);

    pressArrow("handle-after-x", UP);

    expect(readHandleFields()).toEqual(["246", "13", "24", "13"]);
  });

  it("rounds +1 typed on an angle from the side showing a fraction", () => {
    // Before at 247.38014°: `+1` gives 248.38014, rounded to 248; after at 270 − 248 = 22°.
    chooseOption("handle-method", "side");
    typeInField("handle-before-x", "+1");

    expect(readHandleFields()).toEqual(["248", "13", "22", "13"]);
  });

  it("[F45.AC1] steps a point's X and Y at −5 by 1 and 10, its handles moving with it", () => {
    // X −5: Up −4, Shift + Up 6; Y −5: Up −4, Shift + Down −14. The handles' vectors kept,
    // (−12, −5) and (12, 5), so that their ends on the page move with the point: at (6, −14),
    // before at (6 − 12, −14 − 5) = (−6, −19), after at (6 + 12, −14 + 5) = (18, −9).
    typeInField("point-x", "-5");
    typeInField("point-y", "-5");
    pressArrow("point-x", UP);

    expect(selectInput("point-x").value).toBe("-4");

    pressArrow("point-x", SHIFT_UP);
    pressArrow("point-y", UP);

    expect([selectInput("point-x").value, selectInput("point-y").value]).toEqual(["6", "-4"]);

    pressArrow("point-y", SHIFT_DOWN);

    expect([selectInput("point-x").value, selectInput("point-y").value]).toEqual(["6", "-14"]);
    expect(readHandleFields()).toEqual(["-6", "-19", "18", "-9"]);
  });
});

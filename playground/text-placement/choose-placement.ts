import {
  isZeroVector,
  positionKeepingPoint,
  straightBoxShift,
  subtract,
  textFrame,
} from "../../src";

import { readStraightStart } from "./read-straight-start";
import { readTurnedCorner } from "./read-turned-corner";

import type { DrawnTextRecord } from "./drawn-text-record";
import type { Point, SizedText, Text, Vector } from "../../src";

/** The top-left of a frame before it turns, relative to itself: the corner a growing text keeps. */
const TOP_LEFT: Point = { x: 0, y: 0 };

/**
 * Reads how much a text's frame changed size since the text last drawn: none when its angle
 * changed, so that a turn keeps the position (ADR-0055, decision 3).
 *
 * @kind procedure
 * @param drawn - the text last drawn
 * @param sized - the text read from the fields, and its measured block
 * @returns the change of its width and of its height
 * @see US-054
 */
function readFrameChange(drawn: DrawnTextRecord, sized: SizedText): Vector {
  const before = textFrame(drawn.text, drawn.block);
  const after = drawn.text.angle === sized.text.angle ? textFrame(sized.text, sized.block) : before;

  return subtract({ x: after.width, y: after.height }, { x: before.width, y: before.height });
}

/**
 * Chooses where a text is drawn before X and Y move it: a first text at the point typed; a frame
 * that changed size at the same angle at the position keeping the turned top-left corner of the
 * text last drawn (`positionKeepingPoint`, `DERIV-turned-shape` step 5), so that a turned text
 * that grows keeps its corner (US-038, ADR-0057); otherwise, an angle changed included, at the
 * position of the text last drawn, so that a text turns about its middle (ADR-0055).
 *
 * @kind procedure
 * @param drawn - the text last drawn, nothing before the first
 * @param sized - the text read from the fields, its position the start X and Y show, and its
 * measured block
 * @returns that position, the top-left of its frame before it turns
 * @see US-038
 */
function chooseBasePosition(drawn: DrawnTextRecord | undefined, sized: SizedText): Point {
  if (drawn === undefined) {
    return { x: sized.text.x, y: sized.text.y };
  }

  if (isZeroVector(readFrameChange(drawn, sized))) {
    return { x: drawn.text.x, y: drawn.text.y };
  }

  const { height, width } = textFrame(sized.text, sized.block);

  return positionKeepingPoint(
    { angle: sized.text.angle, height, width },
    TOP_LEFT,
    readTurnedCorner(drawn),
  );
}

/** Where a straight box starts on one axis, as X or Y shows it, as the text last drawn did, and at the base position. */
type AxisStarts = {
  /** Where the straight box of the text last drawn starts. */
  readonly kept: number;
  /** What X or Y shows (`readTextPosition`). */
  readonly shown: number;
  /** Where the straight box of the text at its base position starts. */
  readonly start: number;
};

/**
 * Chooses where the straight box must start on one axis: the value X or Y shows, or, while it
 * still shows the start of the text last drawn, where the text at its base position starts.
 *
 * @kind procedure
 * @param starts - what the field shows, the start kept, the start at the base position
 * @returns the start sought on that axis
 * @see RULE-shapes-scripts-and-turning
 */
function chooseAxisTarget(starts: AxisStarts): number {
  return starts.shown === starts.kept ? starts.start : starts.shown;
}

/**
 * Chooses where the straight box must start: on each axis the value X or Y shows, except an axis
 * still showing the start of the text last drawn, which keeps where the text placed at its base
 * position starts, so that only a value typed moves it.
 *
 * @kind procedure
 * @param drawn - the text last drawn, nothing before the first
 * @param shown - the start X and Y show (`readTextPosition`)
 * @param start - where the straight box of the text at its base position starts
 * @returns the start sought
 * @see RULE-shapes-scripts-and-turning
 */
function chooseTargetStart(drawn: DrawnTextRecord | undefined, shown: Point, start: Point): Point {
  if (drawn === undefined) {
    return shown;
  }

  const kept = readStraightStart(drawn);

  return {
    x: chooseAxisTarget({ kept: kept.x, shown: shown.x, start: start.x }),
    y: chooseAxisTarget({ kept: kept.y, shown: shown.y, start: start.y }),
  };
}

/**
 * Chooses where a text is drawn from the start of its straight box X and Y show, as a shape's
 * (Product Owner, 2026-10-23, AUD-040): from its base position (`chooseBasePosition`), a growing
 * turned text keeping its turned top-left corner (ADR-0057) and a turn keeping its middle
 * (ADR-0055), it moves by how far its straight box starts from the value typed
 * (`chooseTargetStart`, `straightBoxShift`), its position, the top-left of its frame before it
 * turns, possibly fractional (ADR-0035). At 0° the start is the position, bit for bit.
 *
 * @kind procedure
 * @param drawn - the text last drawn, nothing before the first
 * @param sized - the text read from the fields, its position the start X and Y show, and its
 * measured block
 * @returns the text at the position it is drawn at, the top-left of its frame before it turns
 * @see RULE-shapes-scripts-and-turning
 */
export function choosePlacement(drawn: DrawnTextRecord | undefined, sized: SizedText): Text {
  const base = { ...sized.text, ...chooseBasePosition(drawn, sized) };
  const start = readStraightStart({ block: sized.block, text: base });
  const target = chooseTargetStart(drawn, sized.text, start);

  return {
    ...base,
    x: base.x + straightBoxShift(start.x, target.x),
    y: base.y + straightBoxShift(start.y, target.y),
  };
}

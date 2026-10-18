import { isZeroVector, positionKeepingPoint, subtract, textFrame } from "../../src";

import { readTurnedCorner } from "./read-turned-corner";

import type { DrawnTextRecord } from "./drawn-text-record";
import type { Point, SizedText, Text, Vector } from "../../src";

/** The top-left of a frame before it turns, relative to itself: the point X and Y place. */
const TOP_LEFT: Point = { x: 0, y: 0 };

/**
 * Reads how much a text's frame changed size since the text last drawn: none when its angle
 * changed, so that a turn keeps the position (ADR-0055, decision 3).
 *
 * @kind procedure
 * @param drawn - the text last drawn
 * @param sized - the text read from the fields, and its measured block
 * @returns the change of its width and of its height
 * @see docs/backlog/stories/E15-F17-US-054-turned-text-position-shows-its-corner.md
 */
function readFrameChange(drawn: DrawnTextRecord, sized: SizedText): Vector {
  const before = textFrame(drawn.text, drawn.block);
  const after = drawn.text.angle === sized.text.angle ? textFrame(sized.text, sized.block) : before;

  return subtract({ x: after.width, y: after.height }, { x: before.width, y: before.height });
}

/**
 * Chooses the position of the text last drawn when nothing moves it: X and Y still show its
 * turned top-left corner, and its frame kept its size, or its angle changed.
 *
 * @kind procedure
 * @param drawn - the text last drawn
 * @param sized - the text read from the fields, its position the corner X and Y show
 * @returns that position, nothing when the text is placed from its corner
 * @see docs/backlog/stories/E15-F17-US-054-turned-text-position-shows-its-corner.md
 */
function chooseKeptPosition(drawn: DrawnTextRecord, sized: SizedText): Point | undefined {
  const isInPlace =
    isZeroVector(subtract(sized.text, readTurnedCorner(drawn))) &&
    isZeroVector(readFrameChange(drawn, sized));

  return isInPlace ? { x: drawn.text.x, y: drawn.text.y } : undefined;
}

/**
 * Chooses where a text is drawn from the turned top-left corner X and Y show (US-054): a corner
 * typed, a first text, or a frame that changed size, drawn with its turned top-left corner at
 * that point (`positionKeepingPoint`, `DERIV-turned-shape` step 5), so that a turned text that
 * grows keeps its corner (US-038, ADR-0057); an angle changed, or nothing that moves it, at the
 * position of the text last drawn (`chooseKeptPosition`), so that a text turns about its middle
 * (ADR-0055). At 0° the corner is the position, bit for bit.
 *
 * @kind procedure
 * @param drawn - the text last drawn, nothing before the first
 * @param sized - the text read from the fields, its position the corner X and Y show, and its
 * measured block
 * @returns the text at the position it is drawn at, the top-left of its frame before it turns
 * @see docs/backlog/stories/E15-F17-US-054-turned-text-position-shows-its-corner.md
 */
export function choosePlacement(drawn: DrawnTextRecord | undefined, sized: SizedText): Text {
  const kept = drawn === undefined ? undefined : chooseKeptPosition(drawn, sized);
  const { height, width } = textFrame(sized.text, sized.block);

  return {
    ...sized.text,
    ...(kept ??
      positionKeepingPoint({ angle: sized.text.angle, height, width }, TOP_LEFT, sized.text)),
  };
}

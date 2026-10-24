// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { contourBandWithin, contourPiecesToPathData, shapePickArea } from "../../src";
import { openServedPage, readShapeModel, setElementBox, typeAndEnter } from "../user-actions";
import { PAGE_VIEWS } from "../view";

import { HELPS_FRAMES } from "./helps-frames";
import { readBandPathData } from "./read-band-path-data";
import { startHelpsFrame } from "./start-helps-frame";

import type { BandSize } from "../../src";

// The page's outlines no longer read the frame (US-086): its tests on the page are the strokes'
// (`helps-on-any-color.test.ts`); the frame itself goes with US-086's T3.

/** The band at 100 %: 2 pixels wide, its chords at most 2 pixels. */
const AT_100: BandSize = { halfWidth: 1, step: 2 };

describe("the helps' frame, kept while a pan stays within its box (EN-149)", () => {
  beforeEach(() => {
    openServedPage("");
    setElementBox("#canvas", { height: 800, left: 100, top: 50, width: 1200 });
    typeAndEnter("radius", "0");
    typeAndEnter("width", "2000");
    typeAndEnter("height", "2000");
    typeAndEnter("shape-x", "0");
    typeAndEnter("shape-y", "0");
    typeAndEnter("radius", "1000");
  });

  it("reuses a band drawn in the same box rather than computing it again", () => {
    const { pieces } = shapePickArea(readShapeModel());

    // A band read once, then replaced by a mark no band has: a band read again gives the mark.
    readBandPathData(document, pieces, AT_100);

    const frame = HELPS_FRAMES.get(document) ?? startHelpsFrame(document);

    HELPS_FRAMES.set(document, {
      ...frame,
      bands: new Map(frame.bands.keys().map((key): [string, string] => [key, "M0 0Z"])),
    });

    expect(readBandPathData(document, pieces, AT_100)).toBe("M0 0Z");

    // The next redraw, 100 pixels right, keeps the box and the band drawn before it.
    PAGE_VIEWS.set(document, { origin: { x: 60, y: -40 }, scale: 1 });
    startHelpsFrame(document);
    expect(readBandPathData(document, pieces, AT_100)).toBe("M0 0Z");

    // 700 pixels right, a new box from (60, −440) to (2 460, 1 160): computed again.
    PAGE_VIEWS.set(document, { origin: { x: 660, y: -40 }, scale: 1 });
    startHelpsFrame(document);

    expect(readBandPathData(document, pieces, AT_100)).toBe(
      contourBandWithin(pieces, AT_100, { maxX: 2460, maxY: 1160, minX: 60, minY: -440 })
        .map((subpath) => contourPiecesToPathData(subpath))
        .join(" "),
    );
  });
});

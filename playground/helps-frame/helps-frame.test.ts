// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { contourBand, contourBandWithin, contourPiecesToPathData, shapePickArea } from "../../src";
import { showHover } from "../hover";
import {
  openServedPage,
  readShapeModel,
  runResizeObservers,
  selectPageElement,
  setElementBox,
  typeAndEnter,
  typeValue,
} from "../user-actions";
import { PAGE_VIEWS } from "../view";

import { HELPS_FRAMES } from "./helps-frames";
import { readBandPathData } from "./read-band-path-data";
import { startHelpsFrame } from "./start-helps-frame";

import type { BandSize, BoundingBox, ContourPiece, Point, View } from "../../src";

// The canvas sits at (100, 50) in the window, 1 200 × 800: the offset (x, y) on the canvas is the
// window's (x + 100, y + 50), over the drawing's point origin + (x, y) / scale. The opening view,
// 100 % with (−40, −40) at the top-left, computes the helps within (−640, −440) to (1 760, 1 160),
// the area shown widened by half the canvas on each side (SP-030's answer 7).

/** The band of the outlines at 100 %: 2 pixels wide, its chords at most 2 pixels. */
const AT_100: BandSize = { halfWidth: 1, step: 2 };

/**
 * Shows a view, as the page would after a zoom or a pan: kept, then the page redrawn as the canvas
 * resized.
 *
 * @param view - the view shown
 */
function showView(view: View): void {
  PAGE_VIEWS.set(document, view);
  runResizeObservers("#canvas");
}

/**
 * Types the rectangle the panel edits at (0, 0), its radius 0 first so that no large rounded
 * shape is drawn on the way.
 *
 * @param width - its width
 * @param height - its height
 * @param radius - its corners' radius
 */
function typeRounded(width: string, height: string, radius: string): void {
  typeValue("angle", "0");
  typeValue("radius", "0");
  typeAndEnter("width", width);
  typeAndEnter("height", height);
  typeAndEnter("shape-x", "0");
  typeAndEnter("shape-y", "0");
  typeValue("radius", radius);
}

/**
 * Reads the path data of an outline of the layer above the drawing.
 *
 * @param name - its class, `selection` or `hover`
 * @returns its path data; undefined for none
 */
function readOutline(name: string): string | undefined {
  return document.querySelector(`#overlay svg path.${name}`)?.getAttribute("d") ?? undefined;
}

/**
 * The path data of a band about a contour within a box, as the outlines are drawn.
 *
 * @param pieces - the contour
 * @param size - the band's half width and chord
 * @param box - the box it is computed within
 * @returns its subpaths' data joined
 */
function bandWithin(pieces: readonly ContourPiece[], size: BandSize, box: BoundingBox): string {
  return contourBandWithin(pieces, size, box)
    .map((subpath) => contourPiecesToPathData(subpath))
    .join(" ");
}

/**
 * The box from one corner to the other.
 *
 * @param from - its top-left corner
 * @param to - its bottom-right corner
 * @returns its extreme coordinates
 */
function boxOf(from: Point, to: Point): BoundingBox {
  return { maxX: to.x, maxY: to.y, minX: from.x, minY: from.y };
}

describe("R's band within the area shown and half the canvas (EN-149)", () => {
  beforeEach(() => {
    openServedPage("");
    setElementBox("#canvas", { height: 800, left: 100, top: 50, width: 1200 });
    typeRounded("99999", "100000", "100000");
  });

  it(
    "[F42.AC5] draws R's band empty at 100 %, 60 redraws of the layer above within a second",
    // R's corners, about 39 270 chords of 2 pixels each, took 156 to 200 ms a band (note 0031, §5):
    // each redraw computes it again, its frame dropped. alone: 88 ms, the median of 5 runs of its
    // file (EN-149); 960 ms, 60 frames of 16 ms, is 10.9 times it.
    { timeout: 960 },
    () => {
      // Selected, its contour 18 647 units from the helps' box: no subpath.
      expect(readOutline("selection")).toBe("");

      const redraws = Array.from({ length: 60 }, () => document);

      for (const page of redraws) {
        HELPS_FRAMES.delete(page);
        showHover(page);
      }

      expect(readOutline("selection")).toBe("");
    },
  );

  it("follows R fitted at 1 % whole, its corners by 393 chords each", () => {
    // Fitted with 40 pixels on each side: min(1 120 / 99 999, 720 / 100 000) = 0.0072, under the
    // bound, so 0.01; its centre (49 999.5, 50 000) at the canvas's (600, 400): o = (49 999.5 − 600
    // / 0.01, 50 000 − 400 / 0.01) = (−10 000.5, 10 000); the helps' box, from (−70 000.5,
    // −30 000) to (169 999.5, 130 000), holds R.
    showView({ origin: { x: -10_000.5, y: 10_000 }, scale: 0.01 });

    const { pieces } = shapePickArea(readShapeModel());
    const whole = contourBand(pieces, 1 / 0.01, 2 / 0.01);

    expect(readOutline("selection")).toBe(
      whole.map((subpath) => contourPiecesToPathData(subpath)).join(" "),
    );

    // Its contour holds its 4 corners and its 2 sides 1 long, those 0 long left out: 6 discs,
    // 4 × 393 chords and one a side.
    expect(pieces.map((piece) => piece.kind)).toEqual([
      "arc",
      "segment",
      "arc",
      "arc",
      "segment",
      "arc",
    ]);

    expect(whole).toHaveLength(6 + 4 * 393 + 2);
  });

  it("[F42.AC5] still finds R under the pointer on its whole contour, its band empty there", () => {
    showView({ origin: { x: 49_000, y: 49_000 }, scale: 1 });

    // Over the offset (100, 100), R's point (49 100, 49 100), deep inside it: R is under the
    // pointer, its contour 48 000 units and more away, its band empty.
    selectPageElement("#canvas").dispatchEvent(
      new PointerEvent("pointermove", { bubbles: true, clientX: 200, clientY: 150, pointerId: 1 }),
    );

    expect(readOutline("hover")).toBe("");
  });
});

describe("the helps kept while a pan stays within their box (EN-149)", () => {
  beforeEach(() => {
    openServedPage("");
    setElementBox("#canvas", { height: 800, left: 100, top: 50, width: 1200 });
  });

  it("keeps the helps of a circle while a pan stays within 600 pixels, computes them again beyond", () => {
    // The circle 2 000 across at (0, 0): its band cut at x = 1 761 and y = 1 161 at 100 %.
    typeRounded("2000", "2000", "1000");

    const { pieces } = shapePickArea(readShapeModel());
    const opening = bandWithin(pieces, AT_100, boxOf({ x: -640, y: -440 }, { x: 1760, y: 1160 }));

    expect(readOutline("selection")).toBe(opening);

    // 100 pixels right: (60, −40) to (1 260, 760) shown, within the box: only the viewBox changes,
    // not the band of the box (−540, −440) to (1 860, 1 160) a new frame would cut.
    showView({ origin: { x: 60, y: -40 }, scale: 1 });
    expect(readOutline("selection")).toBe(opening);

    expect(opening).not.toBe(
      bandWithin(pieces, AT_100, boxOf({ x: -540, y: -440 }, { x: 1860, y: 1160 })),
    );

    expect(selectPageElement("#overlay svg").getAttribute("viewBox")).toBe("60 -40 1200 800");

    // 700 pixels right: (660, −40) to (1 860, 760), beyond 1 760: computed again within (60, −440)
    // to (2 460, 1 160).
    showView({ origin: { x: 660, y: -40 }, scale: 1 });

    expect(readOutline("selection")).toBe(
      bandWithin(pieces, AT_100, boxOf({ x: 60, y: -440 }, { x: 2460, y: 1160 })),
    );
  });

  it("keeps the helps at a pan of 600 pixels, computes them again at 601 (AUD-044)", () => {
    typeRounded("2000", "2000", "1000");

    const { pieces } = shapePickArea(readShapeModel());
    const opening = bandWithin(pieces, AT_100, boxOf({ x: -640, y: -440 }, { x: 1760, y: 1160 }));

    // 600 pixels right, |d_x| ≤ 600: (560, −40) to (1 760, 760) shown, on the box's edge, kept;
    // a new frame would cut within (−40, −440) to (2 360, 1 160).
    showView({ origin: { x: 560, y: -40 }, scale: 1 });
    expect(readOutline("selection")).toBe(opening);

    expect(opening).not.toBe(
      bandWithin(pieces, AT_100, boxOf({ x: -40, y: -440 }, { x: 2360, y: 1160 })),
    );

    // 601 pixels right: (561, −40) to (1 761, 760), past 1 760: computed again within (−39, −440)
    // to (2 361, 1 160).
    showView({ origin: { x: 561, y: -40 }, scale: 1 });

    expect(readOutline("selection")).toBe(
      bandWithin(pieces, AT_100, boxOf({ x: -39, y: -440 }, { x: 2361, y: 1160 })),
    );
  });

  it("computes the band again when the drawing changes, within the box kept", () => {
    typeRounded("2000", "2000", "1000");
    showView({ origin: { x: 60, y: -40 }, scale: 1 });

    // The circle moved to (10, 0): its band computed again, within the box of the opening view.
    typeAndEnter("shape-x", "10");

    const { pieces } = shapePickArea(readShapeModel());

    expect(readOutline("selection")).toBe(
      bandWithin(pieces, AT_100, boxOf({ x: -640, y: -440 }, { x: 1760, y: 1160 })),
    );
  });

  it("computes the band again at another scale", () => {
    typeRounded("2000", "2000", "1000");

    // At 200 % from (180, 130): within (−120, −70) to (1 080, 730), half width 0.5, chords 1.
    showView({ origin: { x: 180, y: 130 }, scale: 2 });

    const { pieces } = shapePickArea(readShapeModel());

    expect(readOutline("selection")).toBe(
      bandWithin(
        pieces,
        { halfWidth: 0.5, step: 1 },
        boxOf({ x: -120, y: -70 }, { x: 1080, y: 730 }),
      ),
    );
  });

  it("reuses a band drawn in the same box rather than computing it again", () => {
    typeRounded("2000", "2000", "1000");

    const { pieces } = shapePickArea(readShapeModel());
    const frame = HELPS_FRAMES.get(document);

    // Each band kept by the redraw replaced by a mark no band has: a band read again gives the mark.
    HELPS_FRAMES.set(document, {
      ...(frame ?? startHelpsFrame(document)),
      bands: new Map([...(frame?.bands.keys() ?? [])].map((key) => [key, "M0 0Z"])),
    });

    expect(readBandPathData(document, pieces, AT_100)).toBe("M0 0Z");

    // The next redraw, 100 pixels right, keeps the box and the band drawn before it.
    PAGE_VIEWS.set(document, { origin: { x: 60, y: -40 }, scale: 1 });
    startHelpsFrame(document);
    expect(readBandPathData(document, pieces, AT_100)).toBe("M0 0Z");

    // 700 pixels right, a new box: computed again.
    PAGE_VIEWS.set(document, { origin: { x: 660, y: -40 }, scale: 1 });
    startHelpsFrame(document);

    expect(readBandPathData(document, pieces, AT_100)).toBe(
      bandWithin(pieces, AT_100, boxOf({ x: 60, y: -440 }, { x: 2460, y: 1160 })),
    );
  });
});

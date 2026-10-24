// @vitest-environment happy-dom

import { beforeEach, describe, expect, it, vi } from "vitest";

import { measuredTextDrawing, textDrawing, textFontMetrics, textMeasure } from "../../src";
import {
  chooseOption,
  chooseShapeKind,
  clickElement,
  openServedPage,
  typeAndEnter,
} from "../user-actions";

/** The library, its measuring functions typed: the others are spread as they are. */
type MeasuringFunctions = Readonly<Record<string, unknown>> & {
  /** Draws a text from what it measures. */
  readonly measuredTextDrawing: typeof measuredTextDrawing;
  /** Measures and draws a text. */
  readonly textDrawing: typeof textDrawing;
  /** The metrics of a text's face. */
  readonly textFontMetrics: typeof textFontMetrics;
  /** A text's lines and block. */
  readonly textMeasure: typeof textMeasure;
};

/** The animation frames asked and not run yet, as the browser holds them until its next repaint. */
const FRAMES: FrameRequestCallback[] = [];

/**
 * Runs the frames asked, as the browser does before its next repaint.
 */
function runFrames(): void {
  for (const frame of FRAMES.splice(0)) {
    frame(0);
  }
}

// The library's measuring functions, each wrapped in a spy that counts its calls from the page.
vi.mock("../../src", async (importOriginal) => {
  const library = await importOriginal<MeasuringFunctions>();

  return {
    ...library,
    measuredTextDrawing: vi.fn(library.measuredTextDrawing),
    textDrawing: vi.fn(library.textDrawing),
    textFontMetrics: vi.fn(library.textFontMetrics),
    textMeasure: vi.fn(library.textMeasure),
  };
});

/**
 * Counts the calls of each measuring function since the counts were cleared.
 *
 * @returns the calls of the metrics, the measure (lines and block), the drawing, and the whole drawing
 */
function readCounts(): Record<string, number> {
  return {
    drawing: vi.mocked(measuredTextDrawing).mock.calls.length,
    measure: vi.mocked(textMeasure).mock.calls.length,
    metrics: vi.mocked(textFontMetrics).mock.calls.length,
    wholeDrawing: vi.mocked(textDrawing).mock.calls.length,
  };
}

describe("What a redraw computes again (US-048)", () => {
  beforeEach(() => {
    openServedPage("");
    chooseShapeKind("text");
    typeAndEnter("text-size", "16");

    // From here, the frames wait for the test to run them, as in a browser.
    Object.defineProperty(document.defaultView ?? {}, "requestAnimationFrame", {
      configurable: true,
      value: (callback: FrameRequestCallback): number => {
        FRAMES.push(callback);

        return FRAMES.length;
      },
    });

    vi.clearAllMocks();
  });

  it("[F17.AC5] an opacity changed: the metrics, lines and block kept, the text drawn once", () => {
    typeAndEnter("text-opacity", "50");
    runFrames();

    expect(readCounts()).toEqual({ drawing: 1, measure: 0, metrics: 0, wholeDrawing: 0 });
  });

  it("[F17.AC5] X changed: the lines and block kept, the text drawn once where it moved", () => {
    typeAndEnter("shape-x", "120");
    runFrames();

    expect(readCounts()).toEqual({ drawing: 1, measure: 0, metrics: 0, wholeDrawing: 0 });
  });

  it("[F17.AC5] a size changed: the metrics kept, the lines and block measured again", () => {
    typeAndEnter("text-size", "20");
    runFrames();

    expect(readCounts()).toEqual({ drawing: 1, measure: 1, metrics: 0, wholeDrawing: 0 });
  });

  it("[F17.AC5] a weight changed: the metrics read again, then the lines and block", () => {
    chooseOption("text-weight", "700");
    runFrames();

    expect(readCounts()).toEqual({ drawing: 1, measure: 1, metrics: 1, wholeDrawing: 0 });
  });

  it("[F17.AC5] italic turned on: the metrics read again, then the lines and block", () => {
    clickElement("text-italic");
    runFrames();

    expect(readCounts()).toEqual({ drawing: 1, measure: 1, metrics: 1, wholeDrawing: 0 });
  });
});

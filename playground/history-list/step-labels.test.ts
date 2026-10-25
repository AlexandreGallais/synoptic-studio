// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { FIELD_LABELS } from "../../src";
import {
  chooseOption,
  chooseShapeKind,
  clickElement,
  dispatchDrawingPointer,
  openPage,
  setElementBox,
  typeAndEnter,
  typeTextContent,
} from "../user-actions";

import type { Point } from "../../src";
import type { PointerKeys } from "../user-actions";

/** No key held. */
const NO_KEY: PointerKeys = { altKey: false, shiftKey: false };

/** Inside A, the rectangle the page opens with, 120 × 80 at (0, 0). */
const IN_A: Point = { x: 60, y: 40 };

/** A unit or a hint in brackets after a label, `Max lines (0 none)`, left out of a step's name. */
const BRACKETS = / \(.*\)$/u;

/**
 * Clicks the drawing as a mouse does: pressed, released, then the `click`.
 *
 * @param at - the point clicked
 */
function clickDrawing(at: Point): void {
  dispatchDrawingPointer("pointerdown", at, NO_KEY);
  dispatchDrawingPointer("pointerup", at, NO_KEY);
  dispatchDrawingPointer("click", at, NO_KEY);
}

/**
 * Reads the name of the last step of the list, the present's.
 *
 * @returns its button's text
 */
function readLastStep(): string {
  return [...document.querySelectorAll("#history-list button")].at(-1)?.textContent ?? "";
}

/**
 * Reads the labels of the panel's fields and buttons as a step names them: each one's own text,
 * its spaces collapsed, a unit or a hint in brackets left out.
 *
 * @returns each label
 */
function readPanelLabels(): ReadonlySet<string> {
  const labels = [...document.querySelectorAll("#panel label, #panel legend, #panel button")];

  return new Set(
    labels.map((label) =>
      [...label.childNodes]
        .filter((node) => node.nodeType === Node.TEXT_NODE)
        .map((node) => node.textContent)
        .join(" ")
        .replaceAll(/\s+/gu, " ")
        .trim()
        .replace(BRACKETS, ""),
    ),
  );
}

describe("The steps named by the panel's labels", () => {
  beforeEach(() => {
    openPage("");
    setElementBox("#canvas", { height: 600, left: 100, top: 50, width: 800 });
    clickDrawing(IN_A);
  });

  it("[F41.AC1] names each field of FIELD_LABELS by a label the panel shows", () => {
    const labels = readPanelLabels();

    expect(
      FIELD_LABELS.values()
        .filter((label) => !labels.has(label))
        .toArray(),
    ).toEqual([]);
  });

  it("[F41.AC1] names a rectangle's radius typed 'Radius', its corners changed", () => {
    typeAndEnter("radius", "5");

    expect(readLastStep()).toBe("Radius");
  });

  it("[F41.AC1] names a polygon's radius 'Radius' and Stretched 'Stretched to its box'", () => {
    chooseShapeKind("polygon");
    typeAndEnter("radius", "5");

    expect(readLastStep()).toBe("Radius");

    clickElement("stretched");

    expect(readLastStep()).toBe("Stretched to its box");
  });

  it("[F41.AC1] names a text's Italic and Strike-through by their buttons' labels", () => {
    // happy-dom does not select the option marked `selected`: the page's weight, 400, chosen again.
    chooseOption("text-weight", "400");
    clickElement("tool-text");
    clickDrawing({ x: 400, y: 300 });
    typeTextContent("Pump");
    clickElement("text-italic");

    expect(readLastStep()).toBe("Italic");

    clickElement("text-strike");

    expect(readLastStep()).toBe("Strike-through");
  });
});

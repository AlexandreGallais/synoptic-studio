// @vitest-environment happy-dom

import { afterEach, describe, expect, it, vi } from "vitest";

import { DEFAULT_FONT, lineWidth } from "../../src";
import { testFaceFile } from "../../tests";
import {
  chooseFiles,
  clickElement,
  openServedPage,
  readText,
  selectChoice,
  waitForPageTasks,
} from "../user-actions";

import { DEFAULT_FONT_MISSING } from "./default-font-missing";

/** The probe line the page measures in each of Inter's faces. */
const PROBE = "Pump P-101 Running";

/**
 * The probe's width from the metrics of one of Inter's shipped faces, at size 1000: what a
 * browser drawing that face measures.
 *
 * @param weight - the face's weight
 * @param isItalic - whether it is italic
 * @returns its width
 */
function shippedWidth(weight: number, isItalic: boolean): number {
  const face = DEFAULT_FONT.faces.find(
    (shipped) => shipped.weight === weight && shipped.isItalic === isItalic,
  );

  return face === undefined
    ? NaN
    : lineWidth(face.metrics, PROBE, { letterSpacing: 0, size: 1000, wordSpacing: 0 });
}

/**
 * The width a browser drawing Inter's upright faces listed measures for a line: the face's own,
 * shifted by its gap; any other face 50 wider, as another font.
 *
 * @param font - the font the canvas was given
 * @param gaps - the gap of each upright weight the browser draws with Inter
 * @returns the probe's width; 0 for another line
 */
function fakeProbeWidth(font: string, gaps: Readonly<Record<number, number>>): number {
  // Such as `italic 700 1000px "Inter", sans-serif`: the style, then the weight.
  const isItalic = font.startsWith("italic ");
  const [weight = ""] = font.replace("italic ", "").split(" ", 1);
  const gap = isItalic ? undefined : gaps[Number(weight)];

  return shippedWidth(Number(weight), isItalic) + (gap ?? 50);
}

/**
 * Makes the browser measure on a canvas (`fakeProbeWidth`).
 *
 * @param gaps - the gap of each upright weight the browser draws with Inter
 * @returns the fonts the page gave the canvas, in order
 */
function fakeCanvas(gaps: Readonly<Record<number, number>>): readonly string[] {
  const asked: string[] = [];
  const context = {
    font: "",
    measureText: (text: string): { readonly width: number } => {
      asked.push(context.font);

      return { width: text === PROBE ? fakeProbeWidth(context.font, gaps) : 0 };
    },
  };

  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- happy-dom has no canvas: the fake holds only what the page reads, `font` and `measureText`
  vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(context as never);

  return asked;
}

/**
 * Reads the weights the weight list offers.
 *
 * @returns their values, in order
 */
function readWeights(): readonly string[] {
  return [...selectChoice("text-weight").options].map((option) => option.value);
}

/**
 * Opens the page on the Text tool.
 */
function openTextTool(): void {
  openServedPage("");
  clickElement("tool-text");
}

describe("Inter's faces kept only when the browser draws them (US-053)", () => {
  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("[F17.AC11] keeps Inter's nine weights when the browser cannot measure on a canvas", () => {
    openTextTool();

    expect(readWeights()).toEqual(["100", "200", "300", "400", "500", "600", "700", "800", "900"]);
  });

  it("[F17.AC11] offers only the weights the browser draws as Inter's metrics measure them, 400 and 700", () => {
    fakeCanvas({ 400: 0, 700: 0 });
    openTextTool();

    expect(readWeights()).toEqual(["400", "700"]);
    expect(document.querySelector("#canvas text")).not.toBeNull();
  });

  it("[F17.AC11] asks the canvas each of Inter's 18 faces, named as the drawing names its font", () => {
    const asked = fakeCanvas({});

    openTextTool();

    expect(asked.slice(0, 18)).toHaveLength(18);
    expect(asked).toContain('400 1000px "Inter", sans-serif');
    expect(asked).toContain('italic 900 1000px "Inter", sans-serif');
  });

  it("[F17.AC11] keeps a face drawn 9 units off at size 1000, half a unit per character of the probe, not one 9.5 off", () => {
    fakeCanvas({ 400: 9, 500: -9, 700: 9.5 });
    openTextTool();

    expect(readWeights()).toEqual(["400", "500"]);
  });

  it("[F17.AC11] says Inter is missing and draws no text when the browser draws none of its faces", () => {
    fakeCanvas({});
    openTextTool();

    expect(readText("#font-message")).toBe(DEFAULT_FONT_MISSING);
    expect(readText("#status")).toBe(DEFAULT_FONT_MISSING);
    expect(document.querySelector("#canvas text")).toBeNull();
    expect(document.querySelector("#text-size")?.getAttribute("aria-invalid")).toBe("false");
  });

  it("[F17.AC11] offers Inter's faces read from its files once imported, Inter missing before", async () => {
    fakeCanvas({});
    openTextTool();

    const bytes = testFaceFile({ family: "Inter", fsType: 0, isItalic: false, weightClass: 400 });

    chooseFiles("font-file", [new File([new Uint8Array(bytes)], "Inter-Regular.ttf")]);

    await waitForPageTasks();

    expect(readWeights()).toEqual(["400"]);

    expect(document.querySelector("#canvas text")?.getAttribute("font-family")).toBe(
      '"Inter", sans-serif',
    );
  });
});

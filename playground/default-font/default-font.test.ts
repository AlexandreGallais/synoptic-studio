// @vitest-environment happy-dom

import { afterEach, describe, expect, it, vi } from "vitest";

import { DEFAULT_FONT, lineWidth } from "../../src";
import { lettersMissingFaceFile, testFaceFile } from "../../tests";
import {
  chooseFiles,
  clickElement,
  openServedPage,
  readDisabledState,
  readText,
  readValue,
  selectChoice,
  selectInput,
  waitForPageTasks,
} from "../user-actions";

import { DEFAULT_FONT_MISSING } from "./default-font-missing";
import { NO_FONT_APPROVED } from "./no-font-approved";

import type { TestFace } from "../../tests";
import type { InstalledFont } from "../fonts";

/** The test font named Test Sans, regular, embeddable. */
const TEST_SANS: TestFace = { family: "Test Sans", fsType: 0, isItalic: false, weightClass: 400 };

/** Said once the browser listed the three installed families of a test. */
const LISTED = "Installed families listed: 3. The font list can be used once they are read.";

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
 * Reads the families the font list offers.
 *
 * @returns their values, in order
 */
function readFamilies(): readonly string[] {
  return [...selectChoice("text-font").options].map((option) => option.value);
}

/**
 * The file of a regular face of the test font, as the browser gives it.
 *
 * @param family - the family its `name` table gives
 * @returns a `Blob` of its bytes
 */
function testFaceBlob(family: string): Blob {
  const bytes = testFaceFile({ ...TEST_SANS, family });

  return new Blob([new Uint8Array(bytes)]);
}

/**
 * An installed family as the browser lists it, one face of the test font, regular, embeddable,
 * every Latin letter mapped.
 *
 * @param family - its name
 * @param file - its file, once the browser gives it
 * @returns its face
 */
function installedFamily(family: string, file: Promise<Blob>): InstalledFont {
  return {
    blob: vi.fn<() => Promise<Blob>>(async () => await file),
    family,
    fullName: family,
    postscriptName: family,
  };
}

/**
 * An installed family as the browser lists it, one face of the test font, its file given at once
 * (`installedFamily`).
 *
 * @param family - its name
 * @returns its face
 */
function installedTestFamily(family: string): InstalledFont {
  return installedFamily(family, Promise.resolve(testFaceBlob(family)));
}

/**
 * Makes the browser list installed faces, as Chrome does once the permission is granted.
 *
 * @param fonts - the faces, in the browser's order
 */
function fakeInstalledFonts(fonts: readonly InstalledFont[]): void {
  const query = vi.fn<() => Promise<readonly InstalledFont[]>>().mockResolvedValue(fonts);

  Object.defineProperty(globalThis, "queryLocalFonts", { configurable: true, value: query });
}

/**
 * Makes the browser list installed families, each one face of the test font (`installedFamily`).
 *
 * @param families - their names, in the browser's order
 */
function fakeLocalFonts(families: readonly string[]): void {
  fakeInstalledFonts(
    families.map((family) => installedFamily(family, Promise.resolve(testFaceBlob(family)))),
  );
}

/**
 * Ticks Embed fonts, as the designer would: its `change` event.
 */
function tickEmbedFonts(): void {
  const box = selectInput("embed-fonts");

  box.checked = true;
  box.dispatchEvent(new Event("change", { bubbles: true }));
}

/**
 * Reads the faces the page lists as not embedded.
 *
 * @returns one line per face
 */
function readLeftOut(): readonly (string | null)[] {
  return [...document.querySelectorAll("#font-left-out li")].map((item) => item.textContent);
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
    Reflect.deleteProperty(globalThis, "queryLocalFonts");
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

  it("[F17.AC14] puts Installed fonts before the font list, under the line saying to start with it", () => {
    openTextTool();

    // querySelectorAll gives the elements in the page's order.
    const order = [...document.querySelectorAll("#installed-fonts, #text-font")].map(
      (element) => element.id,
    );

    expect(order).toEqual(["installed-fonts", "text-font"]);

    expect(readText("#font-first").trim()).toBe(
      "Start with Installed fonts: the font list offers only the fonts read.",
    );
  });

  it("[F17.AC14] lists no font while Inter is not drawn and no installed font is read", () => {
    fakeCanvas({});
    openTextTool();

    expect(readFamilies()).toEqual([]);
  });

  it("[F17.AC14] keeps Inter first in the list when the browser draws it", () => {
    fakeCanvas({ 400: 0 });
    openTextTool();

    expect(readFamilies()).toEqual(["Inter"]);
  });

  it("[F17.AC14] chooses the first family read when Inter is not drawn, and draws the text in it", async () => {
    fakeCanvas({});
    fakeLocalFonts(["Test Sans", "Test Serif"]);
    openTextTool();

    clickElement("installed-fonts");
    await waitForPageTasks();

    expect(readFamilies()).toEqual(["Test Sans", "Test Serif"]);
    expect(readValue("text-font")).toBe("Test Sans");

    expect(readText("#font-message")).toBe(
      "Test Sans is chosen, the first font read: Inter is not installed on this computer.",
    );

    expect(document.querySelector("#canvas text")?.getAttribute("font-family")).toBe(
      '"Test Sans", sans-serif',
    );
  });

  it("[F17.AC17] lists only families read and approved, the list and Installed fonts disabled while they are read", async () => {
    const sans = Promise.withResolvers<Blob>();
    const serif = Promise.withResolvers<Blob>();
    const emoji = new Blob([
      new Uint8Array(lettersMissingFaceFile({ ...TEST_SANS, family: "Test Emoji" })),
    ]);

    fakeCanvas({});

    fakeInstalledFonts([
      installedFamily("Test Sans", sans.promise),
      installedFamily("Test Emoji", Promise.resolve(emoji)),
      installedFamily("Test Serif", serif.promise),
    ]);

    openTextTool();
    clickElement("installed-fonts");

    await vi.waitFor(() => {
      expect(readText("#font-message")).toBe(LISTED);
    });

    // Listed, none read: no family in the list, not even for a moment.
    expect(readFamilies()).toEqual([]);
    expect(readDisabledState("text-font")).toBe("true");
    expect(readDisabledState("installed-fonts")).toBe("true");

    sans.resolve(testFaceBlob("Test Sans"));

    await vi.waitFor(() => {
      expect(readText("#installed-fonts-progress")).toBe(
        "Reading the installed fonts: 2 of 3 families…",
      );
    });

    // Test Sans read and approved, the only family listed; Test Emoji refused; Test Serif not read yet.
    expect(readFamilies()).toEqual(["Test Sans"]);
    expect(readValue("text-font")).toBe("Test Sans");
    expect(readDisabledState("text-font")).toBe("true");
    expect(readDisabledState("installed-fonts")).toBe("true");

    serif.resolve(testFaceBlob("Test Serif"));
    await waitForPageTasks();

    expect(readFamilies()).toEqual(["Test Sans", "Test Serif"]);
    expect(readDisabledState("text-font")).toBe("false");
    expect(readDisabledState("installed-fonts")).toBe("false");
    expect(readValue("text-font")).toBe("Test Sans");
    expect(document.querySelector("#canvas text")).not.toBeNull();
  });

  it("[F17.AC17] offers no text to write while no font is available, and gives it once one is", async () => {
    fakeCanvas({});
    fakeLocalFonts(["Test Sans"]);
    openTextTool();

    expect(readDisabledState("text-font")).toBe("true");
    expect(readDisabledState("text-content")).toBe("true");

    clickElement("installed-fonts");
    await waitForPageTasks();

    expect(readDisabledState("text-content")).toBe("false");
  });

  it("[F17.AC17] leaves the Content field enabled when the browser draws Inter", () => {
    fakeCanvas({ 400: 0 });
    openTextTool();

    expect(readDisabledState("text-font")).toBe("false");
    expect(readDisabledState("text-content")).toBe("false");
  });

  it("[F17.AC17] says that no installed font can be used when every one is refused", async () => {
    const emoji = new Blob([
      new Uint8Array(lettersMissingFaceFile({ ...TEST_SANS, family: "Test Emoji" })),
    ]);

    fakeCanvas({});

    fakeInstalledFonts([installedFamily("Test Emoji", Promise.resolve(emoji))]);

    openTextTool();
    clickElement("installed-fonts");
    await waitForPageTasks();

    expect(readFamilies()).toEqual([]);
    expect(readDisabledState("text-font")).toBe("true");
    expect(readDisabledState("text-content")).toBe("true");
    expect(readDisabledState("installed-fonts")).toBe("false");
    expect(readText("#font-message")).toBe(NO_FONT_APPROVED);
    expect(document.querySelector("#canvas text")).toBeNull();
  });

  it("[F17.AC14] leaves a family chosen before the installed fonts are read as it is", async () => {
    fakeCanvas({ 400: 0 });
    fakeLocalFonts(["Test Sans"]);
    openTextTool();

    clickElement("installed-fonts");
    await waitForPageTasks();

    expect(readFamilies()).toEqual(["Inter", "Test Sans"]);
    expect(readValue("text-font")).toBe("Inter");
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

describe("The font chosen automatically, its files held (US-060)", () => {
  afterEach(() => {
    Reflect.deleteProperty(globalThis, "queryLocalFonts");
    vi.restoreAllMocks();
  });

  it("[F17.AC18] chooses the first approved family once the reading ends, its files read again, so that it is embedded", async () => {
    const sans = installedTestFamily("Test Sans");

    fakeCanvas({});

    fakeInstalledFonts([sans, installedTestFamily("Test Serif")]);

    openTextTool();

    clickElement("installed-fonts");
    await waitForPageTasks();

    // Read ahead once, then read again when chosen: its file held.
    expect(sans.blob).toHaveBeenCalledTimes(2);
    expect(readValue("text-font")).toBe("Test Sans");

    tickEmbedFonts();

    expect(document.querySelector("#canvas text")).not.toBeNull();
    expect(readLeftOut()).toEqual([]);
  });

  it("[F17.AC18] chooses no font before the reading ends", async () => {
    const serif = Promise.withResolvers<Blob>();

    fakeCanvas({});

    fakeInstalledFonts([
      installedTestFamily("Test Sans"),
      installedFamily("Test Serif", serif.promise),
    ]);

    openTextTool();
    clickElement("installed-fonts");

    await vi.waitFor(() => {
      expect(readText("#installed-fonts-progress")).toBe(
        "Reading the installed fonts: 1 of 2 families…",
      );
    });

    // Test Sans read and approved, Test Serif not read yet: no font chosen, no text drawn.
    expect(readText("#font-message")).toBe(
      "Installed families listed: 2. The font list can be used once they are read.",
    );

    expect(document.querySelector("#canvas text")).toBeNull();

    serif.resolve(testFaceBlob("Test Serif"));
    await waitForPageTasks();

    expect(readText("#font-message")).toBe(
      "Test Sans is chosen, the first font read: Inter is not installed on this computer.",
    );
  });

  it("[F17.AC18] leaves out a family whose files are refused when read again, and chooses the next", async () => {
    const blob = vi
      .fn<() => Promise<Blob>>()
      .mockResolvedValueOnce(testFaceBlob("Test Sans"))
      .mockRejectedValue(new TypeError("The font Test Sans could not be accessed."));

    fakeCanvas({});

    fakeInstalledFonts([
      { blob, family: "Test Sans", fullName: "Test Sans", postscriptName: "Test Sans" },
      installedTestFamily("Test Serif"),
    ]);

    openTextTool();
    clickElement("installed-fonts");
    await waitForPageTasks();

    expect(readFamilies()).toEqual(["Test Serif"]);
    expect(readValue("text-font")).toBe("Test Serif");

    tickEmbedFonts();

    expect(document.querySelector("#canvas text")).not.toBeNull();
    expect(readLeftOut()).toEqual([]);
  });
});

describe("The font list held while the default font is read again (US-061)", () => {
  afterEach(() => {
    Reflect.deleteProperty(globalThis, "queryLocalFonts");
    vi.restoreAllMocks();
  });

  it("[F18.AC14] keeps the font list disabled while the default font's files are read again, said so", async () => {
    const again = Promise.withResolvers<Blob>();
    const blob = vi
      .fn<() => Promise<Blob>>()
      .mockResolvedValueOnce(testFaceBlob("Test Sans"))
      .mockReturnValueOnce(again.promise);

    fakeCanvas({});

    fakeInstalledFonts([
      { blob, family: "Test Sans", fullName: "Test Sans", postscriptName: "Test Sans" },
    ]);

    openTextTool();
    clickElement("installed-fonts");

    await vi.waitFor(() => {
      expect(readText("#installed-fonts-progress")).toBe("Installed fonts read: 1 families.");
    });

    // Every family read ahead, Test Sans's file read a second time and not given yet; the page's
    // redraws run at once in a test (`openServedPage`), so the list read is the one drawn.
    expect(readDisabledState("text-font")).toBe("true");
    expect(readDisabledState("installed-fonts")).toBe("true");

    expect(readText("#font-message")).toBe(
      "Reading Test Sans, the default font: the font list can be used once it is read.",
    );

    again.resolve(testFaceBlob("Test Sans"));
    await waitForPageTasks();

    expect(readDisabledState("text-font")).toBe("false");
    expect(readDisabledState("installed-fonts")).toBe("false");
    expect(readValue("text-font")).toBe("Test Sans");
  });
});

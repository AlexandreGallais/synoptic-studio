// @vitest-environment happy-dom

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { fontCollection, testFaceFile } from "../../tests";
import { chooseFiles, clickElement, openServedPage, waitForPageTasks } from "../user-actions";

import { addFontFile } from "./add-font-file";

/** A face the fake browser creates: what `load()` answers. */
type FakeFace = {
  /** Loads the face. */
  readonly load: () => Promise<FakeFace>;
};

/** The faces the fake page's font set holds. */
const addFace = vi.fn<(face: FakeFace) => void>();

/** The fake `FontFace` constructor, recording its arguments. */
const createdFaces = vi.fn<(family: string, source: Uint8Array, descriptors: object) => FakeFace>();

/**
 * Creates a face as the browser's `FontFace` constructor does, its load fulfilled or rejected.
 *
 * @param family - the family, as a CSS string
 * @param source - the file's bytes
 * @param descriptors - its style and weight
 * @returns the face, its `load()` recorded
 */
function createFakeFace(family: string, source: Uint8Array, descriptors: object): FakeFace {
  createdFaces(family, source, descriptors);

  const load = vi.fn<() => Promise<FakeFace>>();
  const face: FakeFace = { load };

  // A family named Broken stands for a face the browser cannot parse (REF-CSS-FONT-LOADING).
  if (family.includes("Broken")) {
    load.mockRejectedValue(new DOMException("The font could not be parsed.", "SyntaxError"));
  } else {
    load.mockResolvedValue(face);
  }

  return face;
}

describe("A font imported drawn by the page, CSS Font Loading's FontFace (US-036)", () => {
  beforeEach(() => {
    openServedPage("");
    clickElement("tool-text");
    Object.defineProperty(globalThis, "FontFace", { configurable: true, value: createFakeFace });
    Object.defineProperty(document, "fonts", { configurable: true, value: { add: addFace } });
  });

  afterEach(() => {
    Reflect.deleteProperty(globalThis, "FontFace");
    Reflect.deleteProperty(document, "fonts");
    vi.clearAllMocks();
  });

  it("[F15.AC1] gives the page the face, its family a CSS string, its weight and style, then redraws once loaded", async () => {
    const update = vi.fn<() => void>();
    const bytes = testFaceFile({
      family: "Test Sans",
      fsType: 0,
      isItalic: true,
      weightClass: 700,
    });

    addFontFile(document, bytes, update);
    expect(update).not.toHaveBeenCalled();
    await waitForPageTasks();

    expect(createdFaces).toHaveBeenCalledExactlyOnceWith('"Test Sans"', bytes, {
      style: "italic",
      weight: "700",
    });

    expect(addFace).toHaveBeenCalledOnce();
    expect(update).toHaveBeenCalledOnce();
  });

  it("[F15.AC1] keeps the face measured when the browser cannot load it", async () => {
    const update = vi.fn<() => void>();
    const bytes = testFaceFile({ family: "Broken", fsType: 0, isItalic: false, weightClass: 400 });

    expect(addFontFile(document, bytes, update)?.family).toBe("Broken");
    await waitForPageTasks();
    expect(addFace).toHaveBeenCalledOnce();
  });

  it("[F17.AC4] gives the page each face of a collection alone, as its own font file", async () => {
    const regular = testFaceFile({
      family: "Test Sans",
      fsType: 0,
      isItalic: false,
      weightClass: 400,
    });
    const bold = testFaceFile({
      family: "Test Sans",
      fsType: 0,
      isItalic: false,
      weightClass: 700,
    });

    const collection = new Uint8Array(fontCollection([regular, bold]));

    chooseFiles("font-file", [new File([collection], "TestSans.ttc")]);

    await waitForPageTasks();

    // Each face written alone gives back the face's own file, byte for byte, not the collection.
    expect(createdFaces).toHaveBeenCalledTimes(2);

    expect(createdFaces).toHaveBeenNthCalledWith(1, '"Test Sans"', regular, {
      style: "normal",
      weight: "400",
    });

    expect(createdFaces).toHaveBeenNthCalledWith(2, '"Test Sans"', bold, {
      style: "normal",
      weight: "700",
    });
  });
});

// @vitest-environment happy-dom

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { testFaceFile } from "../../tests";
import {
  chooseFiles,
  clickElement,
  openServedPage,
  selectChoice,
  typeValue,
  waitForPageTasks,
} from "../user-actions";

import type { InstalledFont } from "./installed-font";

/**
 * 60 installed faces of 20 families, three faces each, each file read ahead (US-053): the 590 faces
 * of 186 families seen by SP-019 are counted once each by `distinctFamilyNames`' own test; a shorter
 * list keeps this page test fast (EN-054).
 */
const INSTALLED: readonly InstalledFont[] = Array.from({ length: 60 }, (unused, index) => {
  const family = `Family ${String(index % 20)}`;
  const file = testFaceFile({ family, fsType: 0, isItalic: false, weightClass: 400 });

  return {
    blob: vi.fn<() => Promise<Blob>>().mockResolvedValue(new Blob([new Uint8Array(file)])),
    family,
    fullName: family,
    postscriptName: `${family}-${String(index)}`,
  };
});

/**
 * Adds an option the page never writes to the font list: gone once the list is written again.
 */
function markFontList(): void {
  selectChoice("text-font").append(
    Object.assign(document.createElement("option"), { value: "Mark" }),
  );
}

/**
 * Whether the font list still holds the mark.
 *
 * @returns `true` when the list was not written since it was marked
 */
function isFontListMarked(): boolean {
  return [...selectChoice("text-font").options].some((option) => option.value === "Mark");
}

describe("The font list written only when the session's fonts change (US-048)", () => {
  beforeEach(async () => {
    Object.defineProperty(globalThis, "queryLocalFonts", {
      configurable: true,
      value: vi.fn<() => Promise<readonly InstalledFont[]>>().mockResolvedValue(INSTALLED),
    });

    openServedPage("");
    clickElement("tool-text");
    clickElement("installed-fonts");
    await waitForPageTasks();
  });

  afterEach(() => {
    Reflect.deleteProperty(globalThis, "queryLocalFonts");
  });

  it("[F17.AC5] lists 20 families after Inter, each once", () => {
    expect(selectChoice("text-font").options).toHaveLength(21);
  });

  it("[F17.AC5] leaves the list as it is when the size, then the opacity changes", () => {
    markFontList();
    typeValue("text-size", "40");
    typeValue("text-opacity", "50");

    expect(isFontListMarked()).toBe(true);
  });

  it("[F17.AC5] writes the list again when a font is imported", async () => {
    const file = new File(
      [
        new Uint8Array(
          testFaceFile({ family: "Test Sans", fsType: 0, isItalic: false, weightClass: 400 }),
        ),
      ],
      "TestSans.ttf",
    );

    markFontList();
    chooseFiles("font-file", [file]);
    await waitForPageTasks();

    expect(isFontListMarked()).toBe(false);
    expect(selectChoice("text-font").options).toHaveLength(22);
  });
});

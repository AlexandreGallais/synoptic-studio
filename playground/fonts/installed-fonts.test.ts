// @vitest-environment happy-dom

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { testFaceFile } from "../../tests";
import {
  chooseOption,
  clickElement,
  openServedPage,
  readText,
  readValue,
  selectChoice,
  selectPageElement,
  typeValue,
} from "../user-actions";

import type { InstalledFont } from "./installed-font";
import type { TestFace } from "../../tests";

/** The test font named Test Sans, regular, installable. */
const TEST_SANS: TestFace = { family: "Test Sans", fsType: 0, isItalic: false, weightClass: 400 };

/** Said in Firefox and Safari, which cannot list the installed fonts. */
const UNSUPPORTED =
  "This browser cannot list the installed fonts (Chrome and Edge can): use Import a font with the font's file.";

/**
 * An installed face as the browser lists it, its file the bytes given.
 *
 * @param family - its family, as the browser names it
 * @param bytes - its file's bytes
 * @returns a `FontData` holding them
 */
function installedFont(family: string, bytes: Uint8Array): InstalledFont {
  return {
    blob: vi.fn<() => Promise<Blob>>().mockResolvedValue(new Blob([new Uint8Array(bytes)])),
    family,
    fullName: family,
  };
}

/**
 * Makes the browser list the installed fonts, as Chrome does once the permission is granted.
 *
 * @param fonts - the faces `queryLocalFonts` lists
 */
function fakeLocalFonts(fonts: readonly InstalledFont[]): void {
  const query = vi.fn<() => Promise<readonly InstalledFont[]>>().mockResolvedValue(fonts);

  Object.defineProperty(globalThis, "queryLocalFonts", { configurable: true, value: query });
}

/**
 * Waits for the page to say a line about the fonts.
 *
 * @param message - the line
 * @returns once the page says it
 */
async function waitForMessage(message: string): Promise<void> {
  await vi.waitFor(() => {
    expect(readText("#font-message")).toBe(message);
  });
}

/**
 * Reads the options of a list.
 *
 * @param id - the list's id
 * @returns their values, in order
 */
function readOptions(id: string): readonly string[] {
  return [...selectChoice(id).options].map((option) => option.value);
}

describe("Installed fonts (US-036)", () => {
  beforeEach(() => {
    openServedPage("");
    clickElement("tool-text");
    chooseOption("text-weight", "400");
    typeValue("text-size", "100");
  });

  afterEach(() => {
    Reflect.deleteProperty(globalThis, "queryLocalFonts");
  });

  it("[F15.AC1] says where the browser cannot list them, Firefox and Safari, and points to Import a font", () => {
    clickElement("installed-fonts");
    expect(readText("#font-message")).toBe(UNSUPPORTED);
    expect(readOptions("text-font")).toEqual(["Inter"]);
  });

  it("[F15.AC1] lists the installed families once the permission is granted, their files unread", async () => {
    const bold = vi.fn<() => Promise<Blob>>().mockResolvedValue(new Blob([]));

    fakeLocalFonts([
      installedFont("Test Sans", testFaceFile(TEST_SANS)),
      { blob: bold, family: "Test Sans", fullName: "Test Sans Bold" },
      installedFont("Test Serif", testFaceFile({ ...TEST_SANS, family: "Test Serif" })),
    ]);

    clickElement("installed-fonts");
    await waitForMessage("Installed families listed: 2. Choose one in the font list.");

    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans", "Test Serif"]);
    expect(readValue("text-font")).toBe("Inter");
    expect(bold).not.toHaveBeenCalled();
  });

  it("[F15.AC1] reads the faces of a family chosen and measures the text with them", async () => {
    fakeLocalFonts([
      installedFont("Test Sans", testFaceFile(TEST_SANS)),
      installedFont("Test Sans", testFaceFile({ ...TEST_SANS, weightClass: 700 })),
    ]);

    clickElement("installed-fonts");
    await waitForMessage("Installed families listed: 1. Choose one in the font list.");
    chooseOption("text-font", "Test Sans");
    await waitForMessage("Test Sans: 2 faces read.");

    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans"]);
    expect(readOptions("text-weight")).toEqual(["400", "700"]);

    // "B" at 100: 700 × 100 / 1000 = 70.
    const area = selectPageElement("#text-content");

    if (area instanceof HTMLTextAreaElement) {
      area.value = "B";
    }

    area.dispatchEvent(new Event("input", { bubbles: true }));
    expect(readText("#contour")).toContain('"width": 70');

    expect(selectPageElement("#canvas text").getAttribute("font-family")).toBe(
      '"Test Sans", sans-serif',
    );
  });

  it("[F15.AC1] counts a file it cannot read, a collection among them, and keeps the list", async () => {
    fakeLocalFonts([installedFont("Test Bad", Uint8Array.from([0x74, 0x74, 0x63, 0x66]))]);

    clickElement("installed-fonts");
    await waitForMessage("Installed families listed: 1. Choose one in the font list.");
    chooseOption("text-font", "Test Bad");

    await waitForMessage(
      "Test Bad: 0 faces read, 1 refused (not a TrueType or OpenType font, a collection among them).",
    );

    expect(readOptions("text-font")).toEqual(["Inter"]);
    expect(readValue("text-font")).toBe("Inter");
  });

  it("[F15.AC1] says the permission was refused", async () => {
    const refused = vi
      .fn<() => Promise<readonly InstalledFont[]>>()
      .mockRejectedValue(new DOMException("Permission denied.", "NotAllowedError"));

    Object.defineProperty(globalThis, "queryLocalFonts", { configurable: true, value: refused });

    clickElement("installed-fonts");

    await waitForMessage(
      "The installed fonts are not listed: the browser's permission was refused.",
    );

    expect(readOptions("text-font")).toEqual(["Inter"]);
  });

  it("[F15.AC1] says a family whose file the browser could not give was not read", async () => {
    const broken = vi.fn<() => Promise<Blob>>().mockRejectedValue(new Error("No file."));

    fakeLocalFonts([{ blob: broken, family: "Test Sans", fullName: "Test Sans" }]);
    clickElement("installed-fonts");
    await waitForMessage("Installed families listed: 1. Choose one in the font list.");
    chooseOption("text-font", "Test Sans");
    await waitForMessage("Test Sans could not be read: Error: No file.");
  });
});

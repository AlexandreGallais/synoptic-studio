// @vitest-environment happy-dom

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import {
  APPLE_TRUE_TYPE_VERSION,
  fontCollection,
  postScriptFaceFile,
  testFaceFile,
  uint32,
} from "../../tests";
import {
  chooseFiles,
  chooseOption,
  clickElement,
  openServedPage,
  readText,
  readValue,
  selectChoice,
  selectPageElement,
  typeValue,
  waitForPageTasks,
} from "../user-actions";

import type { TestFace } from "../../tests";
import type { InstalledFont } from "../fonts";

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
    postscriptName: family,
  };
}

/**
 * An installed face whose file the browser gives as its whole collection, as on Windows and
 * Linux (`REF-CHROMIUM-FONT-BLOB-CODE`).
 *
 * @param family - its family, as the browser names it
 * @param postscriptName - its PostScript name, which finds it in the collection
 * @param collection - the bytes of the whole collection
 * @returns a `FontData` holding them
 */
function collectionFont(
  family: string,
  postscriptName: string,
  collection: Uint8Array,
): InstalledFont {
  return { ...installedFont(family, collection), fullName: postscriptName, postscriptName };
}

/**
 * An installed face whose file the browser cannot give: Chrome's `TypeError`.
 *
 * @param fullName - its full name
 * @param message - Chrome's message
 * @returns a `FontData` whose `blob()` rejects
 */
function inaccessibleFont(fullName: string, message: string): InstalledFont {
  return {
    blob: vi.fn<() => Promise<Blob>>().mockRejectedValue(new TypeError(message)),
    family: "Test Sans",
    fullName,
    postscriptName: fullName.replaceAll(" ", "-"),
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
 * Makes the browser refuse to list the installed fonts.
 *
 * @param error - the error `queryLocalFonts` rejects with
 */
function fakeRefusedList(error: DOMException): void {
  const query = vi.fn<() => Promise<readonly InstalledFont[]>>().mockRejectedValue(error);

  Object.defineProperty(globalThis, "queryLocalFonts", { configurable: true, value: query });
}

/**
 * Imports Test Sans 400 from its file, as the designer does, and waits for it to be read.
 *
 * @returns once it is read
 */
async function importFace(): Promise<void> {
  const bytes = new Uint8Array(testFaceFile(TEST_SANS));

  chooseFiles("font-file", [new File([bytes], "TestSans.ttf")]);
  await waitForPageTasks();
}

/**
 * Types a content in the Text tool's text area, as the designer would.
 *
 * @param content - the lines
 */
function typeContent(content: string): void {
  const area = selectPageElement("#text-content");

  if (area instanceof HTMLTextAreaElement) {
    area.value = content;
  }

  area.dispatchEvent(new Event("input", { bubbles: true }));
}

/**
 * Reads the font list the drawn text names.
 *
 * @returns its `font-family`
 */
function readFontFamily(): string | null {
  return selectPageElement("#canvas text").getAttribute("font-family");
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
      {
        blob: bold,
        family: "Test Sans",
        fullName: "Test Sans Bold",
        postscriptName: "TestSans-Bold",
      },
      installedFont("Test Serif", testFaceFile({ ...TEST_SANS, family: "Test Serif" })),
    ]);

    clickElement("installed-fonts");
    await waitForMessage("Installed families listed: 2. Choose one in the font list.");

    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans", "Test Serif"]);
    expect(readValue("text-font")).toBe("Inter");
    expect(bold).not.toHaveBeenCalled();
  });

  it("[F15.AC1] reads the faces of a family chosen and measures the text with them, never with Inter meanwhile", async () => {
    fakeLocalFonts([
      installedFont("Test Sans", testFaceFile(TEST_SANS)),
      installedFont("Test Sans", testFaceFile({ ...TEST_SANS, weightClass: 700 })),
    ]);

    typeContent("B");

    const before = readText("#contour");

    clickElement("installed-fonts");
    await waitForPageTasks();
    chooseOption("text-font", "Test Sans");

    // While its files are read: the choice shown, the text still measured and drawn in Inter.
    expect([readValue("text-font"), readText("#font-message")]).toEqual([
      "Test Sans",
      "Reading Test Sans…",
    ]);

    expect(readText("#contour")).toBe(before);
    expect(readFontFamily()).toBe('"Inter", sans-serif');

    await waitForPageTasks();

    expect(readText("#font-message")).toBe("Test Sans: 2 faces read.");
    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans"]);
    expect(readOptions("text-weight")).toEqual(["400", "700"]);
    // "B" at 100: 700 × 100 / 1000 = 70.
    expect(readText("#contour")).toContain('"width": 70');
    expect(readFontFamily()).toBe('"Test Sans", sans-serif');
  });

  it("[F15.AC1] keeps the family chosen before when the browser cannot give a family's file", async () => {
    const broken = vi.fn<() => Promise<Blob>>().mockRejectedValue(new Error("No file."));

    fakeLocalFonts([
      { blob: broken, family: "Test Serif", fullName: "Test Serif", postscriptName: "TestSerif" },
    ]);

    await importFace();
    clickElement("installed-fonts");
    await waitForPageTasks();
    chooseOption("text-font", "Test Serif");
    await waitForPageTasks();

    expect(readText("#font-message")).toBe(
      "Test Serif could not be read (Test Serif: Error: No file.).",
    );

    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans"]);
    expect(readValue("text-font")).toBe("Test Sans");
    expect(readFontFamily()).toBe('"Test Sans", sans-serif');
  });

  it("[F15.AC1] keeps the faces read and counts those refused, one good and two bad", async () => {
    const broken = vi.fn<() => Promise<Blob>>().mockRejectedValue(new Error("No file."));

    fakeLocalFonts([
      installedFont("Test Sans", testFaceFile(TEST_SANS)),
      {
        blob: broken,
        family: "Test Sans",
        fullName: "Test Sans Bold",
        postscriptName: "TestSans-Bold",
      },
      installedFont("Test Sans", Uint8Array.from([0x74, 0x74, 0x63, 0x66])),
    ]);

    clickElement("installed-fonts");
    await waitForPageTasks();
    chooseOption("text-font", "Test Sans");
    await waitForPageTasks();

    expect(readText("#font-message")).toBe(
      "Test Sans: 1 faces read, 2 refused (Test Sans Bold: Error: No file.; not a TrueType or OpenType font).",
    );

    expect(readValue("text-font")).toBe("Test Sans");
    expect(readOptions("text-weight")).toEqual(["400"]);
  });

  it("[F15.AC1] gives the list back to Inter for a file it cannot read, a collection's tag alone", async () => {
    fakeLocalFonts([installedFont("Test Bad", Uint8Array.from([0x74, 0x74, 0x63, 0x66]))]);
    clickElement("installed-fonts");
    await waitForPageTasks();
    chooseOption("text-font", "Test Bad");
    await waitForPageTasks();

    expect(readText("#font-message")).toBe(
      "Test Bad could not be read (not a TrueType or OpenType font).",
    );

    expect(readOptions("text-font")).toEqual(["Inter"]);
    expect(readValue("text-font")).toBe("Inter");
  });

  it("[F15.AC1] says a reason shared by the faces refused once", async () => {
    // Two faces whose files are not fonts: one reason, not "…; …" twice.
    fakeLocalFonts([
      installedFont("Test Bad", Uint8Array.from([1, 2, 3])),
      installedFont("Test Bad", Uint8Array.from([4, 5, 6])),
    ]);

    clickElement("installed-fonts");
    await waitForPageTasks();
    chooseOption("text-font", "Test Bad");
    await waitForPageTasks();

    expect(readText("#font-message")).toBe(
      "Test Bad could not be read (not a TrueType or OpenType font).",
    );
  });

  it("[F15.AC1] lists an installed family once when a kept family has its name in another case", async () => {
    fakeLocalFonts([installedFont("TEST SANS", testFaceFile(TEST_SANS))]);
    await importFace();
    clickElement("installed-fonts");
    await waitForPageTasks();

    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans"]);
  });

  it("[F15.AC1] says the permission was refused", async () => {
    fakeRefusedList(new DOMException("Permission denied.", "NotAllowedError"));
    clickElement("installed-fonts");
    await waitForPageTasks();

    expect(readText("#font-message")).toBe(
      "The installed fonts are not listed: the browser's permission was refused.",
    );

    expect(readOptions("text-font")).toEqual(["Inter"]);
  });

  it("[F15.AC1] says why the list was refused for another reason, a click missing (SecurityError)", async () => {
    fakeRefusedList(new DOMException("No user activation.", "SecurityError"));
    clickElement("installed-fonts");
    await waitForPageTasks();

    expect(readText("#font-message")).toMatch(
      /^The installed fonts could not be listed: .*No user activation\./u,
    );
  });

  it("[F15.AC1] reads nothing when a kept family is chosen after the list (M7)", async () => {
    fakeLocalFonts([installedFont("Test Sans", testFaceFile(TEST_SANS))]);
    clickElement("installed-fonts");
    await waitForPageTasks();
    chooseOption("text-font", "Inter");
    await waitForPageTasks();

    expect(readText("#font-message")).toBe(
      "Installed families listed: 1. Choose one in the font list.",
    );
  });

  it("[F15.AC1] chooses the family its files name once read, not the browser's name (M12)", async () => {
    fakeLocalFonts([installedFont("Test Sans Bold", testFaceFile(TEST_SANS))]);
    clickElement("installed-fonts");
    await waitForPageTasks();
    chooseOption("text-font", "Test Sans Bold");
    await waitForPageTasks();

    expect(readValue("text-font")).toBe("Test Sans");
    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans"]);
  });

  it("[F15.AC1] says the browser cannot list them when queryLocalFonts is not a function (M6)", () => {
    Object.defineProperty(globalThis, "queryLocalFonts", { configurable: true, value: 42 });
    clickElement("installed-fonts");
    expect(readText("#font-message")).toBe(UNSUPPORTED);
  });

  it("[F17.AC4] reads a face tagged true, as Chrome gives one on macOS (Helvetica's faces)", async () => {
    const bytes = [...testFaceFile(TEST_SANS)];

    bytes.splice(0, 4, ...uint32(APPLE_TRUE_TYPE_VERSION));
    fakeLocalFonts([installedFont("Test Sans", Uint8Array.from(bytes))]);
    clickElement("installed-fonts");
    await waitForPageTasks();
    chooseOption("text-font", "Test Sans");
    await waitForPageTasks();

    expect(readText("#font-message")).toBe("Test Sans: 1 faces read.");
  });

  it("[F17.AC4] keeps from a whole collection the faces of the family chosen, by PostScript name", async () => {
    const collection = fontCollection([
      postScriptFaceFile(TEST_SANS, "TestSans-Regular"),
      postScriptFaceFile({ ...TEST_SANS, family: "Other Serif" }, "OtherSerif-Regular"),
      postScriptFaceFile({ ...TEST_SANS, weightClass: 700 }, "TestSans-Bold"),
    ]);

    fakeLocalFonts([
      collectionFont("Test Sans", "TestSans-Regular", collection),
      collectionFont("Test Sans", "TestSans-Bold", collection),
      collectionFont("Other Serif", "OtherSerif-Regular", collection),
    ]);

    clickElement("installed-fonts");
    await waitForPageTasks();
    chooseOption("text-font", "Test Sans");
    await waitForPageTasks();

    expect(readText("#font-message")).toBe("Test Sans: 2 faces read.");
    expect(readValue("text-font")).toBe("Test Sans");
    expect(readOptions("text-weight")).toEqual(["400", "700"]);
    // Other Serif stays listed as installed, its file unread.
    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans", "Other Serif"]);
  });

  it("[F17.AC4] refuses a face missing from the collection the browser gives", async () => {
    // Two faces, neither named TestSans-Bold.
    const collection = fontCollection([
      postScriptFaceFile(TEST_SANS, "TestSans-Regular"),
      postScriptFaceFile({ ...TEST_SANS, isItalic: true }, "TestSans-Italic"),
    ]);

    fakeLocalFonts([
      installedFont("Test Sans", testFaceFile(TEST_SANS)),
      collectionFont("Test Sans", "TestSans-Bold", collection),
    ]);

    clickElement("installed-fonts");
    await waitForPageTasks();
    chooseOption("text-font", "Test Sans");
    await waitForPageTasks();

    expect(readText("#font-message")).toBe(
      "Test Sans: 1 faces read, 1 refused (no face named TestSans-Bold in its collection).",
    );
  });

  it("[F17.AC4] refuses a face Chrome cannot give (TypeError) in one line, naming it, the others kept", async () => {
    fakeLocalFonts([
      installedFont("Test Sans", testFaceFile(TEST_SANS)),
      inaccessibleFont("Test Sans Light", "The font Test-Sans-Light could not be accessed."),
    ]);

    clickElement("installed-fonts");
    await waitForPageTasks();
    chooseOption("text-font", "Test Sans");
    await waitForPageTasks();

    expect(readText("#font-message")).toBe(
      "Test Sans: 1 faces read, 1 refused (Test Sans Light: The font Test-Sans-Light could not be accessed. Import its file with Import a font).",
    );

    expect(readValue("text-font")).toBe("Test Sans");
  });
});

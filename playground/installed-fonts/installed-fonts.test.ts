// @vitest-environment happy-dom

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import {
  APPLE_TRUE_TYPE_VERSION,
  fontCollection,
  lettersMissingFaceFile,
  postScriptFaceFile,
  testFaceFile,
  uint32,
} from "../../tests";
import { readFontSession } from "../fonts";
import {
  chooseFiles,
  chooseOption,
  clickElement,
  openServedPage,
  readDisabledState,
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
 * An installed face whose file the browser gives only once released, so that a family can be
 * chosen while it is read ahead.
 *
 * @param family - its family, as the browser names it
 * @param bytes - its file's bytes
 * @returns the face, and what gives its file to every read waiting for it
 */
function heldFont(family: string, bytes: Uint8Array): readonly [InstalledFont, () => void] {
  const { promise, resolve } = Promise.withResolvers<Blob>();
  const font = {
    ...installedFont(family, bytes),
    blob: vi.fn<() => Promise<Blob>>(async () => await promise),
  };

  return [
    font,
    (): void => {
      resolve(new Blob([new Uint8Array(bytes)]));
    },
  ];
}

/**
 * Reads what the page says while the installed fonts are read ahead.
 *
 * @returns its line
 */
function readProgress(): string {
  return readText("#installed-fonts-progress");
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

/** What a browser says when it cannot allocate a buffer, a file too large to hold. */
const ALLOCATION_FAILED = "RangeError: Array buffer allocation failed";

/**
 * Makes the browser refuse to join a face of several parts — a face of a collection — as when
 * its file is too large to hold, a face of one part joined as before.
 */
function refuseCollectionJoins(): void {
  vi.stubGlobal(
    "Blob",
    new Proxy(Blob, {
      construct(page, [parts]: readonly (readonly BlobPart[])[]): object {
        if (parts !== undefined && parts.length > 1) {
          throw new RangeError("Array buffer allocation failed");
        }

        const blob: unknown = Reflect.construct(page, [parts]);

        return blob instanceof page ? blob : {};
      },
    }),
  );
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
    vi.unstubAllGlobals();
  });

  it("[F15.AC1] says where the browser cannot list them, Firefox and Safari, and points to Import a font", () => {
    clickElement("installed-fonts");
    expect(readText("#font-message")).toBe(UNSUPPORTED);
    expect(readOptions("text-font")).toEqual(["Inter"]);
  });

  it("[F17.AC11] lists the installed families once the permission is granted, then reads them ahead, a face that is not a font refused", async () => {
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

    await waitForMessage(
      "Installed families listed: 2. The font list can be used once they are read.",
    );

    await waitForPageTasks();

    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans", "Test Serif"]);
    expect(readValue("text-font")).toBe("Inter");
    // Each face read once ahead (US-053); Test Sans Bold's empty file refused, Test Sans kept.
    expect(bold).toHaveBeenCalledOnce();
    expect(readProgress()).toBe("Installed fonts read: 2 families.");
  });

  it("[F17.AC11] measures a family read ahead at once when chosen, then reads its files again", async () => {
    fakeLocalFonts([
      installedFont("Test Sans", testFaceFile(TEST_SANS)),
      installedFont("Test Sans", testFaceFile({ ...TEST_SANS, weightClass: 700 })),
    ]);

    typeContent("B");

    const before = readText("#contour");

    clickElement("installed-fonts");
    await waitForPageTasks();
    chooseOption("text-font", "Test Sans");

    // While its files are read again: measured at once with the metrics read ahead (US-053).
    expect([readValue("text-font"), readText("#font-message")]).toEqual([
      "Test Sans",
      "Reading Test Sans…",
    ]);

    expect(readText("#contour")).not.toBe(before);
    expect(readFontFamily()).toBe('"Test Sans", sans-serif');

    await waitForPageTasks();

    expect(readText("#font-message")).toBe("Test Sans: 2 faces read.");
    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans"]);
    expect(readOptions("text-weight")).toEqual(["400", "700"]);
    // "B" at 100: 700 × 100 / 1000 = 70.
    expect(readText("#contour")).toContain('"width": 70');
    expect(readFontFamily()).toBe('"Test Sans", sans-serif');
  });

  it("[F17.AC17] never offers an installed family before it is read, the family imported kept when it cannot be", async () => {
    const { promise, reject } = Promise.withResolvers<Blob>();
    const broken = vi.fn<() => Promise<Blob>>(async () => await promise);

    fakeLocalFonts([
      { blob: broken, family: "Test Serif", fullName: "Test Serif", postscriptName: "TestSerif" },
    ]);

    await importFace();
    clickElement("installed-fonts");

    await waitForMessage(
      "Installed families listed: 1. The font list can be used once they are read.",
    );

    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans"]);
    expect(readDisabledState("text-font")).toBe("true");
    reject(new Error("No file."));
    await waitForPageTasks();

    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans"]);
    expect(readDisabledState("text-font")).toBe("false");
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

  it("[F17.AC11] leaves out of the list the families none of whose files can be read, a collection's tag alone, a face Chrome cannot give", async () => {
    fakeLocalFonts([
      installedFont("Test Bad", Uint8Array.from([0x74, 0x74, 0x63, 0x66])),
      installedFont("Test Sans", testFaceFile(TEST_SANS)),
      {
        ...inaccessibleFont("Test Gone", "The font Test-Gone could not be accessed."),
        family: "Test Gone",
      },
    ]);

    clickElement("installed-fonts");
    await waitForPageTasks();

    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans"]);
    expect(readValue("text-font")).toBe("Inter");

    expect(readProgress()).toBe(
      "Installed fonts read: 1 families; 2 left out: 2 unreadable (Test Bad, Test Gone).",
    );
  });

  it("[F17.AC11] says how many families are read while they are, the page answering between two files", async () => {
    const [serif, release] = heldFont(
      "Test Serif",
      testFaceFile({ ...TEST_SANS, family: "Test Serif" }),
    );

    fakeLocalFonts([installedFont("Test Sans", testFaceFile(TEST_SANS)), serif]);
    clickElement("installed-fonts");

    await vi.waitFor(() => {
      expect(readProgress()).toBe("Reading the installed fonts: 1 of 2 families…");
    });

    // Test Sans read while Test Serif's file is awaited: Test Serif not offered yet (US-059).
    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans"]);
    release();
    await waitForPageTasks();
    expect(readProgress()).toBe("Installed fonts read: 2 families.");
  });

  it("[F17.AC11] says an error of the reading ahead, never left uncaught", async () => {
    const [sans, release] = heldFont("Test Sans", testFaceFile(TEST_SANS));

    // Test Serif, still to read when the reading stops, keeps the list disabled unless it is cleared.
    fakeLocalFonts([sans, installedFont("Test Serif", testFaceFile(TEST_SANS))]);
    clickElement("installed-fonts");

    await waitForMessage(
      "Installed families listed: 2. The font list can be used once they are read.",
    );

    // The progress line gone while Test Sans is read: its next write throws.
    selectPageElement("#installed-fonts-progress").remove();
    release();
    await waitForPageTasks();

    expect(readText("#font-message")).toMatch(
      /^The installed fonts could not be read: Error: Playground page without #installed-fonts-progress/u,
    );

    // The reading stopped, the font list can be used again (US-059).
    expect(readDisabledState("text-font")).toBe("false");
  });

  it("[F17.AC11] keeps the file of a face imported when its installed family is read ahead", async () => {
    fakeLocalFonts([installedFont("Test Sans", testFaceFile(TEST_SANS))]);
    await importFace();
    clickElement("installed-fonts");
    await waitForPageTasks();

    const [, family] = readFontSession(document).kept;

    expect(family?.faces.map((face) => face.file === undefined)).toEqual([false]);
  });

  it("[F17.AC11] keeps a family read ahead its metrics, not its files", async () => {
    fakeLocalFonts([installedFont("Test Sans", testFaceFile(TEST_SANS))]);
    clickElement("installed-fonts");
    await waitForPageTasks();

    const [, family] = readFontSession(document).kept;

    expect([family?.name, family?.faces.map((face) => face.file)]).toEqual([
      "Test Sans",
      [undefined],
    ]);
  });

  it("[F15.AC1] says a reason shared by the faces refused once", async () => {
    // Two faces whose files are not fonts: one reason, not "…; …" twice.
    fakeLocalFonts([
      installedFont("Test Sans", testFaceFile(TEST_SANS)),
      installedFont("Test Sans", Uint8Array.from([1, 2, 3])),
      installedFont("Test Sans", Uint8Array.from([4, 5, 6])),
    ]);

    clickElement("installed-fonts");
    await waitForPageTasks();
    chooseOption("text-font", "Test Sans");
    await waitForPageTasks();

    expect(readText("#font-message")).toBe(
      "Test Sans: 1 faces read, 2 refused (not a TrueType or OpenType font).",
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
      "Installed families listed: 1. The font list can be used once they are read.",
    );
  });

  it("[F15.AC1] lists a family under the name its files give once read, not the browser's name (M12)", async () => {
    fakeLocalFonts([installedFont("Test Sans Bold", testFaceFile(TEST_SANS))]);
    clickElement("installed-fonts");
    await waitForPageTasks();

    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans"]);

    chooseOption("text-font", "Test Sans");
    await waitForPageTasks();

    expect(readText("#font-message")).toBe("Test Sans: 1 faces read.");
    expect(readValue("text-font")).toBe("Test Sans");
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
    // Other Serif read ahead too, its metrics kept (US-053).
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

  it("[F17.AC10] refuses a face whose file the browser cannot hold in one line, naming it, the others kept", async () => {
    const collection = fontCollection([
      postScriptFaceFile(TEST_SANS, "TestSans-Regular"),
      postScriptFaceFile({ ...TEST_SANS, weightClass: 700 }, "TestSans-Bold"),
    ]);

    fakeLocalFonts([
      installedFont("Test Sans", testFaceFile(TEST_SANS)),
      collectionFont("Test Sans", "TestSans-Bold", collection),
    ]);

    clickElement("installed-fonts");
    await waitForPageTasks();
    refuseCollectionJoins();
    chooseOption("text-font", "Test Sans");
    await waitForPageTasks();

    expect(readText("#font-message")).toBe(
      `Test Sans: 1 faces read, 1 refused (TestSans-Bold: ${ALLOCATION_FAILED}).`,
    );

    expect(readValue("text-font")).toBe("Test Sans");
    // 700's metrics, read ahead before the joins were refused, stay kept (US-053).
    expect(readOptions("text-weight")).toEqual(["400", "700"]);
  });

  it("[F17.AC14] leaves out the families that cannot be embedded or lack the Latin letters, counted by reason", async () => {
    fakeLocalFonts([
      installedFont("Test Emoji", lettersMissingFaceFile({ ...TEST_SANS, family: "Test Emoji" })),
      installedFont(
        "Test Locked",
        testFaceFile({ ...TEST_SANS, family: "Test Locked", fsType: 2 }),
      ),
      installedFont("Test Sans", testFaceFile(TEST_SANS)),
      installedFont("Test Bad", Uint8Array.from([0x74, 0x74, 0x63, 0x66])),
    ]);

    clickElement("installed-fonts");
    await waitForPageTasks();

    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans"]);

    expect(readProgress()).toBe(
      "Installed fonts read: 1 families; 3 left out: 1 not embeddable (Test Locked); 1 no Latin letters (Test Emoji); 1 unreadable (Test Bad).",
    );
  });

  it("[F17.AC14] leaves out a family whose faces have different reasons under the first, not embeddable before no Latin letters", async () => {
    fakeLocalFonts([
      installedFont(
        "Test Mixed",
        lettersMissingFaceFile({ ...TEST_SANS, family: "Test Mixed", weightClass: 700 }),
      ),
      installedFont("Test Mixed", testFaceFile({ ...TEST_SANS, family: "Test Mixed", fsType: 2 })),
      installedFont("Test Mixed", Uint8Array.from([1, 2, 3])),
    ]);

    clickElement("installed-fonts");
    await waitForPageTasks();

    expect(readOptions("text-font")).toEqual(["Inter"]);

    expect(readProgress()).toBe(
      "Installed fonts read: 0 families; 1 left out: 1 not embeddable (Test Mixed).",
    );
  });

  it("[F17.AC14] keeps a family's faces that can be listed, and says those left out when it is chosen", async () => {
    fakeLocalFonts([
      installedFont("Test Sans", testFaceFile(TEST_SANS)),
      {
        ...installedFont("Test Sans", testFaceFile({ ...TEST_SANS, fsType: 2, weightClass: 700 })),
        fullName: "Test Sans Bold",
      },
    ]);

    clickElement("installed-fonts");
    await waitForPageTasks();

    expect(readProgress()).toBe("Installed fonts read: 1 families.");
    chooseOption("text-font", "Test Sans");
    await waitForPageTasks();

    expect(readText("#font-message")).toBe(
      "Test Sans: 1 faces read, 1 refused (Test Sans Bold: its fsType forbids embedding it).",
    );

    expect(readOptions("text-weight")).toEqual(["400"]);
  });

  it("[F17.AC14] keeps a face imported by hand that lacks the Latin letters, the designer having chosen it", async () => {
    const bytes = lettersMissingFaceFile({ ...TEST_SANS, family: "Test Emoji" });

    chooseFiles("font-file", [new File([new Uint8Array(bytes)], "TestEmoji.ttf")]);
    await waitForPageTasks();

    expect(readOptions("text-font")).toEqual(["Inter", "Test Emoji"]);
    expect(readText("#font-message")).toBe("Test Emoji 400 imported.");
  });
});

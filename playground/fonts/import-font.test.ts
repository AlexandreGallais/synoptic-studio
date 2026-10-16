// @vitest-environment happy-dom

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { facePartsToFile } from "../../src";
import { ARIAL_PATH, fontCollection, systemFont, testFaceFile } from "../../tests";
import {
  chooseFiles,
  chooseOption,
  clickElement,
  openServedPage,
  readText,
  readValue,
  selectChoice,
  selectInput,
  selectPageElement,
  typeValue,
} from "../user-actions";

import type { TestFace } from "../../tests";

/** Arial 5.01 of macOS, read where the machine has it. */
const ARIAL = systemFont(ARIAL_PATH);

/** The test font named Test Sans, regular, installable. */
const TEST_SANS: TestFace = { family: "Test Sans", fsType: 0, isItalic: false, weightClass: 400 };

/**
 * A font file as the designer chooses it.
 *
 * @param face - its family, weight, style and permissions
 * @param name - its file's name
 * @returns a `File` holding its bytes
 */
function fontFileOf(face: TestFace, name: string): File {
  return new File([new Uint8Array(testFaceFile(face))], name);
}

/**
 * Chooses files in the font picker and waits for the page to say what it read.
 *
 * @param files - the files chosen
 * @param message - the line the page says once they are read
 * @returns once the page says it
 */
async function importFiles(files: readonly File[], message: string): Promise<void> {
  chooseFiles("font-file", files);

  await vi.waitFor(() => {
    expect(readText("#font-message")).toBe(message);
  });
}

/**
 * Reads the families the font list offers.
 *
 * @returns their names, in order
 */
function readFontList(): readonly string[] {
  return [...selectChoice("text-font").options].map((option) => option.value);
}

/**
 * Reads the measured block shown in the pipeline column.
 *
 * @returns its width and height
 */
function readBlock(): { readonly height: number; readonly width: number } {
  // eslint-disable-next-line @typescript-eslint/consistent-type-assertions, @typescript-eslint/no-unsafe-type-assertion -- written by showText from a TextBlock
  return JSON.parse(readText("#contour")) as { readonly height: number; readonly width: number };
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

/** What a browser says when it cannot allocate a buffer, a file too large to hold. */
const ALLOCATION_FAILED = "RangeError: Array buffer allocation failed";

/** The library, its join of a face's parts typed: the other functions are spread as they are. */
type JoiningLibrary = Readonly<Record<string, unknown>> & {
  /** Joins a face's parts into its file. */
  readonly facePartsToFile: typeof facePartsToFile;
};

// The library's join of a face's parts, wrapped in a spy that a test can make refuse.
vi.mock("../../src", async (importOriginal) => {
  const library = await importOriginal<JoiningLibrary>();

  return { ...library, facePartsToFile: vi.fn(library.facePartsToFile) };
});

/**
 * Makes the editor refuse to join a face of several parts — a face of a collection — as when
 * its file is too large to hold, a face of one part joined as before.
 */
function refuseCollectionJoins(): void {
  const join = vi.mocked(facePartsToFile);
  const joined = join.getMockImplementation() ?? facePartsToFile;

  join.mockImplementation((parts) => {
    if (parts.length > 1) {
      throw new RangeError("Array buffer allocation failed");
    }

    return joined(parts);
  });
}

describe("Import a font (US-036)", () => {
  afterEach(() => {
    vi.unstubAllGlobals();
    vi.mocked(facePartsToFile).mockReset();
  });

  beforeEach(() => {
    openServedPage("");
    clickElement("tool-text");
    chooseOption("text-weight", "400");
    typeValue("text-size", "100");
    typeContent("AB");
  });

  it("[F15.AC1] offers a list of the families it measures, Inter alone as the page opens", () => {
    expect(selectPageElement("#text-font").tagName).toBe("SELECT");
    expect(readFontList()).toEqual(["Inter"]);
    expect(readValue("text-font")).toBe("Inter");
    expect(selectPageElement("#font-file").getAttribute("accept")).toBe(".ttf,.otf,.ttc");
  });

  it("[F15.AC1] adds the family of a file imported, chooses it, and measures the text with it", async () => {
    await importFiles([fontFileOf(TEST_SANS, "TestSans.ttf")], "Test Sans 400 imported.");

    expect(readFontList()).toEqual(["Inter", "Test Sans"]);
    expect(readValue("text-font")).toBe("Test Sans");

    expect(selectPageElement("#canvas text").getAttribute("font-family")).toBe(
      '"Test Sans", sans-serif',
    );

    // At 100, A is 600 × 100 / 1000 = 60 wide and B 70: 130; one line of
    // (800 + 200 + 90) × 100 / 1000 = 109; the baseline at the ascender, 80.
    expect(readBlock()).toEqual({ height: 109, width: 130 });
    expect(selectPageElement("#canvas tspan").getAttribute("y")).toBe("80");
  });

  it("[F15.AC1] measures with Inter again when Inter is chosen back", async () => {
    await importFiles([fontFileOf(TEST_SANS, "TestSans.ttf")], "Test Sans 400 imported.");
    chooseOption("text-font", "Inter");

    expect(readBlock().width).not.toBe(130);

    expect(selectPageElement("#canvas text").getAttribute("font-family")).toBe(
      '"Inter", sans-serif',
    );
  });

  it("[F17.AC4] imports each face of a collection (.ttc), the last one's family chosen", async () => {
    const collection = fontCollection([
      testFaceFile(TEST_SANS),
      testFaceFile({ ...TEST_SANS, isItalic: true, weightClass: 700 }),
      testFaceFile({ ...TEST_SANS, family: "Other Serif" }),
    ]);

    await importFiles(
      [new File([new Uint8Array(collection)], "Test.ttc")],
      "Test Sans 400, Test Sans 700 italic, Other Serif 400 imported.",
    );

    expect(readFontList()).toEqual(["Inter", "Test Sans", "Other Serif"]);
    expect(readValue("text-font")).toBe("Other Serif");
  });

  it("[F17.AC10] says a collection whose faces the browser cannot hold, the other files kept", async () => {
    const collection = fontCollection([
      testFaceFile({ ...TEST_SANS, family: "Other Serif" }),
      testFaceFile({ ...TEST_SANS, family: "Other Serif", weightClass: 700 }),
    ]);

    refuseCollectionJoins();

    await importFiles(
      [fontFileOf(TEST_SANS, "TestSans.ttf"), new File([new Uint8Array(collection)], "Emoji.ttc")],
      `Test Sans 400 imported. Emoji.ttc could not be read (${ALLOCATION_FAILED}).`,
    );

    expect(readFontList()).toEqual(["Inter", "Test Sans"]);
    expect(readValue("text-font")).toBe("Test Sans");
  });

  it("[F15.AC1] refuses a file that is not a font, said why, the list unchanged", async () => {
    const notes = new File(["Pump P-101"], "notes.ttf");

    await importFiles(
      [notes],
      "notes.ttf is not a TrueType or OpenType font file (.ttf, .otf, .ttc): refused.",
    );

    expect(readFontList()).toEqual(["Inter"]);
    expect(readValue("text-font")).toBe("Inter");
  });

  it("[F15.AC1] keeps one family for two files of it, its name matched without regard to case", async () => {
    await importFiles(
      [
        fontFileOf(TEST_SANS, "TestSans.ttf"),
        fontFileOf({ ...TEST_SANS, family: "TEST SANS", weightClass: 700 }, "TestSansBold.ttf"),
      ],
      "Test Sans 400 imported. TEST SANS 700 imported.",
    );

    expect(readFontList()).toEqual(["Inter", "Test Sans"]);
    expect(readValue("text-font")).toBe("Test Sans");
  });

  it.skipIf(ARIAL === undefined)(
    "[F15.AC1] measures Hello World in Arial 16 with Arial's metrics, its kerning included",
    async () => {
      await importFiles(
        [new File([new Uint8Array(ARIAL ?? [])], "Arial.ttf")],
        "Arial 400 imported.",
      );

      typeValue("text-size", "16");
      typeContent("Hello World");

      // 82.6875 without kerning (US-036); Arial's kerning narrows it to 82.3984375. Inter would
      // give 87.5546875 × 19.359375.
      expect(readBlock()).toEqual({ height: 18.3984375, width: 82.3984375 });

      expect([...selectChoice("text-weight").options].map((option) => option.value)).toEqual([
        "400",
      ]);
    },
  );

  it("[F15.AC1] keeps the files read and says which the browser could not give, one good and one bad", async () => {
    const unreadable = new File([], "Arial.ttf");

    vi.spyOn(unreadable, "arrayBuffer").mockRejectedValue(new Error("Read failed."));

    await importFiles(
      [fontFileOf(TEST_SANS, "TestSans.ttf"), unreadable],
      "Test Sans 400 imported. Arial.ttf could not be read (Error: Read failed.).",
    );

    expect(readFontList()).toEqual(["Inter", "Test Sans"]);
    expect(readValue("text-font")).toBe("Test Sans");
  });

  it("[F15.AC1] names an italic face as italic, and empties the picker for the same file again", async () => {
    await importFiles(
      [fontFileOf({ ...TEST_SANS, isItalic: true }, "TestSansItalic.ttf")],
      "Test Sans 400 italic imported.",
    );

    expect(selectInput("font-file").value).toBe("");
  });

  it("[F15.AC1] chooses the last family of files of two families picked together (M14)", async () => {
    await importFiles(
      [
        fontFileOf(TEST_SANS, "TestSans.ttf"),
        fontFileOf({ ...TEST_SANS, family: "Test Serif" }, "TestSerif.ttf"),
      ],
      "Test Sans 400 imported. Test Serif 400 imported.",
    );

    expect(readFontList()).toEqual(["Inter", "Test Sans", "Test Serif"]);
    expect(readValue("text-font")).toBe("Test Serif");
  });
});

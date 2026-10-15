// @vitest-environment happy-dom
import { afterEach, describe, expect, it, vi } from "vitest";

import { ARIAL_PATH, systemFont, testFaceFile } from "../../../tests";
import { readExportedSvg } from "../../document";
import { readTextModel } from "../../text";
import {
  chooseFiles,
  chooseOption,
  clickElement,
  openOnStep,
  openServedPage,
  readDisplay,
  readText,
  readValue,
  selectChoice,
  selectInput,
  selectPageElement,
  waitForPageTasks,
} from "../../user-actions";
import { CURRENT_GUIDED_STEPS, GUIDED_STEPS } from "../lists";

import type { TestFace } from "../../../tests";

// Arial 5.01 of macOS at 16: a line (1854 + 434 + 67) × 16 / 2048 = 18.3984375 tall, its line gap
// 67 × 16 / 2048 = 0.5234375 shared half above, so the layer is raised by 0.26171875; the
// baseline 0.26171875 + 1854 × 16 / 2048 = 14.484375 below the line's top.

/** Arial 5.01 of macOS, read where the machine has it. */
const ARIAL = systemFont(ARIAL_PATH);

/** Noto Sans Javanese of macOS, an OpenType file with CFF outlines, read where the machine has it. */
const NOTO_JAVANESE = systemFont("/System/Library/Fonts/Supplemental/NotoSansJavanese-Regular.otf");

/** The test font named Test Sans, regular, installable. */
const TEST_SANS: TestFace = { family: "Test Sans", fsType: 0, isItalic: false, weightClass: 400 };

/** Said in Firefox and Safari, which cannot list the installed fonts. */
const UNSUPPORTED =
  "This browser cannot list the installed fonts (Chrome and Edge can): use Import a font with the font's file.";

/**
 * Opens the playground's epic list on a step of F15, found by its title.
 *
 * @param title - the step's title, without "F15 · "
 */
function openStep(title: string): void {
  const index = GUIDED_STEPS.findIndex((step) => step.title === `F15 · ${title}`);

  expect(index).toBeGreaterThanOrEqual(0);
  openOnStep(index + 1);
}

/** Ticks Embed fonts, as the designer would: its `change` event. */
function tickEmbedFonts(): void {
  const box = selectInput("embed-fonts");

  box.checked = true;
  box.dispatchEvent(new Event("change", { bubbles: true }));
}

/**
 * Chooses files in the font picker and waits for the page to say what it read.
 *
 * @param files - the files chosen, each its bytes and its name
 * @param message - the line the page says once they are read
 * @returns once the page says it
 */
async function importFiles(
  files: readonly { readonly bytes: Uint8Array; readonly name: string }[],
  message: string,
): Promise<void> {
  chooseFiles(
    "font-file",
    files.map(({ bytes, name }) => new File([new Uint8Array(bytes)], name)),
  );

  await vi.waitFor(() => {
    expect(readText("#font-message")).toBe(message);
  });
}

/**
 * Imports Arial, as the steps ask, and waits for it to be read.
 *
 * @returns once it is read
 */
async function importArial(): Promise<void> {
  await importFiles(
    [{ bytes: ARIAL ?? new Uint8Array(), name: "Arial.ttf" }],
    "Arial 400 imported.",
  );
}

/**
 * Imports a face of the test font and waits for it to be read.
 *
 * @param face - its family, weight, style and permissions
 * @returns once it is read
 */
async function importTestFace(face: TestFace): Promise<void> {
  const italic = face.isItalic ? " italic" : "";

  await importFiles(
    [{ bytes: testFaceFile(face), name: "TestSans.ttf" }],
    `${face.family} ${String(face.weightClass)}${italic} imported.`,
  );
}

/**
 * Reads the SVG output column.
 *
 * @returns the drawing's code
 */
function readOutput(): string {
  return readText("#path-data");
}

/**
 * Reads the drawing as Export SVG writes it, with the text the page holds.
 *
 * @returns the file's text
 */
function readExport(): string {
  return readExportedSvg(document, [readTextModel(document)]);
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

/**
 * Reads the positioned lines drawn.
 *
 * @returns each line's text
 */
function readLines(): readonly (string | null)[] {
  return [...document.querySelectorAll("#canvas switch tspan, #canvas > svg > g tspan")].map(
    (line) => line.textContent,
  );
}

/**
 * Reads the path data of the text's underline, the path drawn after its `<text>`.
 *
 * @returns its `d` attribute
 */
function readUnderline(): string {
  return selectPageElement("#canvas g:has(> text) > text ~ path").getAttribute("d") ?? "";
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
 * Makes the browser list the installed fonts, as Chrome does once the permission is granted.
 *
 * @param faces - the faces `queryLocalFonts` lists, each with its family
 * @returns the reads of each face's file
 */
function fakeLocalFonts(faces: readonly TestFace[]): readonly ReturnType<typeof vi.fn>[] {
  const reads = faces.map((face) => {
    const bytes = new Uint8Array(testFaceFile(face));

    return vi.fn<() => Promise<Blob>>().mockResolvedValue(new Blob([bytes]));
  });

  const fonts = faces.map((face, index) => ({
    blob: reads[index],
    family: face.family,
    fullName: face.family,
  }));

  const query = vi.fn<() => Promise<readonly unknown[]>>().mockResolvedValue(fonts);

  Object.defineProperty(globalThis, "queryLocalFonts", { configurable: true, value: query });

  return reads;
}

afterEach(() => {
  Reflect.deleteProperty(globalThis, "queryLocalFonts");
  vi.restoreAllMocks();
});

describe("the guided test of E15 (VAL-017): F05, F14, F16 and F15 in a row", () => {
  it("is the current guided test, F05's steps first and F15's last", () => {
    openServedPage("");

    expect(readText("#guide-title")).toBe("F05 · Inter installed");

    expect(CURRENT_GUIDED_STEPS.map((step) => step.title.slice(0, 3))).toEqual([
      ...Array.from({ length: 36 }, () => "F05"),
      ...Array.from({ length: 36 }, () => "F14"),
      ...Array.from({ length: 42 }, () => "F16"),
      ...Array.from({ length: 37 }, () => "F15"),
    ]);
  });

  it("goes back from F15's first step to F16's last", () => {
    openStep("The font list");
    clickElement("guide-previous");
    expect(readText("#guide-title")).toBe("F16 · Back to the rectangle");
  });
});

describe("the guided test of F15 (VAL-017): fonts, embedding, the export", () => {
  it("[F15.AC1] the font list: Inter alone, a list where no name is typed", () => {
    openStep("The font list");

    expect(selectPageElement("#text-font").tagName).toBe("SELECT");
    expect(readOptions("text-font")).toEqual(["Inter"]);
  });

  it("[F15.AC1] a font imported is chosen at once and said", async () => {
    openStep("Import a font");
    await importTestFace(TEST_SANS);

    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans"]);
    expect(readValue("text-font")).toBe("Test Sans");
  });

  it("[F15.AC1] a file that is not a font is refused, said why, the list unchanged", async () => {
    openStep("A file refused");

    const encoder = new TextEncoder();

    await importFiles(
      [{ bytes: encoder.encode("not a font"), name: "notes.ttf" }],
      "notes.ttf is not a TrueType or OpenType font file (.ttf, .otf, .ttc): refused.",
    );

    expect(readOptions("text-font")).toEqual(["Inter"]);
  });

  it("[F15.AC1] a family's weights: only those imported, 400 and 700", async () => {
    openStep("A family's weights");
    await importTestFace(TEST_SANS);
    await importTestFace({ ...TEST_SANS, weightClass: 700 });

    expect(readOptions("text-weight")).toEqual(["400", "700"]);
  });

  it("[F15.AC1] installed fonts in Chrome: listed, then a family's faces read once chosen", async () => {
    const reads = fakeLocalFonts([TEST_SANS, { ...TEST_SANS, weightClass: 700 }]);

    openStep("Installed fonts (Chrome)");
    clickElement("installed-fonts");
    await waitForPageTasks();

    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans"]);
    expect(reads.map((read) => read.mock.calls.length)).toEqual([0, 0]);

    chooseOption("text-font", "Test Sans");
    expect(readText("#font-message")).toBe("Reading Test Sans…");
    await waitForPageTasks();
    expect(readText("#font-message")).toBe("Test Sans: 2 faces read.");
  });

  it("[F15.AC1] installed fonts elsewhere: said where they can be listed", () => {
    openStep("Installed fonts elsewhere");
    clickElement("installed-fonts");
    expect(readText("#font-message")).toBe(UNSUPPORTED);
  });

  it.skipIf(ARIAL === undefined)(
    "[F15.AC1] measured with the font chosen: Arial 82.3984375 × 18.3984375, Inter 87.5546875 × 19.359375",
    async () => {
      openStep("Measured with the font chosen");
      await importArial();
      expect(JSON.parse(readText("#contour"))).toEqual({ height: 18.3984375, width: 82.3984375 });
      chooseOption("text-font", "Inter");
      expect(JSON.parse(readText("#contour"))).toEqual({ height: 19.359375, width: 87.5546875 });
    },
  );

  it("[F15.AC2] Embed fonts: the face imported embedded, Inter listed as not embedded", async () => {
    openStep("Embed fonts");
    await importTestFace(TEST_SANS);
    tickEmbedFonts();

    // The drawing without its fonts (US-045); the file exported with them.
    expect(readOutput()).toMatch(/^<g /u);

    expect(readExport()).toMatch(
      /^<svg [^>]*><defs><style>@font-face \{ font-family: "Test Sans"/u,
    );

    chooseOption("text-font", "Inter");
    expect(readLeftOut()).toEqual(["Inter 400: not embedded, its file not held."]);
  });

  it("[F15.AC2] Export SVG saves drawing.svg with its style, no switch, without the frame", async () => {
    const saved = vi.fn<(file: Blob) => string>().mockReturnValue("blob:drawing");

    Object.defineProperties(URL, {
      createObjectURL: { configurable: true, value: saved },
      revokeObjectURL: { configurable: true, value: vi.fn() },
    });

    const click = vi.spyOn(HTMLAnchorElement.prototype, "click").mockImplementation(() => {
      // The browser would save the file.
    });

    openStep("Export SVG");
    await importTestFace(TEST_SANS);
    tickEmbedFonts();
    clickElement("export-svg");

    const [[savedFile] = []] = saved.mock.calls;
    const exported = savedFile === undefined ? "" : await savedFile.text();
    const [link] = click.mock.contexts;

    expect(link instanceof HTMLAnchorElement && link.download).toBe("drawing.svg");
    expect(exported).toContain("<style>@font-face");
    expect(exported).not.toMatch(/<switch|<foreignObject/u);
    expect(exported).not.toContain("text-frame");
  });

  it.each(["The export in three browsers", "The export in Inkscape"])(
    "[F15.AC3] %s: the step opens on Pump P-101",
    (title) => {
      openStep(title);
      expect(readLines()).toEqual(["Pump P-101"]);
    },
  );
});

describe("the guided test of F15 (VAL-017): its decisions, each with its example", () => {
  it("[F17.AC2] decision 2: Embed fonts off, the only option, the output a group as before F15", () => {
    openStep("Embed fonts off (decision 2)");

    expect([
      selectInput("embed-fonts").checked,
      document.querySelector("#html-layer") === null,
    ]).toEqual([false, true]);

    expect(readOutput()).toMatch(/^<g /u);
  });

  it("decision 3: an italic face of weight class 950 is named by its family, 900 italic", async () => {
    openStep("A face's family, weight and style (decision 3)");

    await importFiles(
      [{ bytes: testFaceFile({ ...TEST_SANS, isItalic: true, weightClass: 950 }), name: "x.ttf" }],
      "Test Sans 900 italic imported.",
    );

    expect(readValue("text-font")).toBe("Test Sans");
  });

  it("decision 4: the same face imported twice is kept once", async () => {
    openStep("A face read again replaces it (decision 4)");
    await importTestFace(TEST_SANS);
    await importTestFace(TEST_SANS);

    expect([readOptions("text-font"), readOptions("text-weight")]).toEqual([
      ["Inter", "Test Sans"],
      ["400"],
    ]);
  });

  it("decision 5: a reload forgets a font imported", async () => {
    openStep("Fonts kept for the session (decision 5)");
    await importTestFace(TEST_SANS);
    openStep("Fonts kept for the session (decision 5)");
    expect(readOptions("text-font")).toEqual(["Inter"]);
  });

  it.each([
    { fsType: 8, leftOut: [] },
    { fsType: 0, leftOut: [] },
    {
      fsType: 2,
      leftOut: ["Test Sans 400: not embedded, its licence (fsType) does not permit it."],
    },
  ])("decision 6: fsType $fsType embedded unless restricted", async ({ fsType, leftOut }) => {
    openStep("Which faces may be embedded (decision 6)");
    await importTestFace({ ...TEST_SANS, fsType });
    tickEmbedFonts();
    expect(readLeftOut()).toEqual(leftOut);
  });

  it("decision 7: Inter, its file not held, listed as not embedded", () => {
    openStep("A face without its file (decision 7)");
    tickEmbedFonts();
    expect(readLeftOut()).toEqual(["Inter 400: not embedded, its file not held."]);
  });

  it("decision 8: one rule for the face drawn, 700 then 400, the whole file, no local()", async () => {
    openStep("Each face once, whole (decision 8)");
    await importTestFace(TEST_SANS);
    await importTestFace({ ...TEST_SANS, weightClass: 700 });
    tickEmbedFonts();
    clickElement("text-bold");
    expect(readExport().match(/@font-face/gu)).toHaveLength(1);
    expect(readExport()).toContain("font-weight: 700");
    clickElement("text-bold");
    expect(readExport()).toContain("font-weight: 400");
    expect(readExport()).not.toContain("local(");
  });

  it("decision 10: a Hebrew line right to left", () => {
    openStep("Right to left in resvg and Batik (decision 10)");
    expect(readValue("text-direction")).toBe("rtl");
    expect(readLines()).toEqual(["משאבה"]);
  });

  it.each([
    "A family's name read in Unicode (decision 11)",
    "A file without a family refused (decision 12)",
    "Several permission bits (decision 14)",
    "A family whole or not at all (decision 16)",
    "A generic name never embedded (decision 17)",
    "Only a family a text names (decision 18)",
  ])("%s: no file at hand shows it, the step says the tests prove it", (title) => {
    openStep(title);

    expect(readText("#guide-explanation")).toBe(
      "Nothing to try: no file at hand shows it; the tests prove it.",
    );
  });

  it("decision 13: a new family listed after those kept", async () => {
    openStep("First name kept, new family last (decision 13)");
    await importTestFace(TEST_SANS);
    await importTestFace({ ...TEST_SANS, family: "Test Serif" });
    expect(readOptions("text-font")).toEqual(["Inter", "Test Sans", "Test Serif"]);
  });

  it.skipIf(NOTO_JAVANESE === undefined)(
    "decision 15: Noto Sans Javanese, CFF outlines, embedded as font/otf and opentype",
    async () => {
      openStep("An OpenType file with CFF outlines (decision 15)");

      await importFiles(
        [{ bytes: NOTO_JAVANESE ?? new Uint8Array(), name: "NotoSansJavanese-Regular.otf" }],
        "Noto Sans Javanese 400 imported.",
      );

      tickEmbedFonts();
      expect(readExport()).toContain("src: url(data:font/otf;base64,T1RUTw");
      expect(readExport()).toContain('format("opentype")');
    },
  );

  it("decision 24: a list of the families measured, monospace not among them", () => {
    openStep("The font list (decision 24)");
    expect(selectPageElement("#text-font").tagName).toBe("SELECT");
    expect(readOptions("text-font")).not.toContain("monospace");
  });

  it("decision 25: a font imported chosen at once", async () => {
    openStep("A font imported, chosen at once (decision 25)");
    await importTestFace(TEST_SANS);
    expect(readValue("text-font")).toBe("Test Sans");
  });

  it("decision 26: the installed families listed without reading a file", async () => {
    const reads = fakeLocalFonts([TEST_SANS]);

    openStep("Installed families read when chosen (decision 26)");
    clickElement("installed-fonts");
    await waitForPageTasks();
    expect(reads.map((read) => read.mock.calls.length)).toEqual([0]);
    chooseOption("text-font", "Test Sans");
    await waitForPageTasks();
    expect(reads.map((read) => read.mock.calls.length)).toEqual([1]);
  });

  it("decision 27: 300 to a family of 400 alone gives 400; Bold keeps 400", async () => {
    openStep("The weight held (decision 27)");
    expect(readValue("text-weight")).toBe("300");
    await importTestFace(TEST_SANS);
    expect(readValue("text-weight")).toBe("400");
    clickElement("text-bold");
    expect(readValue("text-weight")).toBe("400");
  });

  it("decision 28: one line per face left out, the italic one named", () => {
    openStep("The faces left out, one line each (decision 28)");
    tickEmbedFonts();
    clickElement("text-italic");
    expect(readLeftOut()).toEqual(["Inter 400 italic: not embedded, its file not held."]);
  });

  it("decision 29: the selection's frame drawn on the page, the Export SVG button beside", () => {
    openStep("Export SVG (decision 29)");
    expect(document.querySelector("#canvas .text-frame")).not.toBeNull();
    expect(readText("#export-svg")).toBe("Export SVG");
  });

  it("decision 30: a font imported is measured and drawn under its own family", async () => {
    openStep("A font given to the page (decision 30)");
    await importTestFace(TEST_SANS);

    expect(selectPageElement("#canvas text").getAttribute("font-family")).toBe(
      '"Test Sans", sans-serif',
    );
  });

  it("decision 31: while a family is read, the text keeps the family chosen before", async () => {
    fakeLocalFonts([{ ...TEST_SANS, family: "Test Serif" }]);
    openStep("While a family is read (decision 31)");
    await importTestFace(TEST_SANS);
    clickElement("installed-fonts");
    await waitForPageTasks();
    chooseOption("text-font", "Test Serif");

    expect(selectPageElement("#canvas text").getAttribute("font-family")).toBe(
      '"Test Sans", sans-serif',
    );
  });

  it("decision 32: the underline drawn across descenders", () => {
    openStep("The underline across descenders (decision 32)");
    // One rectangle as long as the line, never broken where the p descends.
    expect(readUnderline()).toMatch(/^M[^M]*Z$/u);
  });

  it("back to the rectangle, drawn as before", () => {
    openStep("Back to the rectangle");
    clickElement("tool-rectangle");
    expect(readDisplay("text-options")).toBe("none");
    expect(readOutput()).toBe("M0 0 L120 0 L120 80 L0 80 Z");
  });
});

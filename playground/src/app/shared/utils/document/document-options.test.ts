// @vitest-environment happy-dom

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import {
  chooseFiles,
  chooseOption,
  chooseShapeKind,
  clickElement,
  openPage,
  readDisplay,
  readText,
  selectInput,
  selectPageElement,
  testFaceFile,
  typeAndEnter,
  waitForPageTasks,
} from "../../../../../testing";
import { readTextModel } from "../text";

import { readExportedSvg } from "./read-exported-svg";

import type { TestFace } from "../../../../../testing";

/** The test font named Test Sans, regular, installable. */
const TEST_SANS: TestFace = { family: "Test Sans", fsType: 0, isItalic: false, weightClass: 400 };

/**
 * Ticks or unticks a check box, as the designer would: its `change` event.
 *
 * @param id - the box's id
 * @param isChecked - whether it ends ticked
 */
function tick(id: string, isChecked: boolean): void {
  const box = selectInput(id);

  box.checked = isChecked;
  box.dispatchEvent(new Event("change", { bubbles: true }));
}

/**
 * Imports a face of the test font and waits for the page to say it.
 *
 * @param face - its family, weight, style and permissions
 * @returns once the page says it
 */
async function importFace(face: TestFace): Promise<void> {
  const bytes = new Uint8Array(testFaceFile(face));

  chooseFiles("font-file", [new File([bytes], "TestSans.ttf")]);

  await vi.waitFor(() => {
    expect(readText("#font-message")).toBe(`${face.family} ${String(face.weightClass)} imported.`);
  });
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
 * Reads the drawing as Export SVG writes it, with the text the page holds.
 *
 * @returns the file's text
 */
function readExport(): string {
  return readExportedSvg(document, [readTextModel(document)]);
}

/**
 * Reads the faces the page lists as not embedded.
 *
 * @returns one line per face
 */
function readLeftOut(): readonly (string | null)[] {
  return [...document.querySelectorAll("#font-left-out li")].map((item) => item.textContent);
}

describe("The drawing's options: Embed fonts, no HTML layer", () => {
  beforeEach(() => {
    openPage("");
    chooseShapeKind("text");
    chooseOption("text-weight", "400");
    typeAndEnter("text-size", "16");
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("[F17.AC2] starts with Embed fonts off, no HTML layer option: no <defs>, no switch", () => {
    expect([
      selectInput("embed-fonts").checked,
      document.querySelector("#html-layer") === null,
    ]).toEqual([false, true]);

    expect(document.querySelector("#canvas defs, #canvas switch")).toBeNull();
    expect(readDisplay("font-left-out")).toBe("none");
    expect(readText("#path-data")).toMatch(/^<g /u);
  });

  it("[F15.AC2] lists Inter as not embedded, its file not held, and writes no <style>", () => {
    tick("embed-fonts", true);

    expect(readLeftOut()).toEqual(["Inter 400: not embedded, its file not held."]);
    expect(readDisplay("font-left-out") === "none").toBe(false);
    expect(document.querySelector("#canvas style")).toBeNull();
  });

  it("[F17.AC3] draws a font imported without <style>; the export embeds it in <defs>", async () => {
    await importFace(TEST_SANS);
    tick("embed-fonts", true);

    // On screen: the text alone, no font written, none left out.
    expect(document.querySelector("#canvas defs, #canvas style")).toBeNull();
    expect(readText("#path-data")).toMatch(/^<g /u);
    expect(readLeftOut()).toEqual([]);

    // The file: one <style> in the <defs> opening the <svg>, one @font-face for the face used.
    expect(readExport()).toMatch(
      /^<svg [^>]*><defs><style>@font-face \{ font-family: "Test Sans"; font-style: normal; font-weight: 400; src: url\(data:font\/ttf;base64,AAEAAA/u,
    );

    expect(readExport().match(/<style>/gu)).toHaveLength(1);
    expect(readExport().match(/@font-face/gu)).toHaveLength(1);
  });

  it("[F17.AC3] exports one @font-face for each of 400 and 700, the drawing still without", async () => {
    await importFace(TEST_SANS);
    await importFace({ ...TEST_SANS, weightClass: 700 });
    tick("embed-fonts", true);
    clickElement("text-bold");

    const text = readTextModel(document);
    const exported = readExportedSvg(document, [{ ...text, weight: 400 }, text]);

    expect(document.querySelector("#canvas style")).toBeNull();
    expect(exported.match(/<defs><style>/gu)).toHaveLength(1);

    expect(exported.match(/font-weight: \d+/gu)).toEqual(["font-weight: 400", "font-weight: 700"]);
  });

  it("[F17.AC17] refuses at import a face its licence does not permit to embed, so no export holds it", async () => {
    const bytes = new Uint8Array(testFaceFile({ ...TEST_SANS, fsType: 2 }));

    chooseFiles("font-file", [new File([bytes], "TestSans.ttf")]);
    await waitForPageTasks();
    tick("embed-fonts", true);

    expect(readText("#font-message")).toBe(
      "TestSans.ttf: Test Sans 400 refused, its fsType forbids embedding it in an exported SVG.",
    );

    expect(readExport()).not.toContain("Test Sans");
  });

  it("[F15.AC2] embeds nothing once the option is off again", async () => {
    await importFace(TEST_SANS);
    tick("embed-fonts", true);
    tick("embed-fonts", false);

    expect(document.querySelector("#canvas defs")).toBeNull();
    expect(readDisplay("font-left-out")).toBe("none");
    // Off, the file holds no <style>, the face imported though it is.
    expect(readExport()).not.toContain("<style>");
  });

  it("[F17.AC1] exports a text with every option on: its <style>, no switch, no XHTML", async () => {
    await importFace(TEST_SANS);
    tick("embed-fonts", true);
    typeContent("Pump P-101 is running");
    chooseOption("text-fit", "box");
    typeAndEnter("text-box-width", "50");
    typeAndEnter("text-box-height", "100");
    typeAndEnter("text-maximum-lines", "2");
    tick("text-underline", true);
    tick("text-strike", true);
    tick("text-background", true);

    const exported = readExport();

    expect(exported).toMatch(/^<svg xmlns="http:\/\/www\.w3\.org\/2000\/svg"/u);
    expect(exported).toContain("<style>@font-face");
    // Cut to two lines, the last ending with an ellipsis: three full stops in a face without "…".
    expect(exported).toMatch(/(?:…|\.\.\.)<\/tspan>/u);
    expect(exported).not.toMatch(/<switch|<foreignObject|xhtml|<div|<span/u);
    expect(exported).not.toContain("text-frame");
    expect(document.querySelector("#canvas .text-frame")).not.toBeNull();
  });

  it("[F15.AC2] saves the export as drawing.svg when Export SVG is clicked", async () => {
    const saved = vi.fn<(file: Blob) => string>().mockReturnValue("blob:drawing");

    const revoked = vi.fn<(url: string) => void>();

    Object.defineProperties(URL, {
      createObjectURL: { configurable: true, value: saved },
      revokeObjectURL: { configurable: true, value: revoked },
    });

    const click = vi.spyOn(HTMLAnchorElement.prototype, "click").mockImplementation(() => {
      // The browser would save the file.
    });

    clickElement("export-svg");

    const [[exported] = []] = saved.mock.calls;
    const [link] = click.mock.contexts;

    expect(click).toHaveBeenCalledOnce();
    expect(revoked).toHaveBeenCalledExactlyOnceWith("blob:drawing");
    expect(link instanceof HTMLAnchorElement && link.download).toBe("drawing.svg");
    expect(exported?.type).toBe("image/svg+xml");
    expect(await exported?.text()).toBe(readExport());
  });

  it("[F15.AC2] exports nothing when nothing is drawn", () => {
    selectPageElement("#canvas").replaceChildren();
    expect(readExport()).toBe("");
  });

  it("[F15.AC2] hides the faces left out while the text is refused", () => {
    tick("embed-fonts", true);
    expect(readLeftOut()).toHaveLength(1);
    // A size emptied by a script, which the field never read: the text refused.
    typeAndEnter("text-size", "");
    expect(readLeftOut()).toEqual([]);
    expect(readDisplay("font-left-out")).toBe("none");
  });
});

// @vitest-environment happy-dom

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { testFaceFile } from "../../tests";
import {
  chooseFiles,
  chooseOption,
  clickElement,
  openServedPage,
  readDisplay,
  readText,
  selectInput,
  selectPageElement,
  typeValue,
} from "../user-actions";

import { readExportedSvg } from "./read-exported-svg";

import type { TestFace } from "../../tests";

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
 * Reads the faces the page lists as not embedded.
 *
 * @returns one line per face
 */
function readLeftOut(): readonly (string | null)[] {
  return [...document.querySelectorAll("#font-left-out li")].map((item) => item.textContent);
}

describe("The drawing's options: Embed fonts and HTML layer (US-036)", () => {
  beforeEach(() => {
    openServedPage("");
    clickElement("tool-text");
    chooseOption("text-weight", "400");
    typeValue("text-size", "16");
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("[F15.AC2] starts with both options off: no <defs>, no switch, no face listed", () => {
    expect([selectInput("embed-fonts").checked, selectInput("html-layer").checked]).toEqual([
      false,
      false,
    ]);

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

  it("[F15.AC2] embeds a font imported: one <style> in <defs>, one @font-face for the face used", async () => {
    await importFace(TEST_SANS);
    tick("embed-fonts", true);

    const rules = selectPageElement("#canvas svg > defs > style").textContent;

    expect(document.querySelectorAll("#canvas style")).toHaveLength(1);
    expect(rules.match(/@font-face/gu)).toHaveLength(1);

    expect(rules).toMatch(
      /^@font-face \{ font-family: "Test Sans"; font-style: normal; font-weight: 400; src: url\(data:font\/ttf;base64,AAEAAA/u,
    );

    expect(readLeftOut()).toEqual([]);
    expect(readText("#path-data")).toMatch(/^<defs[^>]*><style>@font-face/u);
  });

  it("[F15.AC2] leaves out a face its licence does not permit, said why", async () => {
    await importFace({ ...TEST_SANS, fsType: 2 });
    tick("embed-fonts", true);

    expect(document.querySelector("#canvas style")).toBeNull();

    expect(readLeftOut()).toEqual([
      "Test Sans 400: not embedded, its licence (fsType) does not permit it.",
    ]);
  });

  it("[F15.AC2] embeds nothing once the option is off again", async () => {
    await importFace(TEST_SANS);
    tick("embed-fonts", true);
    tick("embed-fonts", false);

    expect(document.querySelector("#canvas defs")).toBeNull();
    expect(readDisplay("font-left-out")).toBe("none");
  });

  it("[F15.AC3] writes the text as a switch of its HTML layer and its positioned lines", () => {
    tick("html-layer", true);

    const foreign = selectPageElement("#canvas switch > foreignObject");

    expect(foreign.getAttribute("requiredExtensions")).toBe("http://www.w3.org/1999/xhtml");
    expect(document.querySelector("#canvas switch > g > text")).not.toBeNull();
    expect(readText("#path-data")).toContain("<switch>");
    tick("html-layer", false);
    expect(document.querySelector("#canvas switch")).toBeNull();
  });

  it("[F15.AC3] gives an Arabic text to the browser in its HTML layer, which joins its letters", () => {
    tick("html-layer", true);
    typeContent("مضخة");

    expect(selectPageElement("#canvas foreignObject span").textContent).toBe("مضخة");
  });

  it("[F15.AC2] exports the drawing with its <style> and its switch, without the frame", async () => {
    await importFace(TEST_SANS);
    tick("embed-fonts", true);
    tick("html-layer", true);

    const exported = readExportedSvg(document);

    expect(exported).toMatch(/^<svg xmlns="http:\/\/www\.w3\.org\/2000\/svg"/u);
    expect(exported).toContain("<style>@font-face");
    expect(exported).toContain("<switch>");
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
    expect(await exported?.text()).toBe(readExportedSvg(document));
  });

  it("[F15.AC2] exports nothing when nothing is drawn", () => {
    selectPageElement("#canvas").replaceChildren();
    expect(readExportedSvg(document)).toBe("");
  });

  it("[F15.AC2] hides the faces left out while the text is refused", () => {
    tick("embed-fonts", true);
    expect(readLeftOut()).toHaveLength(1);
    typeValue("text-size", "0");
    expect(readLeftOut()).toEqual([]);
    expect(readDisplay("font-left-out")).toBe("none");
  });
});

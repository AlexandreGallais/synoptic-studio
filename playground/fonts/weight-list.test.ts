// @vitest-environment happy-dom

import { beforeEach, describe, expect, it, vi } from "vitest";

import { testFaceFile } from "../../tests";
import {
  chooseFiles,
  chooseOption,
  chooseShapeKind,
  clickElement,
  openServedPage,
  readPressedState,
  readText,
  readValue,
  selectChoice,
  selectPageElement,
} from "../user-actions";

import type { TestFace } from "../../tests";

/** The test font named Test Sans, regular, installable. */
const TEST_SANS: TestFace = { family: "Test Sans", fsType: 0, isItalic: false, weightClass: 400 };

/**
 * Imports faces of the test font and waits for the page to say it.
 *
 * @param faces - the faces imported, one file each
 * @param message - the line the page says once they are read
 * @returns once the page says it
 */
async function importFaces(faces: readonly TestFace[], message: string): Promise<void> {
  chooseFiles(
    "font-file",
    faces.map(
      (face) => new File([new Uint8Array(testFaceFile(face))], `${String(face.weightClass)}.ttf`),
    ),
  );

  await vi.waitFor(() => {
    expect(readText("#font-message")).toBe(message);
  });
}

/**
 * Reads the weights the weight list offers.
 *
 * @returns their values, in order
 */
function readWeights(): readonly string[] {
  return [...selectChoice("text-weight").options].map((option) => option.value);
}

describe("The weight list of the family chosen", () => {
  beforeEach(() => {
    openServedPage("");
    chooseShapeKind("text");
    chooseOption("text-weight", "400");
  });

  it("[F15.AC1] offers Inter's nine weights, 100 to 900, named", () => {
    expect(readWeights()).toEqual(["100", "200", "300", "400", "500", "600", "700", "800", "900"]);
    expect(selectChoice("text-weight").options[6]?.textContent).toBe("700 Bold");
  });

  it("[F15.AC1] offers only the weights of a family imported with 400 and 700", async () => {
    await importFaces(
      [TEST_SANS, { ...TEST_SANS, weightClass: 700 }],
      "Test Sans 400 imported. Test Sans 700 imported.",
    );

    expect(readValue("text-font")).toBe("Test Sans");
    expect(readWeights()).toEqual(["400", "700"]);
  });

  it("[F15.AC1] names a weight without a common name by its number alone", async () => {
    await importFaces([{ ...TEST_SANS, weightClass: 450 }], "Test Sans 450 imported.");
    expect(readWeights()).toEqual(["450"]);
    expect(selectChoice("text-weight").options[0]?.textContent).toBe("450");
  });

  it("[F15.AC1] keeps 700 from the family of 400 and 700 to Inter, which has it", async () => {
    await importFaces(
      [TEST_SANS, { ...TEST_SANS, weightClass: 700 }],
      "Test Sans 400 imported. Test Sans 700 imported.",
    );

    chooseOption("text-weight", "700");
    chooseOption("text-font", "Inter");

    expect(readValue("text-weight")).toBe("700");
    expect(selectPageElement("#canvas text").getAttribute("font-weight")).toBe("700");
  });

  it.each([
    // Under 400: lighter ones first, none; then heavier ones ascending: 400.
    { asked: "300", chosen: "400" },
    // 400 to 500: heavier up to 500, none; lighter descending: 400.
    { asked: "500", chosen: "400" },
    // Over 500: heavier ascending: 700.
    { asked: "600", chosen: "700" },
    { asked: "900", chosen: "700" },
  ])(
    "[F15.AC1] chooses the nearest face, $chosen, for $asked in Inter, as CSS does",
    async ({ asked, chosen }) => {
      chooseOption("text-weight", asked);

      await importFaces(
        [TEST_SANS, { ...TEST_SANS, weightClass: 700 }],
        "Test Sans 400 imported. Test Sans 700 imported.",
      );

      expect(readValue("text-weight")).toBe(chosen);
      expect(selectPageElement("#canvas text").getAttribute("font-weight")).toBe(chosen);
    },
  );

  it("[F15.AC1] presses Bold within the family's faces: 700, then back to 400", async () => {
    await importFaces(
      [TEST_SANS, { ...TEST_SANS, weightClass: 700 }],
      "Test Sans 400 imported. Test Sans 700 imported.",
    );

    clickElement("text-bold");
    expect([readValue("text-weight"), readPressedState("text-bold")]).toEqual(["700", "true"]);
    clickElement("text-bold");
    expect([readValue("text-weight"), readPressedState("text-bold")]).toEqual(["400", "false"]);
  });

  it("[F15.AC1] presses Bold on a family of 300 and 600: its nearest face, 600, then 300 given back", async () => {
    await importFaces(
      [
        { ...TEST_SANS, weightClass: 300 },
        { ...TEST_SANS, weightClass: 600 },
      ],
      "Test Sans 300 imported. Test Sans 600 imported.",
    );

    // Inter's 400 matches 300 (the lighter faces under 500 first), 700 matches 600.
    expect(readValue("text-weight")).toBe("300");
    clickElement("text-bold");
    expect([readValue("text-weight"), readPressedState("text-bold")]).toEqual(["600", "true"]);
    clickElement("text-bold");
    expect([readValue("text-weight"), readPressedState("text-bold")]).toEqual(["300", "false"]);
  });

  it("[F05.AC2] forgets the weight held when a family chosen rewrites the weight", async () => {
    chooseOption("text-weight", "100");
    clickElement("text-bold");
    expect([readValue("text-weight"), readPressedState("text-bold")]).toEqual(["700", "true"]);
    await importFaces([TEST_SANS], "Test Sans 400 imported.");
    expect([readValue("text-weight"), readPressedState("text-bold")]).toEqual(["400", "false"]);
    expect(Object.hasOwn(selectPageElement("#text-bold").dataset, "heldWeight")).toBe(false);
  });

  it("[F15.AC1] keeps 400 when Bold is pressed on a family of 400 alone, its nearest face", async () => {
    await importFaces([TEST_SANS], "Test Sans 400 imported.");
    clickElement("text-bold");
    expect([readValue("text-weight"), readPressedState("text-bold")]).toEqual(["400", "false"]);
    expect(readText("#status")).toBe("");
  });

  it("[F15.AC1] keeps the lists' options when their values do not change", () => {
    const [font] = selectChoice("text-font").options;
    const [weight] = selectChoice("text-weight").options;

    chooseOption("text-weight", "500");
    expect(selectChoice("text-font").options[0]).toBe(font);
    expect(selectChoice("text-weight").options[0]).toBe(weight);
  });

  it("[F15.AC1] lists the weights from the lightest, whatever the order of import (M19)", async () => {
    await importFaces(
      [{ ...TEST_SANS, weightClass: 700 }, TEST_SANS],
      "Test Sans 700 imported. Test Sans 400 imported.",
    );

    expect(readWeights()).toEqual(["400", "700"]);
  });

  it("[F15.AC1] matches the italic faces first: 700 asked in italic gives 400 italic (M21)", async () => {
    await importFaces(
      [TEST_SANS, { ...TEST_SANS, weightClass: 700 }, { ...TEST_SANS, isItalic: true }],
      "Test Sans 400 imported. Test Sans 700 imported. Test Sans 400 italic imported.",
    );

    clickElement("text-italic");
    chooseOption("text-weight", "700");

    expect(readValue("text-weight")).toBe("400");
  });
});

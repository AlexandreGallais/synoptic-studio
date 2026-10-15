import { fc, test } from "@fast-check/vitest";
import { describe, expect, it } from "vitest";

import { isSameFamilyName } from "../font";

import { distinctFamilyNames } from "./distinct-family-names";

describe("distinctFamilyNames (REF-CSS-FONTS-4, §5.1)", () => {
  it("[F17.AC5] keeps each family once, its first name, in order", () => {
    // "arial" and "ARIAL" fold to "arial", Arial's name: left out after it.
    expect(distinctFamilyNames(["Inter", "Arial", "arial", "Georgia", "ARIAL", "inter"])).toEqual([
      "Inter",
      "Arial",
      "Georgia",
    ]);
  });

  it("folds ß as ss, as isSameFamilyName does", () => {
    expect(distinctFamilyNames(["Straße", "STRASSE"])).toEqual(["Straße"]);
  });

  it("keeps an empty list empty", () => {
    expect(distinctFamilyNames([])).toEqual([]);
  });

  it("[F17.AC5] lists 590 installed faces of 186 families once each", () => {
    const families = Array.from({ length: 186 }, (unused, index) => `Family ${String(index)}`);
    const faces = Array.from(
      { length: 590 },
      (unused, index) => families[index % families.length] ?? "",
    );

    expect(distinctFamilyNames(faces)).toEqual(families);
  });

  test.prop({ names: fc.array(fc.constantFrom("Arial", "arial", "Inter", "INTER", "Georgia")) })(
    "gives what the comparison of every name with each before it gives",
    ({ names }) => {
      const expected = names.filter(
        (name, index) => names.findIndex((other) => isSameFamilyName(name, other)) === index,
      );

      expect(distinctFamilyNames(names)).toEqual(expected);
    },
  );
});

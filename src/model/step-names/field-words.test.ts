import { describe, expect, it } from "vitest";

import { fieldWords } from "./field-words";

describe("fieldWords (US-077)", () => {
  it.each([
    { field: "width", words: "Width" },
    { field: "x", words: "X" },
    { field: "boxWidth", words: "Box width" },
    { field: "hasStrikeThroughColor", words: "Has strike through color" },
    { field: "", words: "" },
  ])("[F41.AC1] writes $field as '$words'", ({ field, words }) => {
    expect(fieldWords(field)).toBe(words);
  });
});

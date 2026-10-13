// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { chooseOption } from "./choose-option";
import { readValue } from "./read-value";
import { selectChoice } from "./select-choice";
import { selectInput } from "./select-input";
import { selectPageElement } from "./select-page-element";

describe("the elements a test names", () => {
  beforeEach(() => {
    const width = document.createElement("input");
    const shape = document.createElement("select");
    const option = document.createElement("option");
    const note = document.createElement("p");

    width.id = "width";
    width.value = "100";
    shape.id = "shape";
    option.value = "polygon";
    shape.append(option);
    note.id = "note";
    document.body.replaceChildren(width, shape, note);
  });

  it("finds an element, an input, a select, and reads their values", () => {
    expect(selectPageElement("#note").tagName).toBe("P");
    expect(selectInput("width").value).toBe("100");
    expect(selectChoice("shape").tagName).toBe("SELECT");
    expect([readValue("width"), readValue("shape")]).toEqual(["100", "polygon"]);
  });

  it("fails the test at once on an element the page lacks, or one of another kind", () => {
    expect(() => selectPageElement("#missing")).toThrow("The page has no #missing.");
    expect(() => selectInput("note")).toThrow("#note is not an input.");
    expect(() => selectChoice("width")).toThrow("#width is not a select.");
    expect(() => readValue("note")).toThrow("#note has no value.");
  });

  it("fails on an option the select lacks, instead of going on with none chosen", () => {
    expect(() => {
      chooseOption("shape", "ellipse");
    }).toThrow("#shape has no option ellipse.");
  });
});

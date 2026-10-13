// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from "vitest";

import { selectInput } from "./select-input";
import { selectPageElement } from "./select-page-element";

describe("selectPageElement and selectInput", () => {
  beforeEach(() => {
    const width = document.createElement("input");
    const shape = document.createElement("select");
    const note = document.createElement("p");

    width.id = "width";
    width.value = "100";
    shape.id = "shape";
    note.id = "note";
    document.body.replaceChildren(width, shape, note);
  });

  it("finds an element, an input and a select of the page", () => {
    expect(selectPageElement("#note").tagName).toBe("P");
    expect(selectInput("width").value).toBe("100");
    expect(selectInput("shape").tagName).toBe("SELECT");
  });

  it("fails the test at once on an element the page lacks, or one that is not an input", () => {
    expect(() => selectPageElement("#missing")).toThrow("The page has no #missing.");
    expect(() => selectInput("note")).toThrow("#note is not an input.");
  });
});

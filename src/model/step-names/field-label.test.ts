import { describe, expect, it } from "vitest";

import { fieldLabel } from "./field-label";

describe("fieldLabel (US-077)", () => {
  it.each([
    // The panel's labels, where the field's words differ.
    { field: "isItalic", label: "Italic" },
    { field: "isStruckThrough", label: "Strike-through" },
    { field: "strikeThroughColor", label: "Strike-through color" },
    { field: "radii", label: "Radius" },
    { field: "stretched", label: "Stretched to its box" },
    { field: "boxWidth", label: "Width" },
    { field: "textCase", label: "Case" },
    { field: "maximumLines", label: "Max lines" },
    // The field's words, the panel's label.
    { field: "width", label: "Width" },
    { field: "letterSpacing", label: "Letter spacing" },
    { field: "radius", label: "Radius" },
    { field: "points", label: "Points" },
  ])("[F41.AC1] names $field '$label'", ({ field, label }) => {
    expect(fieldLabel(field)).toBe(label);
  });
});

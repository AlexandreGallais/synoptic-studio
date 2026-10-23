import { describe, expect, it } from "vitest";

import { fieldsStepName } from "./fields-step-name";

describe("fieldsStepName (US-077)", () => {
  it.each([
    { fields: ["x"], name: "Move" },
    { fields: ["x", "y"], name: "Move" },
    { fields: ["width"], name: "Width" },
    { fields: ["width", "x"], name: "Width" },
    { fields: ["boxWidth"], name: "Box width" },
    { fields: ["height", "width"], name: "Change" },
    { fields: ["kind"], name: "Convert" },
    { fields: ["corners", "kind", "x"], name: "Convert" },
    { fields: [], name: "Change" },
  ])("[F41.AC1] names a step that changed $fields '$name'", ({ fields, name }) => {
    expect(fieldsStepName(fields)).toBe(name);
  });
});

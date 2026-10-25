import { describe, expect, it } from "vitest";

import { fieldArrowStep } from "./field-arrow-step";

describe("fieldArrowStep", () => {
  it.each([
    // W at 100: Up to 101, Down to 99; Shift + Up to 110, Shift + Down to 90.
    { isFurther: false, key: "ArrowUp", step: 1 },
    { isFurther: false, key: "ArrowDown", step: -1 },
    { isFurther: true, key: "ArrowUp", step: 10 },
    { isFurther: true, key: "ArrowDown", step: -10 },
  ])("[F43.AC3] steps a field by $step on $key, Shift $isFurther", ({ isFurther, key, step }) => {
    expect(fieldArrowStep(key, isFurther)).toBe(step);
  });

  it.each(["ArrowLeft", "ArrowRight", "a", "Enter", "Meta", "Up", ""])(
    "steps nothing on %j, a key other than Up or Down",
    (key) => {
      expect(fieldArrowStep(key, false)).toBeUndefined();
      expect(fieldArrowStep(key, true)).toBeUndefined();
    },
  );
});

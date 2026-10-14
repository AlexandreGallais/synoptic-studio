import { describe, expect, it } from "vitest";

import { nextStep } from "./story-finish";

describe("story-finish", () => {
  it("cleans once merged, waits while open for three minutes, then stops", () => {
    expect(nextStep("MERGED", 1)).toBe("clean");
    expect(nextStep("OPEN", 17)).toBe("wait");
    expect(nextStep("OPEN", 18)).toBe("stop");
    expect(nextStep("CLOSED", 1)).toBe("stop");
  });
});
